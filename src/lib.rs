use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use std::convert::TryInto;
pub mod instruction;
pub mod state;
use borsh::BorshSerialize;
use instruction::GameInstruction;
use state::GameAccountState;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = GameInstruction::unpack(instruction_data)?;

    match instruction {
        GameInstruction::AddGameState {
            username,
            score,
            time,
        } => add_game_state(program_id, accounts, username, score, time),
    }
}

pub fn add_game_state(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    username: String,
    score: u8,
    time: String,
) -> ProgramResult {
    msg!("Adding game state...");
    msg!("Username: {}", username);
    msg!("Score: {}", score);
    msg!("Time: {}", time);

    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[
            initializer.key.as_ref(),
            username.as_bytes().as_ref(),
            time.as_bytes().as_ref(),
        ],
        program_id,
    );

    let account_len: usize = 1 + 1 + (4 + username.len()) + (4 + time.len());

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[
            initializer.key.as_ref(),
            username.as_bytes().as_ref(),
            time.as_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;

    msg!("PDA created for storing game data: {}", pda);

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<GameAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    account_data.username = username;
    account_data.score = score;
    account_data.time = time;
    account_data.is_initialized = true;

    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}

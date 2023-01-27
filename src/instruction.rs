use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum GameInstruction {
    AddGameState {
        username: String,
        score: u8,
        time: String,
    },
}

#[derive(BorshDeserialize)]
struct GameStatePayload {
    username: String,
    score: u8,
    time: String,
}

impl GameInstruction {
    // Unpack inbound buffer to associated Instruction
    // The expected format for input is a Borsh serialized vector
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // Split the first byte of data
        let (&operation, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        // `try_from_slice` is one of the implementations from the BorshDeserialization trait
        // Deserializes instruction byte data into the payload struct
        let payload = GameStatePayload::try_from_slice(rest).unwrap();
        // Match the first byte and return the AddMovieReview struct
        Ok(match operation {
            0 => Self::AddGameState {
                username: payload.username,
                score: payload.score,
                time: payload.time,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

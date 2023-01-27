use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct GameAccountState {
    pub is_initialized: bool,
    pub score: u8,
    pub username: String,
    pub time: String,
}

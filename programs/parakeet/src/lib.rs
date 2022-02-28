use anchor_lang::prelude::*;

declare_id!("BCvJA6zrbWKCzYSPuStDdbvjBBBTDh8z45QGS9MCio11");

#[program]
pub mod parakeet {
    use super::*;
    pub fn sendKeet(ctx: Context<SendKeet>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendKeet<'info> {
    #[account(init, payer = keeter, space = Keet::LEN)]
    pub keet: Account<'info, Keet>,
    pub keeter: Signer<'info>,
    pub system_program: AccountInfo<'info>,
}

// Defines structure of our 'Keet' accounts
#[account]
pub struct Keet {
    pub keeter: Pubkey,
    pub timestamp: i64,
    pub song: String,
    pub lyrics: String,
}

// Useful constants for sizing properties
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX usize = 4; // Stores the size of the string
const MAX_SONG_LENGTH: usize = 50 * 4; // 50 characters max
const MAX_LYRICS_LENGTH: usize = 280 * 4; // 280 characters max

// Add a constant to Keet that provides total size
impl Keet {
    const LEN: usize = DISCRIMINATOR_LENGTH
    + PUBLIC_KEY_LENGTH // Keeter
    + TIMESTAMP_LENGTH // Timestamp
    + STRING_LENGTH_PREFIX + MAX_SONG_LENGTH // Song
    + STRING_LENGTH_PREFIX + MAX_LYRICS_LENGTH; // Lyrics
}
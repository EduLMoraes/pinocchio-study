use pinocchio::pubkey::find_program_address;

use super::*;

pub const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: Pubkey = pinocchio_pubkey::pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
pub const SPL_TOKEN_PROGRAM_ID: Pubkey = pinocchio_pubkey::pubkey!("AQoKYV7tYpTrFZN6P5oUufbQKAUr9mNYGe1TTJC9wajM");
    
pub const SYS_PROGRAM: [u8; 32] = pinocchio_pubkey::from_str("11111111111111111111111111111111");

pub struct AtaCreator;

impl AtaCreator {

    pub fn derive_ata(wallet: &Pubkey, mint: &Pubkey) -> Result<Pubkey, String> {
        let seeds = &[
            wallet.as_ref(),
            SPL_TOKEN_PROGRAM_ID.as_ref(),
            mint.as_ref(),
        ];

        let (ata, _bump) =
            find_program_address(seeds, &SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID);

        Ok(ata)
    }
}

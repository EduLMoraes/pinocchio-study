use std::env;

use create_ata::AtaCreator;
use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    pubkey::Pubkey,
    ProgramResult,
};

mod create_ata;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let acc_meta: Vec<AccountMeta> = accounts
        .iter()
        .map(|acc| AccountMeta::new(acc.key(), acc.is_writable(), acc.is_signer()))
        .collect();


    let accs: Vec<&AccountInfo> = accounts.iter().map(|acc| acc).collect();

    let wallet = env::var("WALLET").expect("error to find wallet env");
    let wmp = pinocchio_pubkey::from_str(&wallet);
    let ata = AtaCreator::derive_ata(&wmp, &wmp).expect("error to derive ata");
    let ix = Instruction {
        program_id: &create_ata::SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
        accounts: &vec![
            AccountMeta::new(&wmp, true, true),
            AccountMeta::new(&ata, false, false),
            AccountMeta::readonly(&wmp),
            AccountMeta::readonly(&wmp),
            AccountMeta::readonly(&create_ata::SYS_PROGRAM),
            AccountMeta::readonly(&create_ata::SPL_TOKEN_PROGRAM_ID),
        ],
        data: &[],
    };
    
    

    invoke(&ix, &[accs.iter().next().unwrap()])?;

    while let Some(account) = accounts.iter().next() {
        msg!("Conta encontrada...");
    }

    msg!("Hello from my program!");
    Ok(())
}

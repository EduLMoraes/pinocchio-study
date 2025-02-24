use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    pubkey::Pubkey,
    ProgramResult,
};

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

    let sys_program = pinocchio_pubkey::from_str("11111111111111111111111111111111");

    let ix = Instruction {
        program_id: &sys_program,
        data: instruction_data,
        accounts: &acc_meta,
    };

    invoke(&ix, &[accs.iter().next().unwrap()])?;

    while let Some(account) = accounts.iter().next() {
        msg!("Conta encontrada...");
    }

    msg!("Hello from my program!");
    Ok(())
}

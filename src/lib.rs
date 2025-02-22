use pinocchio::{
    account_info::AccountInfo,
    default_panic_handler,
    msg,
    no_allocator,
    program_entrypoint,
    ProgramResult,
    pubkey::Pubkey
};

program_entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello from `no_std` program!");
    Ok(())
}
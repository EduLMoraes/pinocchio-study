use borsh::{BorshDeserialize, BorshSerialize};
use create_ata::AtaCreator;
use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use state::counter::{CounterAccount, CounterInstruction};

mod create_ata;
mod state;
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Unpack instruction data
    let instruction = CounterInstruction::unpack(instruction_data)?;

    // Match instruction type
    match instruction {
        CounterInstruction::InitializeCounter { initial_value } => {
            process_initialize_counter(program_id, accounts, initial_value)?
        }
        CounterInstruction::IncrementCounter => process_increment_counter(program_id, accounts)?,
    };

    Ok(())
}

fn process_initialize_counter(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    initial_value: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let counter_account = accounts_iter.next().expect("Not found next account");
    let mint_account = accounts_iter.next().expect("Not found next account");
    let wallet_account = accounts_iter.next().expect("Not found next account");
    let payer_account = accounts_iter.next().expect("Not found next account");
    let system_program = accounts_iter.next().expect("Not found next account");

    // Size of our counter account
    let account_space = 8; // Size in bytes to store a u64

    // Calculate minimum balance for rent exemption
    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(account_space);

    let ata = AtaCreator::derive_ata(wallet_account.key(), mint_account.key())
        .expect("Failed to derive ata");

    let meta_accounts = &[
        AccountMeta::new(counter_account.key(), true, false),
        AccountMeta::new(wallet_account.key(), true, false),
        AccountMeta::new(payer_account.key(), true, false),
        AccountMeta::new(system_program.key(), false, false),
    ];

    let ix = Instruction {
        program_id: system_program.key(),
        accounts: meta_accounts,
        data: &[],
    };

    // Create the counter account
    invoke(
        &ix,
        &[
            counter_account,
            wallet_account,
            payer_account,
            system_program,
        ],
    )?;

    // Create a new CounterAccount struct with the initial value
    let counter_data = CounterAccount {
        count: initial_value,
    };

    // Get a mutable reference to the counter account's data
    let mut account_data = &mut *counter_account
        .try_borrow_mut_data()
        .expect("Failed to borrorow counter data");

    // Serialize the CounterAccount struct into the account's data
    counter_data.serialize(&mut account_data);

    msg!("Counter initialized with value: {}", initial_value);

    Ok(())
}

// Update an existing counter's value
fn process_increment_counter(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let counter_account = accounts_iter.next().expect("Not found accounts");

    // Verify account ownership
    if counter_account.owner() != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Mutable borrow the account data
    let mut data = counter_account
        .try_borrow_mut_data()
        .expect("Failed to borrow mut data");

    // Deserialize the account data into our CounterAccount struct
    let mut counter_data: CounterAccount =
        CounterAccount::try_from_slice(&data).expect("Failed to deserialize");

    // Increment the counter value
    counter_data.count = counter_data
        .count
        .checked_add(1)
        .ok_or(ProgramError::InvalidAccountData)?;

    // Serialize the updated counter data back into the account
    counter_data
        .serialize(&mut &mut data[..])
        .expect("Failed to serialize data");

    msg!("Counter incremented to: {}", counter_data.count);
    Ok(())
}

#[cfg(test)]
mod test {
    use mollusk_svm::Mollusk;
    use solana_program_test::*;
    use solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        system_program,
        transaction::Transaction,
    };

    #[tokio::test]
    async fn test_counter_program() {
        let program_id = Pubkey::new_unique();
        let key1 = Pubkey::new_unique();
        let key2 = Pubkey::new_unique();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[],
            vec![
                AccountMeta::new(key1, false),
                AccountMeta::new_readonly(key2, false),
            ],
        );

        let accounts = vec![(key1, Account::default()), (key2, Account::default())];

        let mollusk = Mollusk::new(&program_id, "pinocchio_study");

        // Execute the instruction and get the result.
        let result = mollusk.process_instruction(&instruction, &accounts);
    }
}

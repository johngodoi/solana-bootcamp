use borsh::{BorshDeserialize, BorshSerialize};
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_sdk::address_lookup_table::{instruction, program};

use crate::instructions::CounterInstruction;

pub mod instructions;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CounterAccount {
    pub counter: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Counter program entrypoint");

    let instructions: CounterInstructions = CounterInstructions::unpack(instruction_data)?;
    
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter);

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        CounterInstructions::Increment => {
            counter_account.counter += 1;
        }
        CounterInstructions::Decrement => {
            counter_account.counter -= 1;
        }
        CounterInstructions::Reset => {
            counter_account.counter = 0;
        }
        CounterInstructions::Update(args) => {
            counter_account.counter = args.value;
        }
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}
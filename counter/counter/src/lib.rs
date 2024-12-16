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

use crate::instructions::CounterInstruction;

pub mod instructions;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CounterAccount {
    pub count: u32,
}
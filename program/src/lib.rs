mod claim;
mod close;
mod initialize;
mod mine;
mod open;
mod reset;
mod stake;
mod update;

use claim::*;
use close::*;
use initialize::*;
use mine::*;
use open::*;
use reset::*;
use stake::*;
use update::*;

use gemm_api::instruction::*;
use solana_program::{
    self, account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub(crate) use gemm_utils as utils;

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&gemm_api::id()) {
        println!("Program ID mismatch");
        return Err(ProgramError::IncorrectProgramId);
    }

    let (tag, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    println!("Validated instruction data");

    match GemInstruction::try_from(*tag).or(Err(ProgramError::InvalidInstructionData))? {
        GemInstruction::Claim => process_claim(accounts, data)?,
        GemInstruction::Close => process_close(accounts, data)?,
        GemInstruction::Mine => process_mine(accounts, data)?,
        GemInstruction::Open => process_open(accounts, data)?,
        GemInstruction::Reset => process_reset(accounts, data)?,
        GemInstruction::Stake => process_stake(accounts, data)?,
        GemInstruction::Update => process_update(accounts, data)?,
        GemInstruction::Initialize => process_initialize(accounts, data)?,
    }

    Ok(())
}

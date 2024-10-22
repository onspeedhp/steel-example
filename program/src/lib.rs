mod add;
mod initialize;
mod transfer_sol;

use add::*;
use initialize::*;
use transfer_sol::*;

use my_project_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&my_project_api::ID, program_id, data)?;

    match ix {
        MyProjectInstruction::Initialize => process_initialize(accounts, data)?,
        MyProjectInstruction::Add => process_add(accounts, data)?,
        MyProjectInstruction::TransferSol => process_transfer_sol(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);

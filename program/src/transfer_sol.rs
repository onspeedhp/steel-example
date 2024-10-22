use my_project_api::prelude::*;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, system_instruction,
};
use steel::*;

pub fn process_transfer_sol(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = TransferSol::try_from_bytes(data)?;
    let amount: u64 = u64::from_le_bytes(args.amount);

    // Load accounts.
    let [signer_info, receiver_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;

    invoke(
        &system_instruction::transfer(signer_info.key, receiver_info.key, amount),
        &[
            signer_info.clone(),
            receiver_info.clone(),
            system_program.clone(),
        ],
    )?;

    Ok(())
}

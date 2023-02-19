//importing required methods and structs
use borsh::BorshDeserialize;
use lever::SetPowerStatus;
use solana_program::{
    account_info::{
        next_account_info, AccountInfo
    },
    entrypoint, 
    entrypoint::ProgramResult, 
    instruction::{ AccountMeta, Instruction },
    pubkey::Pubkey,
    program::invoke,
};

<<<<<<< HEAD
// sets pull_lever as the entrypoint of the program
=======
// sets pull_lever entrypoint of the program
>>>>>>> 835ae0cff22525cbb4d8b41e45451bf8a04d5899
entrypoint!(pull_lever);


fn pull_lever(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    //iters over the given accounts
    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;
    let lever_program = next_account_info(accounts_iter)?;

    //deserializes the instruction with borsh
    let set_power_status_instruction = SetPowerStatus::try_from_slice(instruction_data)?;

    // Creates new instruction with borsh
    let ix = Instruction::new_with_borsh(
        lever_program.key.clone(),                         
        &set_power_status_instruction,                      
        vec![AccountMeta::new(power.key.clone(), false)],   // creates a read only vector 
    );

    invoke(&ix, &[power.clone()]) // invokes the lever_program from power program
}
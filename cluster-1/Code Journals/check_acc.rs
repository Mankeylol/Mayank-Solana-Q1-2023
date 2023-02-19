//imported required methods structs
use solana_program::{
    account_info::{ AccountInfo, next_account_info }, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};

<<<<<<< HEAD
//sets process_instruction to be the entrypoint of the program
=======
//sets process instruction as the entrypoint of the program
>>>>>>> 835ae0cff22525cbb4d8b41e45451bf8a04d5899
entrypoint!(process_instruction);


fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {

//checks if the program ID in instruction is the same as the system program
    if system_program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId) // throws an error if it's not the same
    };
    
//checks if there's all the required accs
    if accounts.len() < 4 {
        msg!("This instruction requires 4 accounts:");//meesage that shows up if there's less than 4 accs
        msg!("  payer, account_to_create, account_to_change, system_program");
        return Err(ProgramError::NotEnoughAccountKeys) //error thrown if there's not enough accs
    };

<<<<<<< HEAD
=======
    //iterates over the given accouncs and sets all the variables 
>>>>>>> 835ae0cff22525cbb4d8b41e45451bf8a04d5899
    let accounts_iter = &mut accounts.iter();
    let _payer = next_account_info(accounts_iter)?;
    let account_to_create = next_account_info(accounts_iter)?;
    let account_to_change = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    //checks if the account already exists
    msg!("New account: {}", account_to_create.key);
    if account_to_create.lamports() != 0 {
        msg!("The program expected the account to create to not yet be initialized.");
        return Err(ProgramError::AccountAlreadyInitialized)
    };
   
   //the account making the tx has no gas fees
    msg!("Account to change: {}", account_to_change.key);
    if account_to_change.lamports() == 0 {
        msg!("The program expected the account to change to be initialized.");
        return Err(ProgramError::UninitializedAccount)
    };

    //checks if the the given program ID is the owner of the acc to change
    if account_to_change.owner != program_id {
        msg!("Account to change does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId)
    };

    if system_program.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId)
    };

    Ok(())
}


//the key concepts used are borrowing and macros
// the contract checks the instruction passed
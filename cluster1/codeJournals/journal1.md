### from - solana-developers / program-examples

</br>

> checking accounts - this program does various checks to the accounts passed into the process instruction by verifying the following:

1. is provided program id valid - if not returns `IncorrectProgramId` error
2. the number of accounts passed are at least 4 - if not returns `NotEnoughAccountKeys` error [_here instead of just checking if len if less than 4, check if len is not equal to 4 which also prevents passing unnecessary extra accounts_]
3. creating the iterator of the list of accounts and resolving them to AccountInfo and unwrapping them using `?` makes sure that the account is passed properly & invoking `next_account_info()` 4 times again makes sure that 4 accounts has been passed [_here if the program has access to account_to_create and/or account_to_change structs unpacking them will make sure they are of expected account type_]
4. checks if account_to_create lamports are 0, this makes sure that account is not already initialized - if already initialized returns `AccountAlreadyInitialized` error
5. checks if account_to_change lamports are not 0, this makes sure that account is already initialized - if not already initialized returns `UninitializedAccount` error
6. checks if account_to_change owner is indeed program itself, this makes sure it is a PDA account - if not return `IncorrectProgramId` error
7. lastly checks if system_program id is valid - if not returns `IncorrectProgramId` error

</br>

```rust
use solana_program::{
    account_info::{ AccountInfo, next_account_info },
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};


entrypoint!(process_instruction);


fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {

    // You can verify the program ID from the instruction is in fact
    //      the program ID of your program.
    if system_program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId)
    };

    // You can verify the list has the correct number of accounts.
    // This error will get thrown by default if you
    //      try to reach past the end of the iter.
    if accounts.len() < 4 {
        msg!("This instruction requires 4 accounts:");
        msg!("  payer, account_to_create, account_to_change, system_program");
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    // Accounts passed in a vector must be in the expected order.
    let accounts_iter = &mut accounts.iter();
    let _payer = next_account_info(accounts_iter)?;
    let account_to_create = next_account_info(accounts_iter)?;
    let account_to_change = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // You can make sure an account has NOT been initialized.

    msg!("New account: {}", account_to_create.key);
    if account_to_create.lamports() != 0 {
        msg!("The program expected the account to create to not yet be initialized.");
        return Err(ProgramError::AccountAlreadyInitialized)
    };
    // (Create account...)

    // You can also make sure an account has been initialized.
    msg!("Account to change: {}", account_to_change.key);
    if account_to_change.lamports() == 0 {
        msg!("The program expected the account to change to be initialized.");
        return Err(ProgramError::UninitializedAccount)
    };

    // If we want to modify an account's data, it must be owned by our program.
    if account_to_change.owner != program_id {
        msg!("Account to change does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId)
    };

    // You can also check pubkeys against constants.
    if system_program.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId)
    };

    Ok(())
}
```

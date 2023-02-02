### from - solana-developers / program-examples

</br>

> create account - this program shows how one can create account

1. creates the account iterator
2. resolves all required accounts from the iterator i.e. payer, new_account to create & system_program
3. system_program is required because we are using the system_instruction to create the account
4. lastly use the invoke ix to do cpi by providing all required accounts, space in bytes(0), & lamports(1 SOL)

</br>

```rust
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    native_token::LAMPORTS_PER_SOL,
    program::invoke,
    pubkey::Pubkey,
    system_instruction,
    system_program,
};


entrypoint!(process_instruction);


fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let new_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    msg!("Program invoked. Creating a system account...");
    msg!("  New public key will be: {}", &new_account.key.to_string());

    invoke(
        &system_instruction::create_account(
            &payer.key,
            &new_account.key,
            1 * LAMPORTS_PER_SOL,
            0,
            &system_program::ID,
        ),
        &[
            payer.clone(), new_account.clone(), system_program.clone()
        ]
    )?;

    msg!("Account created successfully.");
    Ok(())
}
```

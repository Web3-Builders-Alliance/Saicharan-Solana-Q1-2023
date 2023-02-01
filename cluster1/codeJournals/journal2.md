### from - solana-developers / program-examples

</br>

> cross program invocation - this program shows how one can invoke instructions of another on-chain program:

1. resolves the required accounts from accounts iterator i.e. `power` & `lever_program`
2. deserializes the `SetPowerStatus` ix using `try_from_slice()` method which takes instruction_data as argument and return ix
3. then new ix is created using `Instruction::new_with_borsh` which takes 3 arguments `lever_program` key, `SetPowerStatus` ix which we deserialized in 2nd step, & vector of accounts
4. here one account is passed, `AccountMeta` constructor is used to create a writable account and which is not a signer this can be understood from the signature of the AccountMeta constructor
5. lastly then by using invoke method provided by program crate of solana_program library, a cross program invocation is done by providing two arguments to invoke i.e. reference of newly created ix and reference of list of accounts in this case power account

</br>

```rust
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


entrypoint!(pull_lever);


fn pull_lever(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;
    let lever_program = next_account_info(accounts_iter)?;

    let set_power_status_instruction = SetPowerStatus::try_from_slice(instruction_data)?;

    let ix = Instruction::new_with_borsh(
        lever_program.key.clone(),                          // Our lever program's ID
        &set_power_status_instruction,                      // Passing instructions through
        vec![AccountMeta::new(power.key.clone(), false)],   // Just the required account for the other program
    );

    invoke(&ix, &[power.clone()])
}
```

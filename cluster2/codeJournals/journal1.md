### from - solana-developers / program-examples

</br>

> checking accounts (anchor program) - this program does various checks to the accounts passed into the instruction and most of the verification is taken care by `Anchor`

1. importing the `anchor_lang::prelude` crate is important as it brings in all what anchor as to offer you for easy development.
2. `declare_id!` is used to assign the Public Key to the Program. Initially when the anchor project is initiated a placeholder Public Key is used that has to be replaced using the keypair that can be found at `<project_name>/target/deploy/<project-name>.json`, the keypair is generated when `anchor build` command is executed. And Public Key can be derived from the keypair using `anchor keys list`. The Public Key has to be changed in `lib.rs` in `declare_id!` macro and also in `Anchor.toml`
3. Using `#[program]` attribute program module is declared and anchor uses these module to extend the code to native and also add extra layer of security and checks.
4. In the module, all the instruction that a program needs are defined here. Here `check_accounts` instruction is a function which takes `Context` with an type `CheckingAccounts` as an attribute and the function returns the `Result<()>`. As simple as that, this is the complete function thats needed for this program. Rest of the things are done at the Accounts below.
5. `derive()` macro is used with `Accounts` as an argument to make sure the anchor understands that this are the accounts that would be needed in the instruction. Use this struct with the right lifetimes passed to the fields to declare the accounts and each field has its own type that are `AccountInfo`, `UncheckedAccount`, `Account`, `Signer` & `Program` and the `account()` macro which are used to do various account validations. Along with this `account()` macro accepts many arguments like `mut`, `seeds`, `signer`, `space`, `bump` and many more find all of them [here](https://docs.rs/anchor-lang/latest/anchor_lang/derive.Accounts.html).
6. Anchor also provides macros for error (`error_code`) in conjunction with `msg()` to declare program specific errors and `#[account]` attribute to declare state structs that program needs to create PDAs.
7. Anchor also provides lots of helps cli commands which can be found [here](https://www.anchor-lang.com/).

```rust
use anchor_lang::prelude::*;


declare_id!("ECWPhR3rJbaPfyNFgphnjxSEexbTArc7vxD8fnW6tgKw");


#[program]
pub mod anchor_program_example {
    use super::*;

    pub fn check_accounts(_ctx: Context<CheckingAccounts>) -> Result<()> {

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CheckingAccounts<'info> {
    payer: Signer<'info>,
    #[account(mut)]
    /// CHECK: This account's data is empty
    account_to_create: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This account's data is empty
    account_to_change: AccountInfo<'info>,
    system_program: Program<'info, System>,
}
```

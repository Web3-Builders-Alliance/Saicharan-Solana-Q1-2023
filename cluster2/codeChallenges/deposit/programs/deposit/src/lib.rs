use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};
use anchor_spl::{
    associated_token::{self, AssociatedToken, Create},
    metadata::{self, CreateMasterEditionV3, CreateMetadataAccountsV3, SignMetadata},
    token::{self, Mint, MintTo, Token, TokenAccount, Transfer as SplTransfer},
};
use mpl_token_metadata::state::{Creator, DataV2};

declare_id!("CtkbSLVD5Rq3TZ6C9oncbcBzfmJ8YrfdBCePS2J9W7WZ");

#[program]
pub mod deposit {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.deposit_auth = ctx.accounts.deposit_auth.key();
        deposit_account.auth_bump = *ctx.bumps.get("pda_auth").unwrap();
        Ok(())
    }

    pub fn deposit_native(ctx: Context<DepositNative>, amount: u64) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        let deposit_auth = &ctx.accounts.deposit_auth;
        let system_program = &ctx.accounts.system_program;
        deposit_account.sol_vault_bump = ctx.bumps.get("sol_vault").copied();
        let cpi_accounts = Transfer {
            from: deposit_auth.to_account_info(),
            to: ctx.accounts.sol_vault.to_account_info(),
        };
        let cpi = CpiContext::new(system_program.to_account_info(), cpi_accounts);
        system_program::transfer(cpi, amount)?;
        Ok(())
    }

    pub fn withdraw_native(ctx: Context<WithdrawNative>, amount: u64) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        let deposit_auth = &ctx.accounts.deposit_auth;
        let pda_auth = &mut ctx.accounts.pda_auth;
        let sol_vault = &mut ctx.accounts.sol_vault;
        let system_program = &ctx.accounts.system_program;
        let cpi_accounts = Transfer {
            from: sol_vault.to_account_info(),
            to: deposit_auth.to_account_info(),
        };
        let seeds = [
            b"sol_vault",
            pda_auth.to_account_info().key.as_ref(),
            &[deposit_account.sol_vault_bump.unwrap()],
        ];
        let signers = &[&seeds[..]];
        let cpi =
            CpiContext::new_with_signer(system_program.to_account_info(), cpi_accounts, signers);
        system_program::transfer(cpi, amount)?;
        Ok(())
    }

    pub fn deposit_spl(ctx: Context<DepositSPL>, amount: u64) -> Result<()> {
        let deposit_auth = &ctx.accounts.deposit_auth;
        let token_program = &ctx.accounts.token_program;
        let cpi_accounts = SplTransfer {
            from: ctx.accounts.depositor_token_account.to_account_info(),
            to: ctx.accounts.sol_vault.to_account_info(),
            authority: deposit_auth.to_account_info(),
        };
        let cpi = CpiContext::new(token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi, amount)?;
        Ok(())
    }

    pub fn withdraw_spl(ctx: Context<WithdrawSPL>, amount: u64) -> Result<()> {
        let deposit_account = &ctx.accounts.deposit_account;
        let token_program = &ctx.accounts.token_program;
        let cpi_accounts = SplTransfer {
            from: ctx.accounts.sol_vault.to_account_info(),
            to: ctx.accounts.depositor_token_account.to_account_info(),
            authority: ctx.accounts.pda_auth.to_account_info(),
        };
        let seeds = &[
            b"auth",
            deposit_account.to_account_info().key.as_ref(),
            &[deposit_account.auth_bump],
        ];
        let signers = &[&seeds[..]];
        let cpi =
            CpiContext::new_with_signer(token_program.to_account_info(), cpi_accounts, signers);
        token::transfer(cpi, amount)?;
        Ok(())
    }

    pub fn mint_nft_and_stake(
        ctx: Context<MintNft>,
        name: String,
        symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        ctx.accounts.mint(name, symbol, metadata_uri)?;
        let deposit_account = &mut ctx.accounts.deposit_account;
        let token_program = &ctx.accounts.token_program;

        // Create Associated Mint Account
        associated_token::create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            Create {
                mint: ctx.accounts.mint.to_account_info(),
                associated_token: ctx.accounts.vault_token_account.to_account_info(),
                authority: ctx.accounts.pda_auth.to_account_info(),
                payer: ctx.accounts.deposit_auth.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        ))?;

        let cpi_accounts = SplTransfer {
            from: ctx.accounts.token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.deposit_auth.to_account_info(),
        };
        let seeds = &[
            b"auth",
            deposit_account.to_account_info().key.as_ref(),
            &[deposit_account.auth_bump],
        ];
        let signers = &[&seeds[..]];
        let cpi =
            CpiContext::new_with_signer(token_program.to_account_info(), cpi_accounts, signers);
        token::transfer(cpi, 1)?;
        Ok(())
    }

    pub fn create_limit(
        ctx: Context<CreateLimit>,
        ask_asset: Asset,
        ask_price_per_asset: u64,
    ) -> Result<()> {
        // let deposit_account = &mut ctx.accounts.deposit_account;
        let limit_account = &mut ctx.accounts.limit_account;
        limit_account.asset_holding_pda = Some(limit_account.key());
        limit_account.asset = Asset {
            asset_type: "".to_string(),
            asset_mint: Some(ctx.accounts.token_mint.key()),
            asset_metadata: None,
        };
        limit_account.ask_price_per_asset = ask_price_per_asset;
        limit_account.ask_asset = ask_asset;
        // limit_account.ask_asset_pda = todo!();
        Ok(())
    }

    pub fn update_limit(ctx: Context<UpdateLimit>) -> Result<()> {
        Ok(())
    }

    pub fn remove_limit(ctx: Context<RemoveLimit>) -> Result<()> {
        Ok(())
    }

    pub fn accept_limit(ctx: Context<AcceptLimit>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = deposit_auth,
        space = DepositBase::LEN,
    )]
    pub deposit_account: Account<'info, DepositBase>,
    /// CHECK:
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump)]
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositNative<'info> {
    #[account(mut, has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    /// CHECK:
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump)]
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump)]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawNative<'info> {
    #[account(mut, has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    /// CHECK:
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump = deposit_account.sol_vault_bump.unwrap())]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositSPL<'info> {
    #[account(mut, has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    /// CHECK:
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    pub pda_auth: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = deposit_auth,
        associated_token::mint = token_mint,
        associated_token::authority = pda_auth
    )]
    pub sol_vault: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    #[account(mut)]
    pub depositor_token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawSPL<'info> {
    #[account(mut, has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    /// CHECK:
    #[account(
        seeds = [
            b"auth",
            deposit_account.key().as_ref()
        ],
        bump = deposit_account.auth_bump
    )]
    pub pda_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = pda_auth
    )]
    pub sol_vault: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    #[account(mut)]
    pub depositor_token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(has_one = deposit_auth)]
    deposit_account: Account<'info, DepositBase>,
    /// CHECK:
    #[account(
        seeds = [
            b"auth",
            deposit_account.key().as_ref(),
        ],
        bump = deposit_account.auth_bump
    )]
    pub pda_auth: UncheckedAccount<'info>,
    /// CHECK: Metaplex will check this
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: Signer<'info>,
    /// CHECK: Metaplex will check this
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: need to add checks
    #[account(mut)]
    pub vault_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Metaplex will check this
    pub master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> MintNft<'info> {
    pub fn mint(&self, name: String, symbol: String, metadata_uri: String) -> Result<()> {
        let rent = Rent::get()?;

        // Create Token Mint Account
        let lamports = rent.minimum_balance(Mint::LEN);
        system_program::create_account(
            CpiContext::new(
                self.deposit_auth.to_account_info(),
                system_program::CreateAccount {
                    from: self.deposit_auth.to_account_info(),
                    to: self.mint.to_account_info(),
                },
            ),
            lamports,
            Mint::LEN.try_into().unwrap(),
            &self.token_program.key(),
        )?;

        // Initialize Mint
        token::initialize_mint2(
            CpiContext::new(
                self.token_program.to_account_info(),
                token::InitializeMint2 {
                    mint: self.mint.to_account_info(),
                },
            ),
            0,
            &self.deposit_auth.key(),
            Some(&self.deposit_auth.key()),
        )?;

        // Create Associated Mint Account
        associated_token::create(CpiContext::new(
            self.associated_token_program.to_account_info(),
            Create {
                mint: self.mint.to_account_info(),
                associated_token: self.token_account.to_account_info(),
                authority: self.deposit_auth.to_account_info(),
                payer: self.deposit_auth.to_account_info(),
                token_program: self.token_program.to_account_info(),
                system_program: self.system_program.to_account_info(),
            },
        ))?;

        // Mint To
        token::mint_to(
            CpiContext::new(
                self.token_program.to_account_info(),
                MintTo {
                    mint: self.mint.to_account_info(),
                    to: self.token_account.to_account_info(),
                    authority: self.deposit_auth.to_account_info(),
                },
            ),
            1,
        )?;

        // Create Metadata Account
        metadata::create_metadata_accounts_v3(
            CpiContext::new(
                self.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    mint: self.mint.to_account_info(),
                    metadata: self.metadata.to_account_info(),
                    mint_authority: self.deposit_auth.to_account_info(),
                    update_authority: self.deposit_auth.to_account_info(),
                    payer: self.deposit_auth.to_account_info(),
                    rent: self.rent.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                },
            ),
            DataV2 {
                name,
                symbol,
                uri: metadata_uri,
                seller_fee_basis_points: 0,
                creators: Some(vec![Creator {
                    address: self.deposit_auth.key(),
                    share: 100,
                    verified: false,
                }]),
                collection: None,
                uses: None,
            },
            true,
            false,
            None,
        )?;

        metadata::sign_metadata(CpiContext::new(
            self.token_program.to_account_info(),
            SignMetadata {
                creator: self.deposit_auth.to_account_info(),
                metadata: self.metadata.to_account_info(),
            },
        ))?;

        metadata::create_master_edition_v3(
            CpiContext::new(
                self.token_metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    mint: self.mint.to_account_info(),
                    metadata: self.metadata.to_account_info(),
                    edition: self.master_edition.to_account_info(),
                    update_authority: self.deposit_auth.to_account_info(),
                    mint_authority: self.deposit_auth.to_account_info(),
                    payer: self.deposit_auth.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                    rent: self.rent.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                },
            ),
            Some(0),
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateLimit<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    /// CHECK:
    #[account(seeds = [b"auth", deposit_auth.key().as_ref()], bump)]
    pub pda_auth: AccountInfo<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    #[account(
        init,
        payer = deposit_auth,
        seeds = [
            b"limit",
            token_mint.key().as_ref(),
            deposit_account.key().as_ref()],
            bump,
            space = Asset::LEN
        )]
    pub limit_account: Account<'info, Limit>,
    pub token_account: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    #[account(owner = Token::id())]
    pub ask_token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateLimit {}

#[derive(Accounts)]
pub struct RemoveLimit {}

#[derive(Accounts)]
pub struct AcceptLimit {}

#[account]
pub struct DepositBase {
    pub deposit_auth: Pubkey,
    pub auth_bump: u8,
    pub sol_vault_bump: Option<u8>,
}

impl DepositBase {
    const LEN: usize = 8 + 32 + 1 + 1 + 1;
}

const OPTION_PUBKEY_LEN: usize = 1 + 32;

#[account]
pub struct Limit {
    pub asset_holding_pda: Option<Pubkey>,
    pub asset: Asset,
    pub ask_price_per_asset: u64,
    pub ask_asset: Asset,
    pub ask_asset_pda: Option<Pubkey>,
}

impl Limit {
    const LEN: usize = 8 + Asset::LEN * 2 + OPTION_PUBKEY_LEN * 2;
}

#[account]
pub struct Asset {
    pub asset_type: String,
    pub asset_metadata: Option<Pubkey>,
    pub asset_mint: Option<Pubkey>,
}

impl Asset {
    const LEN: usize = 32 + OPTION_PUBKEY_LEN * 2;
}

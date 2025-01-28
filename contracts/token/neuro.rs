use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};

#[program]
pub mod neuro_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Token initialization logic
        let token_mint = &mut ctx.accounts.token_mint;
        let authority = &mut ctx.accounts.authority;
        
        // Initialize token with 9 decimals
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: token_mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            9,
            authority.key,
            Some(authority.key),
        )?;

        Ok(())
    }

    // Additional token functions
    pub fn mint_reward(ctx: Context<MintReward>, amount: u64) -> Result<()> {
        // Reward minting logic
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.token_mint.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 82)]
    pub token_mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintReward<'info> {
    #[account(mut)]
    pub token_mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub user_token_account: Account<'info, token::TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

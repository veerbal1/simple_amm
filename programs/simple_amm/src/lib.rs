use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("FXr2fGpzYhSeUK2EzHxDJoWQcm8gsESeKxQ7MKrdYeEg");

#[program]
pub mod simple_amm {
    use super::*;

    pub fn initialize(ctx: Context<InitializePool>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(init, payer = initializer, seeds = [b"pool", token_a_mint.key().as_ref(), token_b_mint.key().as_ref()], bump, space = 8 + 8)]
    pub pool_state: Account<'info, PoolState>,

    pub token_a_mint: Account<'info, Mint>,
    pub token_b_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = initializer,
        seeds = [b"vault", pool_state.key().as_ref(), token_a_mint.key().as_ref()],
        bump,
        token::mint = token_a_mint,
        token::authority = pool_state,
    )]
    pub token_a_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = initializer,
        seeds = [b"vault", pool_state.key().as_ref(), token_b_mint.key().as_ref()],
        bump,
        token::mint = token_b_mint,
        token::authority = pool_state,
    )]
    pub token_b_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct PoolState {
    pub pool_state_bump: u8,
    pub token_a_vault_bump: u8,
    pub token_b_vault_bump: u8,
    pub fee_rate: u16,
    pub lp_mint: Pubkey,
}

impl PoolState {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}

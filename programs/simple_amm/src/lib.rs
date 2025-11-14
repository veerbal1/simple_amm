use anchor_lang::prelude::*;

declare_id!("FXr2fGpzYhSeUK2EzHxDJoWQcm8gsESeKxQ7MKrdYeEg");

#[program]
pub mod simple_amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

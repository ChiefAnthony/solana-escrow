use anchor_lang::prelude::*;

declare_id!("4HYm6NXVJS3tmzvpGfnq55kpNbGtm4KeD7pRxiShHXWV");

mod instructions;
use instructions::*;

mod state;
use state::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Make {}

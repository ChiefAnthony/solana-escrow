use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey, // the mint being offered
    pub mint_b: Pubkey, // mint being asked in return
    pub receive: u64,
    pub bump: u8,
}

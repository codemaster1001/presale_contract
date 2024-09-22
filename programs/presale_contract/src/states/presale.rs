use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PresaleState {
    pub index: u16,
    pub creator: Pubkey,
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub token_0_amount: u64,
    pub token_1_amount: u64,
    pub presale_price_x32: u64,
    pub bump: u8,
}
use anchor_lang::{
    accounts::interface_account::InterfaceAccount,
    prelude::*,
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Token},
    token_2022::{self, },
    token_interface::{Mint, TokenAccount,  Token2022}
};


use crate::states::*;
use crate::error::ErrorCode;
use crate::utils::*;


#[derive(Accounts)]
pub struct BuyToken0<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mint::token_program = token_0_program
    )]
    pub token_0_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mint::token_program = token_1_program,
    )]
    pub token_1_mint: Box<InterfaceAccount<'info, Mint>>,

    ///Check
    #[account(
        seeds = [
            b"presale", presale_state.token_0_mint.key().as_ref(), presale_state.token_1_mint.key().as_ref()
        ],
        bump = presale_state.bump
    )]
    pub presale_state: Account<'info, PresaleState>,

    /// CHECK: Token_0 vault
    #[account(
        mut,
        constraint = token_0_vault.key() == presale_state.token_0_vault
    )]
    pub token_0_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: Token_1 vault
    #[account(
        mut,
        constraint = token_1_vault.key() == presale_state.token_1_vault
    )]
    pub token_1_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_0_mint,
        associated_token::authority = buyer,
        token::token_program = token_0_program
    )]
    pub buyer_token_0 : Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = presale_state.token_1_mint,
        token::authority = buyer,
    )]
    pub buyer_token_1: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_0_program: Program<'info, Token2022>,
    pub token_1_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn buy_token0(ctx: Context<BuyToken0>, token1_amount: u64)->Result<()> {
    let presale_state = &mut ctx.accounts.presale_state;

    transfer_from_user_to_pool_vault(
        ctx.accounts.buyer.to_account_info(),
        ctx.accounts.buyer_token_1.to_account_info(),
        ctx.accounts.token_1_vault.to_account_info(),
        ctx.accounts.token_1_mint.to_account_info(),
        ctx.accounts.token_1_program.to_account_info(),
        token1_amount,
        ctx.accounts.token_1_mint.decimals,
    )?;

    transfer_from_pool_vault_to_user(
        ctx.accounts.presale_state.to_account_info(),
        ctx.accounts.token_0_vault.to_account_info(),
        ctx.accounts.buyer_token_0.to_account_info(),
        ctx.accounts.token_0_mint.to_account_info(),
        ctx.accounts.token_0_program.to_account_info(),
        output_transfer_amount,
        ctx.accounts.token_0_mint.decimals,
        &[&[b"presale", token_0_mint.key().as_ref(), token_1_mint.key().as_ref(), &[presale_state.bump]]],
    )?;

    Ok(())
}
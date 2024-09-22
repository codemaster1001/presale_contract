use anchor_lang::{
    accounts::interface_account::InterfaceAccount,
    prelude::*,
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Token},
    token_2022::{self},
    token_interface::{Mint, TokenAccount,  Token2022}
};


use crate::states::*;
use crate::error::ErrorCode;
use crate::utils::*;


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mint::token_program = token_0_program
    )]
    pub token_0_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mint::token_program = token_1_program,
    )]
    pub token_1_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init,
        seeds = [
            b"presale", token_0_mint.key().as_ref(), token_1_mint.key().as_ref()
        ],
        payer=creator,
        space= 8 + PresaleState::INIT_SPACE,
        bump
    )]
    pub presale_state: Account<'info, PresaleState>,

    /// CHECK: Token_0 vault
    #[account(
        mut,
        seeds = [
            b"token_0_vault",
            presale_state.key().as_ref(),
            token_0_mint.key().as_ref()
        ],
        bump,
    )]
    pub token_0_vault: UncheckedAccount<'info>,

    /// CHECK: Token_1 vault
    #[account(
        mut,
        seeds = [
            b"token_1_vault",
            presale_state.key().as_ref(),
            token_1_mint.key().as_ref()
        ],
        bump,
    )]
    pub token_1_vault: UncheckedAccount<'info>,

    #[account(
        mut,
        token::mint = token_0_mint,
        token::authority = creator,
    )]
    pub creator_token_0: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_0_program: Program<'info, Token2022>,
    pub token_1_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

pub fn initialize(ctx: Context<Initialize>, index: u16, token_0_amount: u64,presale_price_x32: u64)->Result<()> {
    let token0_mint_info = ctx.accounts.token_0_mint.to_account_info();
    if *token0_mint_info.owner != token_2022::Token2022::id() {
        return Err(ErrorCode::Mint0NotToken2022.into());
    }

    let token1_mint_info = ctx.accounts.token_1_mint.to_account_info();
    if *token1_mint_info.owner != token::Token::id(){
        return Err(ErrorCode::Mint1NotUSDC.into());
    }
    // if *token1_mint_info.owner != token::Token::id() && *token1_mint_info.key != crate::usdc::id() {
    //     return Err(ErrorCode::Mint1NotUSDC.into());
    // }

    create_token_account(
        &ctx.accounts.presale_state.to_account_info(),
        &ctx.accounts.creator.to_account_info(),
        &ctx.accounts.token_0_vault.to_account_info(),
        &ctx.accounts.token_0_mint.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        &ctx.accounts.token_0_program.to_account_info(),
        &[&[
            b"token_0_vault",
            ctx.accounts.presale_state.key().as_ref(),
            ctx.accounts.token_0_mint.key().as_ref(),
            &[ctx.bumps.token_0_vault][..],
        ][..]],
    )?;

    transfer_from_user_to_pool_vault(
        ctx.accounts.creator.to_account_info(),
        ctx.accounts.creator_token_0.to_account_info(),
        ctx.accounts.token_0_vault.to_account_info(),
        ctx.accounts.token_0_mint.to_account_info(),
        ctx.accounts.token_0_program.to_account_info(),
        token_0_amount,
        ctx.accounts.token_0_mint.decimals,
    )?;


    create_token_account(
        &ctx.accounts.presale_state.to_account_info(),
        &ctx.accounts.creator.to_account_info(),
        &ctx.accounts.token_1_vault.to_account_info(),
        &ctx.accounts.token_1_mint.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        &ctx.accounts.token_1_program.to_account_info(),
        &[&[
            b"token_1_vault",
            ctx.accounts.presale_state.key().as_ref(),
            ctx.accounts.token_1_mint.key().as_ref(),
            &[ctx.bumps.token_1_vault][..],
        ][..]],
    )?;

    ctx.accounts.presale_state.set_inner(Presale {
        bump: ctx.bumps.presale_state,
        creator: ctx.accounts.creator.key(),
        index: index,
        token_0_mint: ctx.accounts.token_0_mint.key(),
        token_0_amount: token_0_amount,
        token_1_mint: ctx.accounts.token_1_mint.key(),
        token_1_amount: 0,
        token_0_vault: ctx.accounts.token_0_vault.key(),
        token_1_vault: ctx.accounts.token_1_vault.key(),
        presale_price_x32: presale_price_x32
    });
    Ok(())
}
use anchor_lang::prelude::*;

pub mod instructions;
pub mod error;
pub mod states;
pub mod utils;

use instructions::*;

declare_id!("4EzeyFsLrTWcm53oC1riy2mVyh3nQAB5GP63byTpm1rd");

pub mod usdc {
    use anchor_lang::prelude::declare_id;
    #[cfg(feature="devnet")]
    declare_id!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
    #[cfg(not(feature="devnet"))]
    declare_id!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
}

#[program]
pub mod presale_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,  index: u16, token_0_amount: u64, presale_price_x32: u64) -> Result<()> {
        instructions::initialize(ctx, index,token_0_amount,presale_price_x32)
    }

    pub fn buy_token0(ctx: Context<BuyToken0>, token1_amount: u64) -> Result<()> {
        instructions::buy_token0(ctx, token1_amount)
    }
}



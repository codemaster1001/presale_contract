use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Mint0NotToken2022")]
    Mint0NotToken2022,

    #[msg("Mint1NotUSDC")]
    Mint1NotUSDC,

    #[msg("InvalidInput")]
    InvalidInput,
}
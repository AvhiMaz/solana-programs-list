#![cfg_attr(not(test), no_std)]

pub mod instructions;
pub mod state;

pub use instructions::*;
use quasar_lang::prelude::*;

declare_id!("49BmG32poqLgu6RSajXSBUYxPsp9JZhvNCgmVwDzeRWb");

#[program]
mod quasar_escrow {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn make(ctx: Ctx<Make>, deposit: u64, receive: u64) -> Result<(), ProgramError> {
        ctx.accounts.make_escrow(receive, &ctx.bumps)?;
        ctx.accounts.deposit_tokens(deposit)
    }
}

#[cfg(test)]
mod tests;

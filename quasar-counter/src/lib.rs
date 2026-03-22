#![cfg_attr(not(test), no_std)]

pub mod instructions;
pub mod states;
pub use instructions::*;
use quasar_lang::prelude::*;
pub use states::*;

declare_id!("9QWoDrTfMpWpgeh8qP9FqY6NdyqordB6qbfndEw6HjxY");

#[program]
mod quasar_counter {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn initialize(ctx: Ctx<Initialize>) -> Result<(), ProgramError> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    #[instruction(discriminator = 1)]
    pub fn increment(ctx: Ctx<Increment>) -> Result<(), ProgramError> {
        ctx.accounts.increment()
    }

    #[instruction(discriminator = 2)]
    pub fn decrement(ctx: Ctx<Decrement>) -> Result<(), ProgramError> {
        ctx.accounts.decrement()
    }
}

#[cfg(test)]
mod tests;

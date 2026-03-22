#![cfg_attr(not(test), no_std)]

pub mod instructions;
pub mod states;
pub use instructions::*;
use quasar_lang::prelude::*;
pub use states::*;

declare_id!("9QWoDrTfMpWpgeh8qP9FqY6NdyqordB6qbfndEw6HjxY");

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub payer: &'info mut Signer,
    #[account(init, seeds = [b"counter"], bump)]
    pub counter: &'info mut Account<Counter>,
    pub system_program: &'info Program<System>,
}

impl<'info> Initialize<'info> {
    #[inline(always)]
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<(), ProgramError> {
        self.counter.set_inner(0, bumps.counter);
        Ok(())
    }
}

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

    //#[instruction(discriminator = 2)]
    //pub fn decrement(ctx: Ctx<Initialize>) -> Result<(), ProgramError> {
    //    ctx.accounts.initialize()
    //}
}

#[cfg(test)]
mod tests;

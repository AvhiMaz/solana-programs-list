use crate::states::Counter;
use quasar_lang::prelude::*;

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

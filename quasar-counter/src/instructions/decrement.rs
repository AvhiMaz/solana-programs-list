use crate::states::Counter;
use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct Decrement<'info> {
    pub payer: &'info mut Signer,
    #[account(mut, seeds = [b"counter"], bump = counter.bump)]
    pub counter: &'info mut Account<Counter>,
    pub system_program: &'info Program<System>,
}

impl<'info> Decrement<'info> {
    #[inline(always)]
    pub fn decrement(&mut self) -> Result<(), ProgramError> {
        self.counter.count -= 1;
        Ok(())
    }
}

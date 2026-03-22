use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct Increment<'info> {
    pub payer: &'info mut Signer,
    pub system_program: &'info Program<System>,
}

impl<'info> Increment<'info> {
    #[inline(always)]
    pub fn increment(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}

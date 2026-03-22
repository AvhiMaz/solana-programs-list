use quasar_lang::prelude::*;

#[account(discriminator = 1)]
pub struct Counter {
    pub count: u64,
    pub bump: u8,
}

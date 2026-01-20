use anchor_lang::prelude::*;

declare_id!("JD4N57pZSxKUyzFsJTaHw6xcEbswNAmUxenzpWBJw8Nz");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

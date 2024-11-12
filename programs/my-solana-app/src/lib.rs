use anchor_lang::prelude::*;

declare_id!("9EEKqW83vxEStkjVQ44f9gMKyviySsxKFH8APC2XVVSg");

#[program]
pub mod my_solana_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

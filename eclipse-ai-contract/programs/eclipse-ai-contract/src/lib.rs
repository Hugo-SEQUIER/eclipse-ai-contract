use anchor_lang::prelude::*;

declare_id!("GsmQ5nMCAveArB81aJ4G3U1QTkXhayPkMecyfQpMhZpu");

#[program]
pub mod eclipse_ai_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

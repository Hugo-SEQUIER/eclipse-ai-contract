use anchor_lang::prelude::*;

declare_id!("GsmQ5nMCAveArB81aJ4G3U1QTkXhayPkMecyfQpMhZpu");

#[program]
pub mod eclipse_ai_contract {
    use super::*;

    pub fn sign_access(ctx: Context<SignAccess>) -> Result<()> {
        // This function doesn't need to do anything other than succeed
        // The mere fact that it was called and succeeded is enough to grant access
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SignAccess<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
}

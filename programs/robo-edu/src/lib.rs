use anchor_lang::prelude::*;

declare_id!("64CjAsB2ndjRk43aoCSRsJC6zvy9rN7d719MnxFofRUE");  // Keep your Program ID

#[program]
pub mod robo_edu {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("🚕 Robotaxi #001 initialized on devnet!");
        msg!("Owner: {}", ctx.accounts.user.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
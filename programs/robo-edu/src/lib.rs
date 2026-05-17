use anchor_lang::prelude::*;

declare_id!("64CjAsB2ndjRk43aoCSRsJC6zvy9rN7d719MnxFofRUE");  // Keep your Program ID

// This is the INSTRUCTION (the function)
#[program]
pub mod robo_edu {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("🚕 Robotaxi #001 initialized on devnet!");
        msg!("Owner: {}", ctx.accounts.user.key());
        Ok(())
    }
}

// This is the ACCOUNTS STRUCT (the checklist)
#[derive(Accounts)]             // tells Anchor to auto-generate the validation logic for that checklist. Without it, you'd have to write hundreds of lines of manual checks yourself. Anchor does it for you.
pub struct Initialize<'info> {                      // Before the initialize instruction runs, Solana checks: "did the caller provide everything on this list?" The struct defines what's required.
    #[account(mut)]                 // means this account's data is allowed to change during this instruction. mut is short for mutable — the opposite of read-only
    pub user: Signer<'info>,                         // ← must provide: a user who signed the transaction   
    pub system_program: Program<'info, System>,        // ← must provide: the system program
}

// Defines the on-chain data shape for a robotaxi vehicle
#[account]
pub struct VehicleAccount {
    pub authority: Pubkey,           // who manages this vehicle (32 bytes)
    pub vehicle_id: u64,             // unique identifier (8 bytes)
    pub total_shares: u64,           // maximum shares available e.g. 1000 (8 bytes)
    pub shares_issued: u64,          // how many shares have been sold so far (8 bytes)
    pub revenue_accumulated: u64,    // ride revenue in lamports (8 bytes)
    pub is_active: bool,             // has the taxi been purchased and is operating (1 byte)
    pub bump: u8,                    // PDA bump seed - we'll explain this next (1 byte)
}


// If anything on the checklist is missing or wrong, the transaction is rejected before your code even runs.



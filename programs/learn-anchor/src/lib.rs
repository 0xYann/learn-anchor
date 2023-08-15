use anchor_lang::prelude::*;

declare_id!("D4k3kCKpHCdoP5YxVRsR1fTRPB7NtTfCtp8cDLstfQms");

#[program]
pub mod learn_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

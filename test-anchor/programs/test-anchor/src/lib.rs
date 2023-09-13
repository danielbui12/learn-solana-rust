use anchor_lang::prelude::*;

declare_id!("Ec2mfjaAxxBYvtEKwK1i9tdgGm5xog5gDn5nLKmhDBFU");

#[program]
pub mod test_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

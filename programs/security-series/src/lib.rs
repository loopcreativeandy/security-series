use anchor_lang::prelude::*;

declare_id!("SECmF7dX572jE1S6KGchN6uxi9TMXwPZWUwArfQdgYn");

#[program]
pub mod security_series {
    use anchor_lang::system_program;

    use super::*;

    pub fn distribute(ctx: Context<DistributeAccounts>, bump: u8) -> Result<()> {
        
        system_program::create_account(
            CpiContext::new_with_signer(ctx.accounts.system_program.to_account_info(), 
                system_program::CreateAccount{
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.claimed.to_account_info()
                }, 
                &[&[b"claimed", ctx.accounts.user.key().as_ref(), &[bump]]]), 
            1176249, 41, ctx.program_id)?;

        msg!("here - claim this cool stuff!");

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct DistributeAccounts<'info> {
    pub user: Signer<'info>,
    /// CHECK: I totally know what I'm doing here!
    #[account(
      mut, seeds=[b"claimed", user.key().as_ref()], bump = bump
    )]
    pub claimed: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ClaimedAccount {
    pub claimee: Pubkey,
    pub bump: u8
}

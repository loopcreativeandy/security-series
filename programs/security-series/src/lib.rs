use anchor_lang::prelude::*;

declare_id!("SECmF7dX572jE1S6KGchN6uxi9TMXwPZWUwArfQdgYn");

#[program]
pub mod security_series {
    use anchor_lang::system_program;

    use super::*;

    pub fn admin_signup(ctx: Context<AdminSignupAccounts>) -> Result<()> {
        // not implemented yet

        err!(MyError::NotImplementedYet)
    }

    pub fn user_signup(ctx: Context<UserSignupAccounts>, _name: String) -> Result<()> {
        Ok(())
    }
    
    pub fn do_admin_stuff(ctx: Context<AdminAccounts>) -> Result<()> {
        msg!("you have all the power!");
        Ok(())
    }

    pub fn do_user_stuff(ctx: Context<UserAccounts>, _name: String) -> Result<()> {
        msg!("HAHA! you are just a user!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AdminSignupAccounts<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
      init, payer = admin, space = 8+32+1,
      seeds=[b"useradmin", admin.key().as_ref()], bump
    )]
    pub admin_account: Account<'info, UserAccout>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct UserSignupAccounts<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
      init, payer = user, space = 8+32+1,
      seeds=[b"user", name.as_bytes(), user.key().as_ref()], bump
    )]
    pub user_account: Account<'info, UserAccout>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdminAccounts<'info> {
    pub admin: Signer<'info>,
    #[account(
      mut,
      seeds=[b"useradmin", admin.key().as_ref()], bump
    )]
    pub admin_account: Account<'info, UserAccout>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct UserAccounts<'info> {
    pub user: Signer<'info>,
    #[account(
        mut, seeds=[b"user", name.as_bytes(), user.key().as_ref()], bump
    )]
    pub user_account: Account<'info, UserAccout>
}

#[account]
pub struct UserAccout {
    pub user: Pubkey,
    pub bump: u8
}

#[error_code]
pub enum MyError {
    #[msg("MyAccount may only hold data below 100")]
    NotImplementedYet,
}

use anchor_lang::prelude::*;
use solana_program::{
    sysvar::{rent::Rent, Sysvar, self}, hash::Hash, program_error::ProgramError,
};

declare_id!("CaskxYs2fbFggrf1wsccAQGRKL3FgGM8vWUsJ1khMdHs");

#[program]
pub mod security_series {

    use super::*;

    pub fn init_player(ctx: Context<InitPlayerAccounts>) -> Result<()> {
        msg!("setup player account");
        ctx.accounts.player_account.player = *ctx.accounts.player.key;
        ctx.accounts.player_account.bump = *ctx.bumps.get("player_account").unwrap();
        ctx.accounts.player_account.points = 0;
        ctx.accounts.player_account.lucky_number = 0;

        Ok(())
    }

    pub fn play(ctx: Context<PlayAccounts>, round: u32) -> Result<()> {
        msg!("let's play");
        msg!("player1 points: {}", ctx.accounts.player1_account.points);
        msg!("player2 points: {}", ctx.accounts.player2_account.points);

        let (r1, r2) = get_pseudo_random_nrs(&ctx.accounts.sysvar_slothahses_account)?;
        // msg!("random numbers: {} {}", r1, r2);

        ctx.accounts.player1_account.lucky_number = r1;
        ctx.accounts.player2_account.lucky_number = r2;

        
        Ok(())
    }

    pub fn evaluate(ctx: Context<PlayAccounts>) -> Result<()> {
        
        let (winner, looser) = if ctx.accounts.player1_account.lucky_number > ctx.accounts.player2_account.lucky_number {
            msg!("player 1 won!");
            (&mut ctx.accounts.player1_account, &mut ctx.accounts.player2_account)
        } else {
            msg!("player 2 won!");
            (&mut ctx.accounts.player2_account, &mut ctx.accounts.player1_account)
        };

        winner.points += looser.points/4 + 100;
        looser.points = 0;

        msg!("winner points: {}", winner.points);
        msg!("player1 points: {}", ctx.accounts.player1_account.points);
        msg!("player2 points: {}", ctx.accounts.player2_account.points);

        Ok(())
    }


}


#[derive(Accounts)]
pub struct PlayAccounts<'info> {
    #[account(mut)]
    pub player1: Signer<'info>,
    #[account(mut)]
    pub player2: Signer<'info>,
    #[account(
      mut, 
      seeds=[b"player", player1.key().as_ref()], bump = player1_account.bump
    )]
    pub player1_account: Account<'info, PlayerAccount>,
    #[account(
        mut, 
        seeds=[b"player", player2.key().as_ref()], bump = player2_account.bump
      )]
    pub player2_account: Account<'info, PlayerAccount>,
    /// CHECK: we check that manually in the program
    pub sysvar_slothahses_account: UncheckedAccount<'info>
}

#[derive(Accounts)]
pub struct InitPlayerAccounts<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(init, payer = player, space = 8+32+1+4+4, seeds=[b"player", player.key().as_ref()], bump)]
    pub player_account: Account<'info, PlayerAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct NoAccounts<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PlayerAccount {
    pub player: Pubkey,
    pub bump: u8,
    pub points: u32,
    pub lucky_number: u32
}

#[error_code]
pub enum MyError {
    #[msg("Some error")]
    MaybeINeedThatLater,
}





fn get_pseudo_random_nrs(sysvar_slothahses_account: &AccountInfo) -> Result<(u32, u32)>{
        
    if *sysvar_slothahses_account.key != sysvar::slot_hashes::id() {
        msg!("Invalid SlotHashes sysvar");
        return Err(ProgramError::InvalidArgument.into());
    }

    let slot_data = sysvar_slothahses_account.try_borrow_data()?;

    // let num_slot_hashes = u64::from_le_bytes(data[0..8].try_into().unwrap());
    let hash_nr = 1;
    let mut offset = 8 // u64 storing number of hashes
        + (8 + 32) * hash_nr; // more recent entries
    let slot_number = u64::from_le_bytes(slot_data[offset..offset + 8].try_into().unwrap());
    offset+=8; // slot number
    let slot_hash = &slot_data[offset..offset + 32];

    msg!("Using hash from slot {}: {}", slot_number, Hash::new(slot_hash));

    let random_number1 = u32::from_le_bytes(slot_hash[10..14].try_into().unwrap());
    let random_number2 = u32::from_le_bytes(slot_hash[16..20].try_into().unwrap());
    msg!("Calculated pseudo-random numbers: {} {}", random_number1, random_number2);

    Ok((random_number1, random_number2))
}

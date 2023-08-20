use anchor_lang::prelude::*;
use solana_program::{
    sysvar::{rent::Rent, Sysvar, self}, hash::Hash, program_error::ProgramError,
};

declare_id!("CaskxYs2fbFggrf1wsccAQGRKL3FgGM8vWUsJ1khMdHs");

const COINFLIP_FEE : u64 = 10000000;

#[program]
pub mod security_series {

    use super::*;

    pub fn init_treasury(ctx: Context<InitAccounts>) -> Result<()> {
        msg!("setup treasury account");
        ctx.accounts.treasury_account.bump = *ctx.bumps.get("treasury_account").unwrap();

        Ok(())
    }

    pub fn flip(ctx: Context<CoinFlipAccounts>) -> Result<()> {
        msg!("let's flip a coin!");

        // if **ctx.accounts.player.to_account_info().try_borrow_lamports()? < COINFLIP_FEE {
        //     return err!(MyError::InsufficentFunds);
        // };
        // msg!("you have enough funds {}!", **ctx.accounts.player.to_account_info().try_borrow_lamports()?);


        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.player.key(),
            &ctx.accounts.treasury_account.key(),
            COINFLIP_FEE,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[ctx.accounts.player.to_account_info(), ctx.accounts.treasury_account.to_account_info()],
        )?;

        let win = get_pseudo_random_bit(&ctx.accounts.sysvar_slothahses_account)?;
        if win {
            msg!("congratulations! you have won!");
            **ctx.accounts.treasury_account.to_account_info().try_borrow_mut_lamports()? -= COINFLIP_FEE*2;
            **ctx.accounts.player.to_account_info().try_borrow_mut_lamports()? += COINFLIP_FEE*2;
        } else {
            msg!("you have lost! better luck next time!");
        };
        
        Ok(())
    }

}


#[derive(Accounts)]
pub struct InitAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer, space = 8+1, seeds=[b"treasury"], bump)]
    pub treasury_account: Account<'info, TreasuryAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CoinFlipAccounts<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds=[b"treasury"], bump = treasury_account.bump)]
    pub treasury_account: Account<'info, TreasuryAccount>,
    pub system_program: Program<'info, System>,
    /// CHECK: we check that manually in the program
    pub sysvar_slothahses_account: UncheckedAccount<'info>
}

#[account]
pub struct TreasuryAccount {
    pub bump: u8,
}

#[error_code]
pub enum MyError {
    #[msg("Insufficient funds for flip")]
    InsufficentFunds,
}





fn get_pseudo_random_bit(sysvar_slothahses_account: &AccountInfo) -> Result<bool>{
        
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
    let random_bit = random_number1 % 2 == 1;
    msg!("Calculated pseudo-random number: {} -> {}", random_number1, random_bit);

    Ok(random_bit)
}

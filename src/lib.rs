use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::program_error::ProgramError;

declare_id!("AFDNGbaMr2SqHKZnhXSTkbVB2d6npfxQdFFthrzsD7KN");

pub mod states;
pub mod constants;
use crate::{states::*, constants::*};

#[program]
pub mod eminence_voucher{
    use super::*;

    pub fn generate_voucher(ctx: Context<GenerateVoucher>, uid: String, passphrase: String, amount: u64) -> ProgramResult {
        let voucher = &mut ctx.accounts.voucher;

        //Error handling
        if voucher.claimed {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        if voucher.initialized {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        if amount == 0 {
            return Err(ProgramError::InsufficientFunds);
        }

        //Intialize Empty PDA
        voucher.initialized = true;
        voucher.authority = ctx.accounts.authority.key();
        voucher.amount = amount;
        voucher.claimed = false;

        //Fund the PDA
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.authority.key(),
            &ctx.accounts.voucher.key(),
            amount,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.voucher.to_account_info(),
            ],
        );

        Ok(())
    }

    pub fn redeem_voucher(ctx: Context<RedeemVoucher>, uid: String, passphrase: String) -> ProgramResult {
        let voucher = &mut ctx.accounts.voucher;
        let authority = &mut ctx.accounts.authority;

        //Error Handling
        if voucher.claimed {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        //Withdraw
        **voucher.to_account_info().try_borrow_mut_lamports()? -= voucher.amount;
        **authority.to_account_info().try_borrow_mut_lamports()? += voucher.amount;
        voucher.claimed = true;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(uid : String, passphrase : String)]
pub struct GenerateVoucher<'info>{
    #[account(mut)]
    pub authority : Signer<'info>,

    #[account(init,
              seeds = [VOUCHER_TAG, uid.as_bytes(), passphrase.as_bytes()],
              bump,
              payer = authority,
              space = 8 + std::mem::size_of::<Voucher>()
              )]
    pub voucher : Box<Account<'info, Voucher>>,

    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
#[instruction(uid : String, passphrase : String)]
pub struct RedeemVoucher<'info>{
    #[account(mut)]
    pub authority : Signer<'info>,

    #[account(mut,
              seeds = [VOUCHER_TAG, uid.as_bytes(), passphrase.as_bytes()],
              bump
              )]
    pub voucher : Box<Account<'info, Voucher>>,
}




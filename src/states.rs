use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Voucher{
    pub authority : Pubkey,
    pub amount : u64,
    pub initialized : bool,
    pub claimed : bool,
}
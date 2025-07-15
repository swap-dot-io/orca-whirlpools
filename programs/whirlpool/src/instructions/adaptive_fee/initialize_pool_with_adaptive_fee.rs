use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::*;

#[derive(Accounts)]
pub struct InitializePoolWithAdaptiveFee<'info> {
    pub whirlpools_config: Box<Account<'info, WhirlpoolsConfig>>,

    pub token_mint_a: Box<InterfaceAccount<'info, Mint>>,
    pub token_mint_b: Box<InterfaceAccount<'info, Mint>>,

    #[account(seeds = [b"token_badge", whirlpools_config.key().as_ref(), token_mint_a.key().as_ref()], bump)]
    /// CHECK: checked in the handler
    pub token_badge_a: UncheckedAccount<'info>,
    #[account(seeds = [b"token_badge", whirlpools_config.key().as_ref(), token_mint_b.key().as_ref()], bump)]
    /// CHECK: checked in the handler
    pub token_badge_b: UncheckedAccount<'info>,

    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(constraint = adaptive_fee_tier.is_valid_initialize_pool_authority(initialize_pool_authority.key()))]
    pub initialize_pool_authority: Signer<'info>,

    #[account(init,
      seeds = [
        b"whirlpool".as_ref(),
        whirlpools_config.key().as_ref(),
        token_mint_a.key().as_ref(),
        token_mint_b.key().as_ref(),
        adaptive_fee_tier.fee_tier_index.to_le_bytes().as_ref()
      ],
      bump,
      payer = funder,
      space = Whirlpool::LEN)]
    pub whirlpool: Box<Account<'info, Whirlpool>>,

    #[account(
        init,
        payer = funder,
        seeds = [b"oracle", whirlpool.key().as_ref()],
        bump,
        space = Oracle::LEN)]
    pub oracle: AccountLoader<'info, Oracle>,

    /// CHECK: initialized in the handler
    #[account(mut)]
    pub token_vault_a: Signer<'info>,

    /// CHECK: initialized in the handler
    #[account(mut)]
    pub token_vault_b: Signer<'info>,

    #[account(has_one = whirlpools_config)]
    pub adaptive_fee_tier: Box<Account<'info, AdaptiveFeeTier>>,

    #[account(address = *token_mint_a.to_account_info().owner)]
    pub token_program_a: Interface<'info, TokenInterface>,
    #[account(address = *token_mint_b.to_account_info().owner)]
    pub token_program_b: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    _ctx: Context<InitializePoolWithAdaptiveFee>,
    _initial_sqrt_price: u128,
    _trade_enable_timestamp: Option<u64>,
) -> Result<()> {
    Ok(())
}

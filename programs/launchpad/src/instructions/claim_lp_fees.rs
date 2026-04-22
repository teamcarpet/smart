use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::cpi_meteora::{self, ClaimPositionFeeAccounts, METEORA_PROGRAM_ID, POOL_AUTHORITY};
use crate::errors::LaunchpadError;
use crate::state::BuybackState;

#[derive(Accounts)]
pub struct ClaimLpFees<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [BuybackState::SEED, buyback_state.pool.as_ref()],
        bump = buyback_state.bump,
    )]
    pub buyback_state: Box<Account<'info, BuybackState>>,

    #[account(
        mut,
        token::authority = buyback_state,
        seeds = [BuybackState::LP_FEE_VAULT_SEED, buyback_state.pool.as_ref(), lp_fee_vault.mint.as_ref()],
        bump,
    )]
    pub lp_fee_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = lp_fee_vault.mint)]
    pub creator_fee_account: Account<'info, TokenAccount>,

    #[account(mut, token::mint = lp_fee_vault.mint)]
    pub protocol_fee_account: Account<'info, TokenAccount>,

    #[account(mut, token::mint = lp_fee_vault.mint)]
    pub keeper_fee_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SplitClaimedFees<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [BuybackState::SEED, buyback_state.pool.as_ref()],
        bump = buyback_state.bump,
    )]
    pub buyback_state: Account<'info, BuybackState>,

    #[account(
        mut,
        token::authority = buyback_state,
        seeds = [BuybackState::LP_FEE_VAULT_SEED, buyback_state.pool.as_ref(), lp_fee_vault.mint.as_ref()],
        bump,
    )]
    pub lp_fee_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = lp_fee_vault.mint)]
    pub creator_fee_account: Account<'info, TokenAccount>,

    #[account(mut, token::mint = lp_fee_vault.mint)]
    pub protocol_fee_account: Account<'info, TokenAccount>,

    #[account(mut, token::mint = lp_fee_vault.mint)]
    pub keeper_fee_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct HarvestAndSplitLpFees<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [BuybackState::SEED, buyback_state.pool.as_ref()],
        bump = buyback_state.bump,
    )]
    pub buyback_state: Box<Account<'info, BuybackState>>,

    /// CHECK: Program PDA that owns/custodies the Meteora LP position.
    #[account(
        seeds = [BuybackState::LP_CUSTODY_SEED, buyback_state.pool.as_ref()],
        bump,
        constraint = lp_custody.key() == buyback_state.lp_custody @ LaunchpadError::InvalidPoolParams,
    )]
    pub lp_custody: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        token::mint = token_a_mint,
        token::authority = buyback_state,
        seeds = [BuybackState::LP_FEE_VAULT_SEED, buyback_state.pool.as_ref(), token_a_mint.key().as_ref()],
        bump,
    )]
    pub token_a_fee_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = payer,
        token::mint = token_b_mint,
        token::authority = buyback_state,
        seeds = [BuybackState::LP_FEE_VAULT_SEED, buyback_state.pool.as_ref(), token_b_mint.key().as_ref()],
        bump,
    )]
    pub token_b_fee_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut, token::mint = token_a_mint)]
    pub creator_fee_account_a: Box<Account<'info, TokenAccount>>,
    #[account(mut, token::mint = token_a_mint)]
    pub protocol_fee_account_a: Box<Account<'info, TokenAccount>>,
    #[account(mut, token::mint = token_a_mint)]
    pub keeper_fee_account_a: Box<Account<'info, TokenAccount>>,

    #[account(mut, token::mint = token_b_mint)]
    pub creator_fee_account_b: Box<Account<'info, TokenAccount>>,
    #[account(mut, token::mint = token_b_mint)]
    pub protocol_fee_account_b: Box<Account<'info, TokenAccount>>,
    #[account(mut, token::mint = token_b_mint)]
    pub keeper_fee_account_b: Box<Account<'info, TokenAccount>>,

    /// CHECK: Meteora DAMM v2 program
    #[account(constraint = meteora_program.key() == METEORA_PROGRAM_ID @ LaunchpadError::InvalidPoolParams)]
    pub meteora_program: UncheckedAccount<'info>,

    /// CHECK: Meteora pool recorded at migration.
    #[account(
        constraint = meteora_pool.key() == buyback_state.meteora_pool @ LaunchpadError::InvalidPoolParams,
    )]
    pub meteora_pool: UncheckedAccount<'info>,

    /// CHECK: Meteora pool authority PDA.
    #[account(constraint = meteora_pool_authority.key() == POOL_AUTHORITY @ LaunchpadError::InvalidPoolParams)]
    pub meteora_pool_authority: UncheckedAccount<'info>,

    /// CHECK: Meteora position account.
    #[account(mut)]
    pub position: UncheckedAccount<'info>,

    /// CHECK: Token account holding the Meteora position NFT.
    pub position_nft_account: UncheckedAccount<'info>,

    /// CHECK: Meteora pool token A vault.
    #[account(mut)]
    pub meteora_token_a_vault: UncheckedAccount<'info>,

    /// CHECK: Meteora pool token B vault.
    #[account(mut)]
    pub meteora_token_b_vault: UncheckedAccount<'info>,

    /// WSOL/token A mint.
    pub token_a_mint: Box<Account<'info, Mint>>,

    /// Launch token mint.
    #[account(
        constraint = token_b_mint.key() == buyback_state.mint @ LaunchpadError::InvalidPoolParams,
    )]
    pub token_b_mint: Box<Account<'info, Mint>>,

    /// CHECK: Meteora event authority PDA.
    pub meteora_event_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_claim_lp_fees(ctx: Context<ClaimLpFees>) -> Result<()> {
    let claimed = ctx.accounts.lp_fee_vault.amount;
    split_one_side(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.buyback_state.to_account_info(),
        ctx.accounts.buyback_state.pool,
        ctx.accounts.buyback_state.bump,
        ctx.accounts.buyback_state.creator_fee_bps,
        ctx.accounts.buyback_state.protocol_fee_bps,
        ctx.accounts.buyback_state.keeper_fee_bps,
        ctx.accounts.lp_fee_vault.to_account_info(),
        ctx.accounts.creator_fee_account.to_account_info(),
        ctx.accounts.protocol_fee_account.to_account_info(),
        ctx.accounts.keeper_fee_account.to_account_info(),
        claimed,
    )?;
    ctx.accounts.buyback_state.total_lp_fees_claimed_a = ctx
        .accounts
        .buyback_state
        .total_lp_fees_claimed_a
        .checked_add(claimed)
        .ok_or(LaunchpadError::MathOverflow)?;
    Ok(())
}

pub fn handle_harvest_and_split_lp_fees(ctx: Context<HarvestAndSplitLpFees>) -> Result<()> {
    let pool = ctx.accounts.buyback_state.pool;
    let lp_custody_bump = ctx.bumps.lp_custody;
    let lp_custody_signer: &[&[&[u8]]] = &[&[
        BuybackState::LP_CUSTODY_SEED,
        pool.as_ref(),
        &[lp_custody_bump],
    ]];

    let before_a = ctx.accounts.token_a_fee_vault.amount;
    let before_b = ctx.accounts.token_b_fee_vault.amount;

    let meteora_accounts = ClaimPositionFeeAccounts {
        pool_authority: ctx.accounts.meteora_pool_authority.to_account_info(),
        pool: ctx.accounts.meteora_pool.to_account_info(),
        position: ctx.accounts.position.to_account_info(),
        token_a_account: ctx.accounts.token_a_fee_vault.to_account_info(),
        token_b_account: ctx.accounts.token_b_fee_vault.to_account_info(),
        token_a_vault: ctx.accounts.meteora_token_a_vault.to_account_info(),
        token_b_vault: ctx.accounts.meteora_token_b_vault.to_account_info(),
        token_a_mint: ctx.accounts.token_a_mint.to_account_info(),
        token_b_mint: ctx.accounts.token_b_mint.to_account_info(),
        position_nft_account: ctx.accounts.position_nft_account.to_account_info(),
        owner: ctx.accounts.lp_custody.to_account_info(),
        token_a_program: ctx.accounts.token_program.to_account_info(),
        token_b_program: ctx.accounts.token_program.to_account_info(),
        event_authority: ctx.accounts.meteora_event_authority.to_account_info(),
        meteora_program: ctx.accounts.meteora_program.to_account_info(),
    };

    cpi_meteora::cpi_claim_position_fee(&meteora_accounts, lp_custody_signer)?;

    ctx.accounts.token_a_fee_vault.reload()?;
    ctx.accounts.token_b_fee_vault.reload()?;

    let claimed_a = ctx
        .accounts
        .token_a_fee_vault
        .amount
        .checked_sub(before_a)
        .ok_or(LaunchpadError::MathUnderflow)?;
    let claimed_b = ctx
        .accounts
        .token_b_fee_vault
        .amount
        .checked_sub(before_b)
        .ok_or(LaunchpadError::MathUnderflow)?;

    split_one_side(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.buyback_state.to_account_info(),
        pool,
        ctx.accounts.buyback_state.bump,
        ctx.accounts.buyback_state.creator_fee_bps,
        ctx.accounts.buyback_state.protocol_fee_bps,
        ctx.accounts.buyback_state.keeper_fee_bps,
        ctx.accounts.token_a_fee_vault.to_account_info(),
        ctx.accounts.creator_fee_account_a.to_account_info(),
        ctx.accounts.protocol_fee_account_a.to_account_info(),
        ctx.accounts.keeper_fee_account_a.to_account_info(),
        claimed_a,
    )?;

    split_one_side(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.buyback_state.to_account_info(),
        pool,
        ctx.accounts.buyback_state.bump,
        ctx.accounts.buyback_state.creator_fee_bps,
        ctx.accounts.buyback_state.protocol_fee_bps,
        ctx.accounts.buyback_state.keeper_fee_bps,
        ctx.accounts.token_b_fee_vault.to_account_info(),
        ctx.accounts.creator_fee_account_b.to_account_info(),
        ctx.accounts.protocol_fee_account_b.to_account_info(),
        ctx.accounts.keeper_fee_account_b.to_account_info(),
        claimed_b,
    )?;

    ctx.accounts.buyback_state.total_lp_fees_claimed_a = ctx
        .accounts
        .buyback_state
        .total_lp_fees_claimed_a
        .checked_add(claimed_a)
        .ok_or(LaunchpadError::MathOverflow)?;
    ctx.accounts.buyback_state.total_lp_fees_claimed_b = ctx
        .accounts
        .buyback_state
        .total_lp_fees_claimed_b
        .checked_add(claimed_b)
        .ok_or(LaunchpadError::MathOverflow)?;

    Ok(())
}

pub fn handle_split_claimed_fees(ctx: Context<SplitClaimedFees>) -> Result<()> {
    let claimed = ctx.accounts.lp_fee_vault.amount;
    split_one_side(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.buyback_state.to_account_info(),
        ctx.accounts.buyback_state.pool,
        ctx.accounts.buyback_state.bump,
        ctx.accounts.buyback_state.creator_fee_bps,
        ctx.accounts.buyback_state.protocol_fee_bps,
        ctx.accounts.buyback_state.keeper_fee_bps,
        ctx.accounts.lp_fee_vault.to_account_info(),
        ctx.accounts.creator_fee_account.to_account_info(),
        ctx.accounts.protocol_fee_account.to_account_info(),
        ctx.accounts.keeper_fee_account.to_account_info(),
        claimed,
    )?;
    ctx.accounts.buyback_state.total_lp_fees_claimed_b = ctx
        .accounts
        .buyback_state
        .total_lp_fees_claimed_b
        .checked_add(claimed)
        .ok_or(LaunchpadError::MathOverflow)?;
    Ok(())
}

fn split_one_side<'info>(
    token_program: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    pool: Pubkey,
    bump: u8,
    creator_fee_bps: u16,
    protocol_fee_bps: u16,
    keeper_fee_bps: u16,
    from: AccountInfo<'info>,
    creator_to: AccountInfo<'info>,
    protocol_to: AccountInfo<'info>,
    keeper_to: AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    require!(
        fee_split_is_valid(creator_fee_bps, protocol_fee_bps, keeper_fee_bps),
        LaunchpadError::InvalidFeeConfig
    );
    if amount == 0 {
        return Ok(());
    }

    let creator_amount = split_amount(amount, creator_fee_bps)?;
    let keeper_amount = split_amount(amount, keeper_fee_bps)?;
    let protocol_amount = amount
        .checked_sub(creator_amount)
        .ok_or(LaunchpadError::MathUnderflow)?
        .checked_sub(keeper_amount)
        .ok_or(LaunchpadError::MathUnderflow)?;
    let signer: &[&[&[u8]]] = &[&[BuybackState::SEED, pool.as_ref(), &[bump]]];

    if creator_amount > 0 {
        token::transfer(
            CpiContext::new_with_signer(
                token_program.clone(),
                Transfer {
                    from: from.clone(),
                    to: creator_to,
                    authority: authority.clone(),
                },
                signer,
            ),
            creator_amount,
        )?;
    }

    if keeper_amount > 0 {
        token::transfer(
            CpiContext::new_with_signer(
                token_program.clone(),
                Transfer {
                    from: from.clone(),
                    to: keeper_to,
                    authority: authority.clone(),
                },
                signer,
            ),
            keeper_amount,
        )?;
    }

    if protocol_amount > 0 {
        token::transfer(
            CpiContext::new_with_signer(
                token_program,
                Transfer {
                    from,
                    to: protocol_to,
                    authority,
                },
                signer,
            ),
            protocol_amount,
        )?;
    }

    Ok(())
}

pub fn fee_split_is_valid(
    creator_fee_bps: u16,
    protocol_fee_bps: u16,
    keeper_fee_bps: u16,
) -> bool {
    (creator_fee_bps as u32) + (protocol_fee_bps as u32) + (keeper_fee_bps as u32) == 10_000
}

fn split_amount(amount: u64, bps: u16) -> Result<u64> {
    let value = (amount as u128)
        .checked_mul(bps as u128)
        .ok_or(LaunchpadError::MathOverflow)?
        .checked_div(10_000)
        .ok_or(LaunchpadError::DivisionByZero)?;

    u64::try_from(value).map_err(|_| LaunchpadError::CastOverflow.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fee_split_sums_correctly() {
        assert!(fee_split_is_valid(7000, 2950, 50));
        assert!(fee_split_is_valid(7000, 3000, 0));
        assert!(!fee_split_is_valid(8000, 1999, 0));
    }

    #[test]
    fn deterministic_fee_split_uses_remainder_for_protocol_after_keeper() {
        let total = 101u64;
        let creator = split_amount(total, 7000).unwrap();
        let keeper = split_amount(total, 50).unwrap();
        let protocol = total - creator - keeper;

        assert_eq!(creator, 70);
        assert_eq!(keeper, 0);
        assert_eq!(protocol, 31);
        assert_eq!(creator + protocol + keeper, total);
    }
}

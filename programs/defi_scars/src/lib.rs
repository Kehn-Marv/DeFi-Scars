use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("3EUbCw45m9gWr32QJ4muF4SiofFgggGzqTpWJEfKEhiV"); // placeholder — update after `anchor build`

/// DeFi Scars — A global map of DeFi mistakes.
/// Every pin is a real loss with a lesson attached.
/// Content hashes are stored on-chain for tamper-proof integrity.
#[program]
pub mod defi_scars {
    use super::*;

    /// Initialize the global state. Called once after deployment.
    pub fn initialize(ctx: Context<Initialize>, owner: Pubkey) -> Result<()> {
        let global = &mut ctx.accounts.global_state;
        global.owner = owner;
        global.next_id = 1;
        global.total_scars = 0;
        global.bump = ctx.bumps.global_state;
        Ok(())
    }

    /// Post a new scar (DeFi mistake + lesson).
    /// Requires a 0.01 SOL stake that is forwarded to the treasury.
    pub fn post_scar(
        ctx: Context<PostScar>,
        content_hash: [u8; 32],
        ipfs_cid: String,
        category: u8,
        severity: u8,
        lat: i32,
        lon: i32,
    ) -> Result<()> {
        require!(category <= 6, DefiScarsError::InvalidCategory);
        require!(severity <= 3, DefiScarsError::InvalidSeverity);
        require!(ipfs_cid.len() <= 128, DefiScarsError::CidTooLong);

        // ── Rate Limiting ──────────────────────────────────
        let clock = Clock::get()?;
        let today = clock.unix_timestamp / 86400; // seconds per day
        let user_stats = &mut ctx.accounts.user_stats;

        if user_stats.last_post_day == today as u64 {
            require!(
                user_stats.posts_today < MAX_PER_DAY,
                DefiScarsError::DailyLimitReached
            );
            user_stats.posts_today += 1;
        } else {
            user_stats.last_post_day = today as u64;
            user_stats.posts_today = 1;
        }

        // ── Stake Transfer (0.01 SOL → treasury) ───────────
        let stake = MIN_STAKE;
        require!(
            ctx.accounts.author.lamports() >= stake,
            DefiScarsError::InsufficientFunds
        );

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.author.to_account_info(),
                    to: ctx.accounts.treasury.to_account_info(),
                },
            ),
            stake,
        )?;

        // ── Create Scar ────────────────────────────────────
        let global = &mut ctx.accounts.global_state;
        let scar = &mut ctx.accounts.scar;

        scar.id = global.next_id;
        scar.author = ctx.accounts.author.key();
        scar.content_hash = content_hash;
        scar.ipfs_cid = ipfs_cid;
        scar.category = category;
        scar.severity = severity;
        scar.stake_amount = stake;
        scar.created_at = clock.unix_timestamp;
        scar.saved_me_count = 0;
        scar.flag_count = 0;
        scar.lat = lat;
        scar.lon = lon;
        scar.is_hidden = false;
        scar.bump = ctx.bumps.scar;

        global.next_id += 1;
        global.total_scars += 1;

        emit!(ScarPosted {
            author: scar.author,
            id: scar.id,
            content_hash,
            ipfs_cid: scar.ipfs_cid.clone(),
            category,
            severity,
            lat,
            lon,
            stake_amount: stake,
        });

        Ok(())
    }

    /// Upvote a scar — "this saved me". One vote per wallet per scar.
    pub fn saved_me(ctx: Context<SavedMe>, _scar_id: u64) -> Result<()> {
        let scar = &mut ctx.accounts.scar;

        require!(!scar.is_hidden, DefiScarsError::ScarIsHidden);
        require!(
            scar.author != ctx.accounts.voter.key(),
            DefiScarsError::CannotVoteOwnScar
        );

        // The VoteRecord PDA init guarantees one vote per wallet per scar.
        // If it already exists, Anchor will reject the tx (init constraint).
        let vote_record = &mut ctx.accounts.vote_record;
        vote_record.scar_id = scar.id;
        vote_record.voter = ctx.accounts.voter.key();
        vote_record.bump = ctx.bumps.vote_record;

        scar.saved_me_count += 1;

        emit!(ScarSavedMe {
            id: scar.id,
            voter: ctx.accounts.voter.key(),
            new_count: scar.saved_me_count,
        });

        Ok(())
    }

    /// Flag/report a scar. Auto-hides after FLAG_THRESHOLD flags.
    pub fn flag_scar(ctx: Context<FlagScar>, _scar_id: u64) -> Result<()> {
        let scar = &mut ctx.accounts.scar;

        require!(
            scar.author != ctx.accounts.flagger.key(),
            DefiScarsError::CannotFlagOwnScar
        );

        // The FlagRecord PDA init guarantees one flag per wallet per scar.
        let flag_record = &mut ctx.accounts.flag_record;
        flag_record.scar_id = scar.id;
        flag_record.flagger = ctx.accounts.flagger.key();
        flag_record.bump = ctx.bumps.flag_record;

        scar.flag_count += 1;

        emit!(ScarFlagged {
            id: scar.id,
            flagger: ctx.accounts.flagger.key(),
            new_flag_count: scar.flag_count,
        });

        if scar.flag_count >= FLAG_THRESHOLD && !scar.is_hidden {
            scar.is_hidden = true;
            emit!(ScarHidden { id: scar.id });
        }

        Ok(())
    }
}

/* ═══════════════════════════════════════════════════
   CONSTANTS
   ═══════════════════════════════════════════════════ */

pub const MIN_STAKE: u64 = 10_000_000; // 0.01 SOL in lamports
pub const MAX_PER_DAY: u64 = 3;
pub const FLAG_THRESHOLD: u64 = 5;

// Max string length for IPFS CID (46 chars for CIDv0, up to ~128 for CIDv1)
pub const MAX_CID_LEN: usize = 128;

/* ═══════════════════════════════════════════════════
   ACCOUNTS
   ═══════════════════════════════════════════════════ */

#[account]
#[derive(InitSpace)]
pub struct GlobalState {
    pub owner: Pubkey,       // treasury wallet
    pub next_id: u64,
    pub total_scars: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Scar {
    pub id: u64,
    pub author: Pubkey,
    pub content_hash: [u8; 32],
    #[max_len(128)]
    pub ipfs_cid: String,
    pub category: u8,
    pub severity: u8,
    pub stake_amount: u64,
    pub created_at: i64,
    pub saved_me_count: u64,
    pub flag_count: u64,
    pub lat: i32,
    pub lon: i32,
    pub is_hidden: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct UserStats {
    pub last_post_day: u64,
    pub posts_today: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct VoteRecord {
    pub scar_id: u64,
    pub voter: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct FlagRecord {
    pub scar_id: u64,
    pub flagger: Pubkey,
    pub bump: u8,
}

/* ═══════════════════════════════════════════════════
   INSTRUCTION CONTEXTS
   ═══════════════════════════════════════════════════ */

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + GlobalState::INIT_SPACE,
        seeds = [b"global"],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PostScar<'info> {
    #[account(
        mut,
        seeds = [b"global"],
        bump = global_state.bump
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        init,
        payer = author,
        space = 8 + Scar::INIT_SPACE,
        seeds = [b"scar", global_state.next_id.to_le_bytes().as_ref()],
        bump
    )]
    pub scar: Account<'info, Scar>,

    #[account(
        init_if_needed,
        payer = author,
        space = 8 + UserStats::INIT_SPACE,
        seeds = [b"user_stats", author.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserStats>,

    #[account(mut)]
    pub author: Signer<'info>,

    /// CHECK: The treasury wallet. Validated against global_state.owner.
    #[account(
        mut,
        constraint = treasury.key() == global_state.owner @ DefiScarsError::InvalidTreasury
    )]
    pub treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(scar_id: u64)]
pub struct SavedMe<'info> {
    #[account(
        mut,
        seeds = [b"scar", scar_id.to_le_bytes().as_ref()],
        bump = scar.bump
    )]
    pub scar: Account<'info, Scar>,

    #[account(
        init,
        payer = voter,
        space = 8 + VoteRecord::INIT_SPACE,
        seeds = [b"vote", scar_id.to_le_bytes().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>,

    #[account(mut)]
    pub voter: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(scar_id: u64)]
pub struct FlagScar<'info> {
    #[account(
        mut,
        seeds = [b"scar", scar_id.to_le_bytes().as_ref()],
        bump = scar.bump
    )]
    pub scar: Account<'info, Scar>,

    #[account(
        init,
        payer = flagger,
        space = 8 + FlagRecord::INIT_SPACE,
        seeds = [b"flag", scar_id.to_le_bytes().as_ref(), flagger.key().as_ref()],
        bump
    )]
    pub flag_record: Account<'info, FlagRecord>,

    #[account(mut)]
    pub flagger: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/* ═══════════════════════════════════════════════════
   EVENTS
   ═══════════════════════════════════════════════════ */

#[event]
pub struct ScarPosted {
    pub author: Pubkey,
    pub id: u64,
    pub content_hash: [u8; 32],
    pub ipfs_cid: String,
    pub category: u8,
    pub severity: u8,
    pub lat: i32,
    pub lon: i32,
    pub stake_amount: u64,
}

#[event]
pub struct ScarSavedMe {
    pub id: u64,
    pub voter: Pubkey,
    pub new_count: u64,
}

#[event]
pub struct ScarFlagged {
    pub id: u64,
    pub flagger: Pubkey,
    pub new_flag_count: u64,
}

#[event]
pub struct ScarHidden {
    pub id: u64,
}

/* ═══════════════════════════════════════════════════
   ERRORS
   ═══════════════════════════════════════════════════ */

#[error_code]
pub enum DefiScarsError {
    #[msg("Invalid category (must be 0-6)")]
    InvalidCategory,
    #[msg("Invalid severity (must be 0-3)")]
    InvalidSeverity,
    #[msg("IPFS CID too long (max 128 chars)")]
    CidTooLong,
    #[msg("Daily post limit reached (3 per day)")]
    DailyLimitReached,
    #[msg("Insufficient funds for stake")]
    InsufficientFunds,
    #[msg("Scar is hidden")]
    ScarIsHidden,
    #[msg("Cannot vote on your own scar")]
    CannotVoteOwnScar,
    #[msg("Cannot flag your own scar")]
    CannotFlagOwnScar,
    #[msg("Invalid treasury account")]
    InvalidTreasury,
}

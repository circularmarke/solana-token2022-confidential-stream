use anchor_lang::prelude::*;

declare_id!("Strm111111111111111111111111111111111111111");

#[program]
pub mod confidential_stream {
    use super::*;

    pub fn initialize_stream(
        ctx: Context<InitializeStream>,
        rate_per_second: u64,
        duration_seconds: u64,
    ) -> Result<()> {
        require!(rate_per_second > 0, StreamError::InvalidRate);
        require!(duration_seconds >= 60, StreamError::InvalidDuration);

        let stream = &mut ctx.accounts.stream_vault;
        stream.sender = ctx.accounts.sender.key();
        stream.recipient = ctx.accounts.recipient.key();
        stream.token_2022_mint = ctx.accounts.token_2022_mint.key();
        stream.rate_per_second = rate_per_second;
        stream.start_time = Clock::get()?.unix_timestamp;
        stream.end_time = stream.start_time + duration_seconds as i64;
        stream.total_claimed = 0;
        stream.bump = ctx.bumps.stream_vault;

        emit!(StreamInitializedEvent {
            sender: stream.sender,
            recipient: stream.recipient,
            rate_per_second,
            end_time: stream.end_time,
        });

        Ok(())
    }

    pub fn claim_stream(ctx: Context<ClaimStream>) -> Result<()> {
        let stream = &mut ctx.accounts.stream_vault;
        let now = Clock::get()?.unix_timestamp;

        let effective_time = if now > stream.end_time {
            stream.end_time
        } else {
            now
        };

        let elapsed = (effective_time.saturating_sub(stream.start_time)) as u64;
        let total_vested = elapsed.saturating_mul(stream.rate_per_second);
        let claimable = total_vested.saturating_sub(stream.total_claimed);

        require!(claimable > 0, StreamError::NothingToClaim);
        stream.total_claimed = stream.total_claimed.saturating_add(claimable);

        emit!(StreamClaimedEvent {
            recipient: stream.recipient,
            claimed_amount: claimable,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeStream<'info> {
    #[account(
        init,
        payer = sender,
        space = 8 + StreamVault::LEN,
        seeds = [b"confidential-stream", sender.key().as_ref(), recipient.key().as_ref()],
        bump
    )]
    pub stream_vault: Account<'info, StreamVault>,
    /// CHECK: Token-2022 mint account
    pub token_2022_mint: UncheckedAccount<'info>,
    /// CHECK: Beneficiary
    pub recipient: UncheckedAccount<'info>,
    #[account(mut)]
    pub sender: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimStream<'info> {
    #[account(mut, has_one = recipient)]
    pub stream_vault: Account<'info, StreamVault>,
    pub recipient: Signer<'info>,
}

#[account]
pub struct StreamVault {
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub token_2022_mint: Pubkey,
    pub rate_per_second: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub total_claimed: u64,
    pub bump: u8,
}

impl StreamVault {
    pub const LEN: usize = 32 + 32 + 32 + 8 + 8 + 8 + 8 + 1;
}

#[event]
pub struct StreamInitializedEvent {
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub rate_per_second: u64,
    pub end_time: i64,
}

#[event]
pub struct StreamClaimedEvent {
    pub recipient: Pubkey,
    pub claimed_amount: u64,
}

#[error_code]
pub enum StreamError {
    #[msg("Rate per second must be strictly greater than 0.")]
    InvalidRate,
    #[msg("Stream duration must be at least 60 seconds.")]
    InvalidDuration,
    #[msg("No claimable vested tokens at this timestamp.")]
    NothingToClaim,
}

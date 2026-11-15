use anchor_lang::prelude::*;

declare_id!("n41USxoi9Lf1RyZUVAfCHD5TSeuDBuqDxEpUHzkDqf2");

#[program]
pub mod access_control {
    use super::*;

    /// Initialize access record when asset is created
    pub fn initialize_access_record(ctx: Context<InitializeAccessRecord>) -> Result<()> {
        let mint_key = ctx.accounts.mint.key();
        let access_record = &mut ctx.accounts.access_record;
        access_record.mint = mint_key;
        access_record.owner = ctx.accounts.authority.key();
        access_record.active_rental = None;
        access_record.last_verified = Clock::get()?.unix_timestamp;
        // Compute bump manually to store in the account
        let seeds = &[b"access_record", mint_key.as_ref()];
        let (_, bump) = Pubkey::find_program_address(seeds, ctx.program_id);
        access_record.bump = bump;
        
        emit!(AccessRecordCreated {
            access_record: access_record.key(),
            mint: access_record.mint,
            owner: access_record.owner,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    /// Update access record when rental starts/ends
    pub fn update_access_record(
        ctx: Context<UpdateAccessRecord>,
        active_rental: Option<ActiveRentalInfo>,
    ) -> Result<()> {
        // Verify this is called by marketplace program
        require!(
            ctx.accounts.marketplace_program.key() == marketplace_id(),
            ErrorCode::UnauthorizedProgram
        );
        
        let access_record = &mut ctx.accounts.access_record;
        access_record.active_rental = active_rental.clone();
        access_record.last_verified = Clock::get()?.unix_timestamp;
        
        emit!(AccessRecordUpdated {
            access_record: access_record.key(),
            active_rental: active_rental.clone(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    /// Verify access for a wallet (read-only)
    pub fn verify_access(
        ctx: Context<VerifyAccess>,
        wallet: Pubkey,
    ) -> Result<AccessStatus> {
        let access_record = &ctx.accounts.access_record;
        let clock = Clock::get()?;
        
        // Check if wallet is owner
        if wallet == access_record.owner {
            return Ok(AccessStatus::Owner);
        }
        
        // Check if wallet has active rental
        if let Some(rental_info) = &access_record.active_rental {
            if rental_info.renter == wallet {
                if clock.unix_timestamp < rental_info.expiry {
                    return Ok(AccessStatus::Rented {
                        expiry: rental_info.expiry,
                    });
                } else {
                    return Ok(AccessStatus::NoAccess);
                }
            }
        }
        
        Ok(AccessStatus::NoAccess)
    }
}

// Account Structures

#[account]
pub struct AccessRecord {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub active_rental: Option<ActiveRentalInfo>,
    pub last_verified: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ActiveRentalInfo {
    pub renter: Pubkey,
    pub rental: Pubkey,
    pub expiry: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum AccessStatus {
    Owner,
    Rented { expiry: i64 },
    NoAccess,
}

// Context Structures

#[derive(Accounts)]
pub struct InitializeAccessRecord<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// CHECK: Mint account
    pub mint: UncheckedAccount<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 1 + 73 + 8 + 1,
        seeds = [b"access_record", mint.key().as_ref()],
        bump
    )]
    pub access_record: Account<'info, AccessRecord>,
    
    /// CHECK: Game asset account (for validation)
    pub game_asset: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAccessRecord<'info> {
    /// CHECK: Marketplace program (must be CPI)
    pub marketplace_program: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub access_record: Account<'info, AccessRecord>,
    
    /// CHECK: Rental account (optional, for validation)
    pub rental: Option<UncheckedAccount<'info>>,
}

#[derive(Accounts)]
pub struct VerifyAccess<'info> {
    #[account(
        seeds = [b"access_record", access_record.mint.as_ref()],
        bump = access_record.bump
    )]
    pub access_record: Account<'info, AccessRecord>,
    
    /// CHECK: Clock sysvar
    pub clock: Sysvar<'info, Clock>,
}

// Events

#[event]
pub struct AccessRecordCreated {
    pub access_record: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AccessRecordUpdated {
    pub access_record: Pubkey,
    pub active_rental: Option<ActiveRentalInfo>,
    pub timestamp: i64,
}

// Error Codes

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized program")]
    UnauthorizedProgram,
}

// Marketplace program ID for validation
// This should match the actual deployed marketplace program ID
// For now, we'll use a placeholder that will be updated after deployment
use anchor_lang::solana_program::pubkey::Pubkey;
use std::str::FromStr;

fn marketplace_id() -> Pubkey {
    // This will be updated to the actual marketplace program ID after deployment
    Pubkey::from_str("HA9caQSP8cHkZ7QUU7x4EjaLZ6ET1xoVPSXqj1fsmHt3").unwrap()
}


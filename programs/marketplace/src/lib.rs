use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use spl_token::state::Account as TokenAccountState;
use anchor_lang::solana_program::program_pack::Pack;

declare_id!("G46EH6LskmJpBTvZNewSL4Xe5fkdbcdkMpRM8wQ9T3RH");

#[program]
pub mod marketplace {
    use super::*;

    /// Initialize marketplace program
    pub fn initialize_marketplace(
        ctx: Context<InitializeMarketplace>,
        platform_fee_bps: u16,
        treasury: Pubkey,
    ) -> Result<()> {
        require!(platform_fee_bps <= 1000, ErrorCode::InvalidFee); // Max 10%
        
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.treasury = treasury;
        config.platform_fee_bps = platform_fee_bps;
        config.min_rental_duration = 3600; // 1 hour
        config.max_rental_duration = 2592000; // 30 days
        config.listing_fee = 0;
        config.total_listings = 0;
        config.total_rentals = 0;
        config.total_volume = 0;
        config.is_paused = false;
        config.bump = ctx.bumps.config;
        
        emit!(MarketplaceInitialized {
            authority: config.authority,
            treasury: config.treasury,
            platform_fee_bps,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    /// Create a listing for an asset
    pub fn create_listing(
        ctx: Context<CreateListing>,
        price_per_hour: u64,
        price_per_day: u64,
        max_rental_duration: i64,
        min_rental_duration: i64,
        auto_renewal: bool,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        require!(!config.is_paused, ErrorCode::MarketplacePaused);
        require!(price_per_hour > 0 || price_per_day > 0, ErrorCode::InvalidPrice);
        require!(
            min_rental_duration >= config.min_rental_duration,
            ErrorCode::InvalidDuration
        );
        require!(
            max_rental_duration <= config.max_rental_duration,
            ErrorCode::InvalidDuration
        );
        require!(
            max_rental_duration >= min_rental_duration,
            ErrorCode::InvalidDuration
        );
        
        // Verify token account ownership and amount
        let token_account_data = TokenAccountState::unpack(&ctx.accounts.token_account.to_account_info().try_borrow_data()?)?;
        require!(
            token_account_data.mint == ctx.accounts.mint.key(),
            ErrorCode::Unauthorized
        );
        require!(
            token_account_data.owner == ctx.accounts.authority.key(),
            ErrorCode::Unauthorized
        );
        require!(
            token_account_data.amount == 1,
            ErrorCode::Unauthorized
        );
        
        // Charge listing fee if configured
        if config.listing_fee > 0 {
            **ctx.accounts.treasury.try_borrow_mut_lamports()? += config.listing_fee;
            **ctx.accounts.authority.try_borrow_mut_lamports()? -= config.listing_fee;
        }
        
        let listing = &mut ctx.accounts.listing;
        listing.asset = ctx.accounts.game_asset.key();
        listing.owner = ctx.accounts.authority.key();
        listing.mint = ctx.accounts.mint.key();
        listing.price_per_hour = price_per_hour;
        listing.price_per_day = price_per_day;
        listing.max_rental_duration = max_rental_duration;
        listing.min_rental_duration = min_rental_duration;
        listing.is_available = true;
        listing.auto_renewal = auto_renewal;
        listing.total_rentals = 0;
        listing.total_revenue = 0;
        listing.listed_at = Clock::get()?.unix_timestamp;
        listing.updated_at = Clock::get()?.unix_timestamp;
        listing.bump = ctx.bumps.listing;
        
        let config_account = &mut ctx.accounts.config;
        config_account.total_listings = config_account.total_listings.checked_add(1).unwrap();
        
        emit!(ListingCreated {
            listing: listing.key(),
            asset: listing.asset,
            owner: listing.owner,
            price_per_hour,
            price_per_day,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    /// Update listing parameters
    pub fn update_listing(
        ctx: Context<UpdateListing>,
        price_per_hour: Option<u64>,
        price_per_day: Option<u64>,
        max_rental_duration: Option<i64>,
        is_available: Option<bool>,
    ) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        require!(listing.owner == ctx.accounts.authority.key(), ErrorCode::Unauthorized);
        
        let config = &ctx.accounts.config;
        
        if let Some(price) = price_per_hour {
            require!(price > 0, ErrorCode::InvalidPrice);
            listing.price_per_hour = price;
        }
        
        if let Some(price) = price_per_day {
            require!(price > 0, ErrorCode::InvalidPrice);
            listing.price_per_day = price;
        }
        
        if let Some(duration) = max_rental_duration {
            require!(
                duration <= config.max_rental_duration,
                ErrorCode::InvalidDuration
            );
            require!(
                duration >= listing.min_rental_duration,
                ErrorCode::InvalidDuration
            );
            listing.max_rental_duration = duration;
        }
        
        if let Some(available) = is_available {
            listing.is_available = available;
        }
        
        listing.updated_at = Clock::get()?.unix_timestamp;
        
        emit!(ListingUpdated {
            listing: listing.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    /// Cancel a listing
    pub fn cancel_listing(ctx: Context<CancelListing>) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        require!(listing.owner == ctx.accounts.authority.key(), ErrorCode::Unauthorized);
        
        // Note: In production, you'd want to check for active rentals here
        // For now, we'll allow cancellation
        
        emit!(ListingCancelled {
            listing: listing.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    /// Rent an asset
    pub fn rent_asset(
        ctx: Context<RentAsset>,
        duration: i64,
        game: Option<Pubkey>,
    ) -> Result<()> {
        let listing = &ctx.accounts.listing;
        let config = &ctx.accounts.config;
        
        require!(!config.is_paused, ErrorCode::MarketplacePaused);
        require!(listing.is_available, ErrorCode::ListingNotAvailable);
        require!(
            duration >= listing.min_rental_duration,
            ErrorCode::InvalidDuration
        );
        require!(
            duration <= listing.max_rental_duration,
            ErrorCode::InvalidDuration
        );
        
        // Calculate rental cost
        let hours = duration / 3600;
        let days = duration / 86400;
        
        let cost_by_hour = listing.price_per_hour
            .checked_mul(hours as u64)
            .ok_or(ErrorCode::InvalidPrice)?;
        let cost_by_day = listing.price_per_day
            .checked_mul(days as u64)
            .ok_or(ErrorCode::InvalidPrice)?;
        
        let rental_price = cost_by_hour.min(cost_by_day);
        require!(rental_price > 0, ErrorCode::InvalidPrice);
        
        // Calculate fees
        let platform_fee = rental_price
            .checked_mul(config.platform_fee_bps as u64)
            .and_then(|x| x.checked_div(10000))
            .ok_or(ErrorCode::InvalidPrice)?;
        
        let game_share = if let Some(_game_pubkey) = game {
            // In production, fetch game account and calculate share
            // For now, assume 0
            0
        } else {
            0
        };
        
        let owner_share = rental_price
            .checked_sub(platform_fee)
            .and_then(|x| x.checked_sub(game_share))
            .ok_or(ErrorCode::InvalidPrice)?;
        
        // Transfer payment to escrow
        **ctx.accounts.escrow.try_borrow_mut_lamports()? += rental_price;
        **ctx.accounts.renter.try_borrow_mut_lamports()? -= rental_price;
        
        let clock = Clock::get()?;
        let start_time = clock.unix_timestamp;
        let end_time = start_time.checked_add(duration).ok_or(ErrorCode::InvalidDuration)?;
        
        // Create rental record
        let rental = &mut ctx.accounts.rental;
        rental.asset = listing.asset;
        rental.owner = listing.owner;
        rental.renter = ctx.accounts.renter.key();
        rental.mint = listing.mint;
        rental.listing = listing.key();
        rental.rental_price = rental_price;
        rental.duration = duration;
        rental.start_time = start_time;
        rental.end_time = end_time;
        rental.escrow = ctx.accounts.escrow.key();
        rental.status = RentalStatus::Active;
        rental.auto_renewed = 0;
        rental.game = game;
        rental.created_at = start_time;
        rental.bump = ctx.bumps.rental;
        
        // Create escrow account
        let escrow_account = &mut ctx.accounts.escrow_account;
        escrow_account.rental = rental.key();
        escrow_account.amount = rental_price;
        escrow_account.owner_share = owner_share;
        escrow_account.platform_fee = platform_fee;
        escrow_account.game_share = game_share;
        escrow_account.is_released = false;
        escrow_account.bump = ctx.bumps.escrow_account;
        
        // Update listing
        let listing_account = &mut ctx.accounts.listing;
        listing_account.total_rentals = listing_account.total_rentals.checked_add(1).unwrap();
        listing_account.is_available = false; // Mark as unavailable during rental
        
        // Update marketplace stats
        let config_account = &mut ctx.accounts.config;
        config_account.total_rentals = config_account.total_rentals.checked_add(1).unwrap();
        config_account.total_volume = config_account.total_volume.checked_add(rental_price).unwrap();
        
        emit!(AssetRented {
            rental: rental.key(),
            asset: rental.asset,
            owner: rental.owner,
            renter: rental.renter,
            price: rental_price,
            duration,
            start_time,
            end_time,
            game,
            timestamp: start_time,
        });
        
        Ok(())
    }

    /// Complete a rental and distribute payments
    pub fn complete_rental(ctx: Context<CompleteRental>) -> Result<()> {
        let rental = &mut ctx.accounts.rental;
        let escrow_account = &mut ctx.accounts.escrow_account;
        
        require!(rental.status == RentalStatus::Active, ErrorCode::RentalNotActive);
        
        let clock = Clock::get()?;
        require!(
            clock.unix_timestamp >= rental.end_time,
            ErrorCode::RentalStillActive
        );
        
        // Mark rental as completed
        rental.status = RentalStatus::Completed;
        
        // Distribute funds
        require!(!escrow_account.is_released, ErrorCode::EscrowAlreadyReleased);
        
        **ctx.accounts.owner.try_borrow_mut_lamports()? += escrow_account.owner_share;
        **ctx.accounts.treasury.try_borrow_mut_lamports()? += escrow_account.platform_fee;
        
        if escrow_account.game_share > 0 {
            if let Some(game_account) = &ctx.accounts.game_account {
                **game_account.try_borrow_mut_lamports()? += escrow_account.game_share;
            }
        }
        
        escrow_account.is_released = true;
        
        // Update listing availability
        let listing = &mut ctx.accounts.listing;
        listing.is_available = true;
        listing.total_revenue = listing.total_revenue.checked_add(escrow_account.amount).unwrap();
        
        // Update asset stats if provided
        // TODO: Implement via CPI to asset-registry program
        // For now, asset stats updates are handled by the asset-registry program
        // if let Some(asset_stats) = &mut ctx.accounts.asset_stats {
        //     // This requires CPI to asset-registry program
        // }
        
        emit!(RentalCompleted {
            rental: rental.key(),
            asset: rental.asset,
            renter: rental.renter,
            actual_duration: rental.duration,
            owner_payout: escrow_account.owner_share,
            platform_fee: escrow_account.platform_fee,
            game_share: escrow_account.game_share,
            timestamp: clock.unix_timestamp,
        });
        
        Ok(())
    }

    /// Extend rental duration
    pub fn extend_rental(
        ctx: Context<ExtendRental>,
        additional_duration: i64,
    ) -> Result<()> {
        let rental = &mut ctx.accounts.rental;
        let listing = &ctx.accounts.listing;
        let config = &ctx.accounts.config;
        
        require!(rental.status == RentalStatus::Active, ErrorCode::RentalNotActive);
        require!(rental.renter == ctx.accounts.renter.key(), ErrorCode::Unauthorized);
        
        let new_end_time = rental.end_time
            .checked_add(additional_duration)
            .ok_or(ErrorCode::InvalidDuration)?;
        
        let total_duration = new_end_time
            .checked_sub(rental.start_time)
            .ok_or(ErrorCode::InvalidDuration)?;
        
        require!(
            total_duration <= listing.max_rental_duration,
            ErrorCode::InvalidDuration
        );
        require!(
            total_duration <= config.max_rental_duration,
            ErrorCode::InvalidDuration
        );
        
        // Calculate extension cost
        let hours = additional_duration / 3600;
        let days = additional_duration / 86400;
        
        let cost_by_hour = listing.price_per_hour
            .checked_mul(hours as u64)
            .ok_or(ErrorCode::InvalidPrice)?;
        let cost_by_day = listing.price_per_day
            .checked_mul(days as u64)
            .ok_or(ErrorCode::InvalidPrice)?;
        
        let additional_cost = cost_by_hour.min(cost_by_day);
        
        // Transfer additional payment
        **ctx.accounts.escrow.try_borrow_mut_lamports()? += additional_cost;
        **ctx.accounts.renter.try_borrow_mut_lamports()? -= additional_cost;
        
        // Update rental
        rental.end_time = new_end_time;
        rental.duration = total_duration;
        rental.rental_price = rental.rental_price.checked_add(additional_cost).unwrap();
        
        // Update escrow
        let escrow_account = &mut ctx.accounts.escrow_account;
        escrow_account.amount = escrow_account.amount.checked_add(additional_cost).unwrap();
        
        // Recalculate shares
        let platform_fee = additional_cost
            .checked_mul(config.platform_fee_bps as u64)
            .and_then(|x| x.checked_div(10000))
            .ok_or(ErrorCode::InvalidPrice)?;
        
        escrow_account.platform_fee = escrow_account.platform_fee.checked_add(platform_fee).unwrap();
        escrow_account.owner_share = escrow_account.owner_share.checked_add(
            additional_cost.checked_sub(platform_fee).unwrap()
        ).unwrap();
        
        emit!(RentalExtended {
            rental: rental.key(),
            additional_duration,
            new_end_time,
            additional_cost,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
}

// Account Structures

#[account]
pub struct MarketplaceConfig {
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub platform_fee_bps: u16,
    pub min_rental_duration: i64,
    pub max_rental_duration: i64,
    pub listing_fee: u64,
    pub total_listings: u64,
    pub total_rentals: u64,
    pub total_volume: u64,
    pub is_paused: bool,
    pub bump: u8,
}

#[account]
pub struct Listing {
    pub asset: Pubkey,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub price_per_hour: u64,
    pub price_per_day: u64,
    pub max_rental_duration: i64,
    pub min_rental_duration: i64,
    pub is_available: bool,
    pub auto_renewal: bool,
    pub total_rentals: u64,
    pub total_revenue: u64,
    pub listed_at: i64,
    pub updated_at: i64,
    pub bump: u8,
}

#[account]
pub struct Rental {
    pub asset: Pubkey,
    pub owner: Pubkey,
    pub renter: Pubkey,
    pub mint: Pubkey,
    pub listing: Pubkey,
    pub rental_price: u64,
    pub duration: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub escrow: Pubkey,
    pub status: RentalStatus,
    pub auto_renewed: u8,
    pub game: Option<Pubkey>,
    pub created_at: i64,
    pub bump: u8,
}

#[account]
pub struct Escrow {
    pub rental: Pubkey,
    pub amount: u64,
    pub owner_share: u64,
    pub platform_fee: u64,
    pub game_share: u64,
    pub is_released: bool,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum RentalStatus {
    Active,
    Completed,
    Expired,
    Cancelled,
}

// Context Structures

#[derive(Accounts)]
pub struct InitializeMarketplace<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 2 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 1,
        seeds = [b"marketplace_config"],
        bump
    )]
    pub config: Account<'info, MarketplaceConfig>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateListing<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// CHECK: Game asset account from Asset Registry
    pub game_asset: UncheckedAccount<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 1 + 1 + 8 + 8 + 8 + 8 + 1,
        seeds = [b"listing", game_asset.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,
    
    /// CHECK: Mint account
    pub mint: UncheckedAccount<'info>,
    
    /// CHECK: Token account - validated in instruction
    pub token_account: UncheckedAccount<'info>,
    
    #[account(
        seeds = [b"marketplace_config"],
        bump = config.bump
    )]
    pub config: Account<'info, MarketplaceConfig>,
    
    /// CHECK: Treasury account
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateListing<'info> {
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub listing: Account<'info, Listing>,
    
    #[account(
        seeds = [b"marketplace_config"],
        bump = config.bump
    )]
    pub config: Account<'info, MarketplaceConfig>,
}

#[derive(Accounts)]
pub struct CancelListing<'info> {
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub listing: Account<'info, Listing>,
}

#[derive(Accounts)]
pub struct RentAsset<'info> {
    #[account(mut)]
    pub renter: Signer<'info>,
    
    /// CHECK: Asset owner
    #[account(mut)]
    pub owner: UncheckedAccount<'info>,
    
    /// CHECK: Game asset account
    pub game_asset: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub listing: Account<'info, Listing>,
    
    #[account(
        init,
        payer = renter,
        space = 8 + 32 + 32 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 32 + 1 + 1 + 1 + 32 + 8 + 1,
        seeds = [
            b"rental",
            listing.asset.as_ref(),
            renter.key().as_ref()
        ],
        bump
    )]
    pub rental: Account<'info, Rental>,
    
    #[account(
        init,
        payer = renter,
        space = 8 + 32 + 8 + 8 + 8 + 8 + 1 + 1,
        seeds = [b"escrow", rental.key().as_ref()],
        bump
    )]
    pub escrow_account: Account<'info, Escrow>,
    
    /// CHECK: Escrow PDA for holding funds
    #[account(
        mut,
        seeds = [b"escrow", rental.key().as_ref()],
        bump = escrow_account.bump
    )]
    pub escrow: UncheckedAccount<'info>,
    
    #[account(
        seeds = [b"marketplace_config"],
        bump = config.bump
    )]
    pub config: Account<'info, MarketplaceConfig>,
    
    /// CHECK: Game account (optional)
    pub game_account: Option<UncheckedAccount<'info>>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteRental<'info> {
    #[account(mut)]
    pub rental: Account<'info, Rental>,
    
    #[account(mut)]
    pub escrow_account: Account<'info, Escrow>,
    
    /// CHECK: Asset owner
    #[account(mut)]
    pub owner: UncheckedAccount<'info>,
    
    /// CHECK: Treasury
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,
    
    /// CHECK: Game account (optional)
    #[account(mut)]
    pub game_account: Option<UncheckedAccount<'info>>,
    
    #[account(mut)]
    pub listing: Account<'info, Listing>,
    
    /// CHECK: Asset stats (optional)
    #[account(mut)]
    pub asset_stats: Option<UncheckedAccount<'info>>,
    
    #[account(
        seeds = [b"marketplace_config"],
        bump = config.bump
    )]
    pub config: Account<'info, MarketplaceConfig>,
}

#[derive(Accounts)]
pub struct ExtendRental<'info> {
    #[account(mut)]
    pub renter: Signer<'info>,
    
    #[account(mut)]
    pub rental: Account<'info, Rental>,
    
    #[account(mut)]
    pub escrow_account: Account<'info, Escrow>,
    
    /// CHECK: Escrow PDA
    #[account(
        mut,
        seeds = [b"escrow", rental.key().as_ref()],
        bump = escrow_account.bump
    )]
    pub escrow: UncheckedAccount<'info>,
    
    pub listing: Account<'info, Listing>,
    
    #[account(
        seeds = [b"marketplace_config"],
        bump = config.bump
    )]
    pub config: Account<'info, MarketplaceConfig>,
    
    pub system_program: Program<'info, System>,
}

// Events

#[event]
pub struct MarketplaceInitialized {
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub platform_fee_bps: u16,
    pub timestamp: i64,
}

#[event]
pub struct ListingCreated {
    pub listing: Pubkey,
    pub asset: Pubkey,
    pub owner: Pubkey,
    pub price_per_hour: u64,
    pub price_per_day: u64,
    pub timestamp: i64,
}

#[event]
pub struct ListingUpdated {
    pub listing: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ListingCancelled {
    pub listing: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AssetRented {
    pub rental: Pubkey,
    pub asset: Pubkey,
    pub owner: Pubkey,
    pub renter: Pubkey,
    pub price: u64,
    pub duration: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub game: Option<Pubkey>,
    pub timestamp: i64,
}

#[event]
pub struct RentalCompleted {
    pub rental: Pubkey,
    pub asset: Pubkey,
    pub renter: Pubkey,
    pub actual_duration: i64,
    pub owner_payout: u64,
    pub platform_fee: u64,
    pub game_share: u64,
    pub timestamp: i64,
}

#[event]
pub struct RentalExtended {
    pub rental: Pubkey,
    pub additional_duration: i64,
    pub new_end_time: i64,
    pub additional_cost: u64,
    pub timestamp: i64,
}

// Error Codes

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid fee percentage")]
    InvalidFee,
    #[msg("Marketplace is paused")]
    MarketplacePaused,
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Invalid duration")]
    InvalidDuration,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Listing not available")]
    ListingNotAvailable,
    #[msg("Rental not active")]
    RentalNotActive,
    #[msg("Rental still active")]
    RentalStillActive,
    #[msg("Escrow already released")]
    EscrowAlreadyReleased,
}


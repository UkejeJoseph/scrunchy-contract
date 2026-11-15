use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, Transfer};
use spl_token::state::Account as TokenAccountState;
use anchor_lang::solana_program::program_pack::Pack;
// Note: associated_token feature removed to avoid dependency conflicts in Anchor 0.31.0
// Use spl-associated-token-account directly if needed

declare_id!("7vxpftgcdbh6NW77uAGVxp4aM2RcbQtmeGixNp2ifCDn");

#[program]
pub mod asset_registry {
    use super::*;

    /// Initialize the asset registry program
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.marketplace_fee_bps = 250; // 2.5%
        config.creation_fee = 10_000_000; // 0.01 SOL
        config.bump = ctx.bumps.config;
        
        emit!(ProgramInitialized {
            authority: config.authority,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    /// Register a game for asset integration
    pub fn register_game(
        ctx: Context<RegisterGame>,
        name: String,
        description: String,
        supported_types: Vec<AssetType>,
    ) -> Result<()> {
        require!(name.len() > 0 && name.len() <= 64, ErrorCode::InvalidName);
        require!(description.len() > 0 && description.len() <= 200, ErrorCode::InvalidDescription);
        
        let game = &mut ctx.accounts.registered_game;
        game.authority = ctx.accounts.authority.key();
        game.name = name.clone();
        game.description = description;
        game.supported_types = supported_types;
        game.api_key = generate_api_key(&ctx.accounts.authority.key());
        game.revenue_share_bps = 500; // Default 5%
        game.is_active = true;
        game.is_verified = false;
        game.registered_at = Clock::get()?.unix_timestamp;
        game.bump = ctx.bumps.registered_game;
        
        emit!(GameRegistered {
            game: game.key(),
            authority: game.authority,
            name: name.clone(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    /// Create a new game asset as an NFT
    pub fn create_asset(
        ctx: Context<CreateAsset>,
        name: String,
        asset_type: AssetType,
        rarity: Rarity,
        metadata_uri: String,
        base_attributes: Vec<Attribute>,
        game_mappings: Vec<GameMapping>,
        rental_config: Option<RentalConfig>,
    ) -> Result<()> {
        require!(name.len() > 0 && name.len() <= 64, ErrorCode::InvalidName);
        require!(metadata_uri.len() > 0 && metadata_uri.len() <= 200, ErrorCode::InvalidMetadataUri);
        require!(base_attributes.len() <= 20, ErrorCode::MaxAttributesExceeded);
        require!(game_mappings.len() <= 10, ErrorCode::MaxGameMappingsReached);
        
        // Charge creation fee
        let config = &ctx.accounts.config;
        **ctx.accounts.treasury.try_borrow_mut_lamports()? += config.creation_fee;
        **ctx.accounts.authority.try_borrow_mut_lamports()? -= config.creation_fee;
        
        let game_asset = &mut ctx.accounts.game_asset;
        game_asset.authority = ctx.accounts.authority.key();
        game_asset.owner = ctx.accounts.authority.key();
        game_asset.mint = ctx.accounts.mint.key();
        game_asset.metadata_uri = metadata_uri.clone();
        game_asset.name = name.clone();
        game_asset.asset_type = asset_type;
        game_asset.rarity = rarity;
        game_asset.base_attributes = base_attributes;
        game_asset.game_mappings = game_mappings;
        game_asset.rental_config = rental_config.clone();
        game_asset.created_at = Clock::get()?.unix_timestamp;
        game_asset.updated_at = Clock::get()?.unix_timestamp;
        game_asset.is_rentable = rental_config.is_some();
        game_asset.is_transferable = true;
        game_asset.is_active = true;
        game_asset.bump = ctx.bumps.game_asset;
        
        // Initialize asset stats
        let asset_stats = &mut ctx.accounts.asset_stats;
        asset_stats.asset = game_asset.key();
        asset_stats.total_rentals = 0;
        asset_stats.total_revenue = 0;
        asset_stats.active_rentals = 0;
        asset_stats.total_usage_time = 0;
        asset_stats.games_used = Vec::new();
        asset_stats.last_rental = 0;
        asset_stats.bump = ctx.bumps.asset_stats;
        
        emit!(AssetCreated {
            asset: game_asset.key(),
            mint: game_asset.mint,
            authority: game_asset.authority,
            name: name.clone(),
            asset_type: game_asset.asset_type,
            rarity: game_asset.rarity,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    /// Update asset metadata
    pub fn update_asset(
        ctx: Context<UpdateAsset>,
        metadata_uri: Option<String>,
        base_attributes: Option<Vec<Attribute>>,
        game_mappings: Option<Vec<GameMapping>>,
        rental_config: Option<RentalConfig>,
        is_rentable: Option<bool>,
    ) -> Result<()> {
        let game_asset = &mut ctx.accounts.game_asset;
        
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
        
        if let Some(uri) = metadata_uri {
            require!(uri.len() > 0 && uri.len() <= 200, ErrorCode::InvalidMetadataUri);
            game_asset.metadata_uri = uri;
        }
        
        if let Some(attrs) = base_attributes {
            require!(attrs.len() <= 20, ErrorCode::MaxAttributesExceeded);
            game_asset.base_attributes = attrs;
        }
        
        if let Some(mappings) = game_mappings {
            require!(mappings.len() <= 10, ErrorCode::MaxGameMappingsReached);
            game_asset.game_mappings = mappings;
        }
        
        if let Some(config) = rental_config {
            game_asset.rental_config = Some(config);
        }
        
        if let Some(rentable) = is_rentable {
            game_asset.is_rentable = rentable;
        }
        
        game_asset.updated_at = Clock::get()?.unix_timestamp;
        
        emit!(AssetUpdated {
            asset: game_asset.key(),
            authority: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    /// Add game mapping to asset
    pub fn add_game_mapping(
        ctx: Context<AddGameMapping>,
        game_id: Pubkey,
        attributes: Vec<Attribute>,
    ) -> Result<()> {
        let game_asset = &mut ctx.accounts.game_asset;
        let registered_game = &ctx.accounts.registered_game;
        
        require!(registered_game.is_active, ErrorCode::GameNotActive);
        require!(game_asset.game_mappings.len() < 10, ErrorCode::MaxGameMappingsReached);
        
        // Check if mapping already exists
        require!(
            !game_asset.game_mappings.iter().any(|m| m.game_id == game_id),
            ErrorCode::GameMappingExists
        );
        
        // Verify asset type is supported
        require!(
            registered_game.supported_types.contains(&game_asset.asset_type),
            ErrorCode::AssetTypeNotSupported
        );
        
        game_asset.game_mappings.push(GameMapping {
            game_id,
            attributes,
        });
        
        game_asset.updated_at = Clock::get()?.unix_timestamp;
        
        emit!(GameMappingAdded {
            asset: game_asset.key(),
            game: game_id,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }

    /// Transfer asset to new owner
    pub fn transfer_asset(
        ctx: Context<TransferAsset>,
        new_owner: Pubkey,
    ) -> Result<()> {
        let game_asset = &mut ctx.accounts.game_asset;
        
        require!(game_asset.is_transferable, ErrorCode::AssetNotTransferable);
        
        // Verify from token account ownership and amount
        let from_token_data = TokenAccountState::unpack(&ctx.accounts.from_token_account.to_account_info().try_borrow_data()?)?;
        require!(
            from_token_data.owner == ctx.accounts.authority.key(),
            ErrorCode::Unauthorized
        );
        require!(
            from_token_data.amount == 1,
            ErrorCode::Unauthorized
        );
        
        // Transfer NFT
        let cpi_accounts = Transfer {
            from: ctx.accounts.from_token_account.to_account_info(),
            to: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, 1)?;
        
        // Update owner
        game_asset.owner = new_owner;
        game_asset.updated_at = Clock::get()?.unix_timestamp;
        
        emit!(AssetTransferred {
            asset: game_asset.key(),
            from: ctx.accounts.authority.key(),
            to: new_owner,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
}

// Helper function to generate API key
fn generate_api_key(authority: &Pubkey) -> [u8; 32] {
    let mut key = [0u8; 32];
    let authority_bytes = authority.to_bytes();
    let timestamp = Clock::get().unwrap().unix_timestamp.to_le_bytes();
    
    for i in 0..32 {
        key[i] = authority_bytes[i % authority_bytes.len()] ^ timestamp[i % timestamp.len()];
    }
    
    key
}

// Account Structures

#[account]
pub struct ProgramConfig {
    pub authority: Pubkey,
    pub marketplace_fee_bps: u16,
    pub creation_fee: u64,
    pub bump: u8,
}

#[account]
pub struct GameAsset {
    pub authority: Pubkey,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub metadata_uri: String,
    pub name: String,
    pub asset_type: AssetType,
    pub rarity: Rarity,
    pub base_attributes: Vec<Attribute>,
    pub game_mappings: Vec<GameMapping>,
    pub rental_config: Option<RentalConfig>,
    pub created_at: i64,
    pub updated_at: i64,
    pub is_rentable: bool,
    pub is_transferable: bool,
    pub is_active: bool,
    pub bump: u8,
}

#[account]
pub struct RegisteredGame {
    pub authority: Pubkey,
    pub name: String,
    pub description: String,
    pub api_key: [u8; 32],
    pub supported_types: Vec<AssetType>,
    pub revenue_share_bps: u16,
    pub total_assets_used: u64,
    pub total_rentals: u64,
    pub is_active: bool,
    pub is_verified: bool,
    pub registered_at: i64,
    pub bump: u8,
}

#[account]
pub struct AssetStats {
    pub asset: Pubkey,
    pub total_rentals: u64,
    pub total_revenue: u64,
    pub active_rentals: u32,
    pub total_usage_time: i64,
    pub games_used: Vec<Pubkey>,
    pub last_rental: i64,
    pub bump: u8,
}

// Supporting Structures

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum AssetType {
    Weapon,
    Armor,
    Skill,
    PowerUp,
    Consumable,
    Character,
    Vehicle,
    Other,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct GameMapping {
    pub game_id: Pubkey,
    pub attributes: Vec<Attribute>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RentalConfig {
    pub price_per_hour: u64,
    pub price_per_day: u64,
    pub max_rental_duration: i64,
    pub min_rental_duration: i64,
    pub auto_renewal: bool,
}

// Context Structures

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 2 + 8 + 1,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, ProgramConfig>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterGame<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + 64 + 4 + 200 + 32 + 4 + 10 + 2 + 8 + 8 + 1 + 1 + 8 + 1,
        seeds = [b"registered_game", authority.key().as_ref()],
        bump
    )]
    pub registered_game: Account<'info, RegisteredGame>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateAsset<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 32 + 4 + 200 + 4 + 64 + 1 + 1 + 4 + (100 * 20) + 4 + (132 * 10) + 1 + 41 + 8 + 8 + 1 + 1 + 1 + 1,
        seeds = [b"game_asset", mint.key().as_ref()],
        bump
    )]
    pub game_asset: Account<'info, GameAsset>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + 4 + 8 + 4 + (32 * 10) + 8 + 1,
        seeds = [b"asset_stats", game_asset.key().as_ref()],
        bump
    )]
    pub asset_stats: Account<'info, AssetStats>,
    
    #[account(mut)]
    pub mint: Signer<'info>,
    
    /// CHECK: Token account - validated in instruction
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    
    /// CHECK: Treasury account for fees
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,
    
    /// CHECK: Config account
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProgramConfig>,
}

#[derive(Accounts)]
pub struct UpdateAsset<'info> {
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub game_asset: Account<'info, GameAsset>,
    
    /// CHECK: Mint account
    pub mint: UncheckedAccount<'info>,
    
    /// CHECK: Token account - validated in instruction
    pub token_account: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct AddGameMapping<'info> {
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub game_asset: Account<'info, GameAsset>,
    
    #[account(
        seeds = [b"registered_game", registered_game.authority.as_ref()],
        bump = registered_game.bump
    )]
    pub registered_game: Account<'info, RegisteredGame>,
}

#[derive(Accounts)]
pub struct TransferAsset<'info> {
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub game_asset: Account<'info, GameAsset>,
    
    /// CHECK: Mint account
    pub mint: UncheckedAccount<'info>,
    
    /// CHECK: From token account
    #[account(mut)]
    pub from_token_account: UncheckedAccount<'info>,
    
    /// CHECK: To token account
    #[account(mut)]
    pub to_token_account: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
}

// Events

#[event]
pub struct ProgramInitialized {
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AssetCreated {
    pub asset: Pubkey,
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub name: String,
    pub asset_type: AssetType,
    pub rarity: Rarity,
    pub timestamp: i64,
}

#[event]
pub struct AssetUpdated {
    pub asset: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct GameRegistered {
    pub game: Pubkey,
    pub authority: Pubkey,
    pub name: String,
    pub timestamp: i64,
}

#[event]
pub struct GameMappingAdded {
    pub asset: Pubkey,
    pub game: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AssetTransferred {
    pub asset: Pubkey,
    pub from: Pubkey,
    pub to: Pubkey,
    pub timestamp: i64,
}

// Error Codes

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid name length")]
    InvalidName,
    #[msg("Invalid description length")]
    InvalidDescription,
    #[msg("Invalid metadata URI")]
    InvalidMetadataUri,
    #[msg("Maximum attributes exceeded")]
    MaxAttributesExceeded,
    #[msg("Maximum game mappings reached")]
    MaxGameMappingsReached,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Asset is not transferable")]
    AssetNotTransferable,
    #[msg("Game is not active")]
    GameNotActive,
    #[msg("Game mapping already exists")]
    GameMappingExists,
    #[msg("Asset type not supported by game")]
    AssetTypeNotSupported,
}


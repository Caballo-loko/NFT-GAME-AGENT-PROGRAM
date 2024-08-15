use anchor_lang::prelude::*;
use crate::state::Collection;
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(name: String, symbol: String, strategy: String, collection_id: Pubkey)]
pub struct CreateCollection<'info> {
    #[account(
        init,
        payer = authority,
        space = Collection::LEN,
        seeds = [b"collection", collection_id.as_ref()],
        bump
    )]
    pub collection: Account<'info, Collection>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_collection_handler(
    ctx: Context<CreateCollection>,
    name: String,
    symbol: String,
    strategy: String,
    collection_id: Pubkey,
) -> Result<()> {
    let collection = &mut ctx.accounts.collection;
    
    // Fix for name
    let name_bytes = name.as_bytes();
    collection.name = [0u8; 32];
    collection.name[..name_bytes.len().min(32)].copy_from_slice(&name_bytes[..name_bytes.len().min(32)]);
    
    // Fix for symbol
    let symbol_bytes = symbol.as_bytes();
    collection.symbol = [0u8; 10];
    collection.symbol[..symbol_bytes.len().min(10)].copy_from_slice(&symbol_bytes[..symbol_bytes.len().min(10)]);
    
    // Validate strategy
    validate_strategy(&strategy)?;
    
    // Convert strategy to uppercase and store it
    let strategy_upper = strategy.to_uppercase();
    let strategy_bytes = strategy_upper.as_bytes();
    collection.strategy = [0u8; 10];
    collection.strategy[..strategy_bytes.len().min(10)].copy_from_slice(&strategy_bytes[..strategy_bytes.len().min(10)]);
    
    collection.authority = ctx.accounts.authority.key();
    collection.collection_id = collection_id;
    msg!("Setting collection_id to: {}", collection_id);
    Ok(())
}

// Helper function to validate strategy
pub fn validate_strategy(strategy: &str) -> Result<()> {
    match strategy.to_uppercase().as_str() {
        "DQN" | "A2C" | "PPO" | "DDPG" | "TD3" | "SAC" | "TRPO" | "REINFORCE" => Ok(()),
        _ => {
            msg!("Invalid strategy provided: {}", strategy);
            Err(ErrorCode::InvalidStrategy.into())
        }
    }
}
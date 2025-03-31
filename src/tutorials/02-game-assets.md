# Lesson 2: Implementing Game Assets and Ship System

In this lesson, you'll learn how to create and manage game assets using Stellar's native asset system and smart contracts. We'll implement the ship system and resource management using Stellar's built-in features.

## What You'll Learn
- Creating and managing game assets on Stellar
- Implementing ship system using smart contracts
- Managing resource balances using Stellar's native asset system

## Step 1: Creating Game Assets

First, let's create a new contract for managing game assets. Create `src/assets.rs`:

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec, Map};

#[contract]
pub struct GameAssets;

#[contractimpl]
impl GameAssets {
    // Initialize game assets
    pub fn initialize_assets(env: Env, issuer: Address) -> Symbol {
        // Create asset map
        let assets = Map::new(env.clone());
        
        // Define game resources
        let resources = vec![
            &env,
            Symbol::new(&env, "IRON"),
            Symbol::new(&env, "WATER"),
            Symbol::new(&env, "ENERGY")
        ];
        
        // Store assets
        for resource in resources.iter() {
            assets.set(resource.clone(), &issuer);
        }
        
        Symbol::new(&env, "ASSETS_CREATED")
    }

    // Create a new asset
    pub fn create_asset(env: Env, issuer: Address, code: Symbol) -> Symbol {
        // Verify issuer
        if !env.storage().has(&issuer) {
            return Symbol::new(&env, "INVALID_ISSUER");
        }
        
        // Create asset
        env.storage().set(&code, &issuer);
        
        Symbol::new(&env, "ASSET_CREATED")
    }

    // Get asset issuer
    pub fn get_asset_issuer(env: Env, code: Symbol) -> Address {
        env.storage().get::<_, Address>(&code).unwrap()
    }
}
```

## Step 2: Implementing Ship System

Create `src/ships.rs`:

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec, Map};

#[derive(Clone)]
enum ShipType {
    Scout,
    Explorer,
    Cargo
}

#[contract]
pub struct ShipSystem;

#[contractimpl]
impl ShipSystem {
    // Create a new ship
    pub fn create_ship(
        env: Env,
        player: Address,
        ship_type: Symbol,
        resources: Map<Symbol, i128>
    ) -> Symbol {
        // Verify player exists
        if !env.storage().has(&player) {
            return Symbol::new(&env, "PLAYER_NOT_FOUND");
        }

        // Create ship data
        let ship_data = Map::new(env.clone());
        ship_data.set(Symbol::new(&env, "type"), &ship_type);
        ship_data.set(Symbol::new(&env, "resources"), &resources);
        
        // Store ship
        let ships = env.storage().get::<_, Vec<Map<Symbol, Symbol>>>(&player)
            .unwrap_or_else(|| Vec::new(env.clone()));
        ships.push_back(ship_data);
        env.storage().set(&player, &ships);
        
        Symbol::new(&env, "SHIP_CREATED")
    }

    // Get player's ships
    pub fn get_player_ships(env: Env, player: Address) -> Vec<Map<Symbol, Symbol>> {
        env.storage().get::<_, Vec<Map<Symbol, Symbol>>>(&player)
            .unwrap_or_else(|| Vec::new(env.clone()))
    }

    // Transfer ship
    pub fn transfer_ship(
        env: Env,
        from: Address,
        to: Address,
        ship_index: i128
    ) -> Symbol {
        // Get ships
        let mut ships = env.storage().get::<_, Vec<Map<Symbol, Symbol>>>(&from)
            .unwrap_or_else(|| Vec::new(env.clone()));
        
        // Verify ship exists
        if ship_index < 0 || ship_index >= ships.len() as i128 {
            return Symbol::new(&env, "INVALID_SHIP");
        }
        
        // Transfer ship
        let ship = ships.remove(ship_index as u32);
        let mut to_ships = env.storage().get::<_, Vec<Map<Symbol, Symbol>>>(&to)
            .unwrap_or_else(|| Vec::new(env.clone()));
        to_ships.push_back(ship);
        
        // Update storage
        env.storage().set(&from, &ships);
        env.storage().set(&to, &to_ships);
        
        Symbol::new(&env, "SHIP_TRANSFERRED")
    }
}
```

## Step 3: Testing the System

Create `tests/game_test.rs`:

```rust
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{vec, Env};

    #[test]
    fn test_create_ship() {
        let env = Env::default();
        let player = Address::generate(&env);
        let ship_type = Symbol::new(&env, "SCOUT");
        
        // Create resources map
        let resources = Map::new(env.clone());
        resources.set(Symbol::new(&env, "IRON"), &100);
        resources.set(Symbol::new(&env, "WATER"), &100);
        resources.set(Symbol::new(&env, "ENERGY"), &100);
        
        // Create ship
        let result = ShipSystem::create_ship(&env, &player, &ship_type, &resources);
        assert_eq!(result, Symbol::new(&env, "SHIP_CREATED"));
        
        // Verify ship exists
        let ships = ShipSystem::get_player_ships(&env, &player);
        assert_eq!(ships.len(), 1);
    }
}
```

## Understanding the Code

1. **Asset Management**:
   - We use Stellar's native asset system
   - Assets are created and managed through smart contracts
   - Each asset has an issuer and can be tracked on-chain

2. **Ship System**:
   - Ships are represented as data structures in the smart contract
   - Each ship has a type and resource requirements
   - Ships can be transferred between players

3. **Storage Management**:
   - We use Stellar's native storage system
   - Data is stored in a structured format using Maps and Vectors
   - All operations are deterministic and verifiable

## Practice Exercise

Try to:
1. Add ship upgrade functionality that requires specific resources
2. Implement a resource collection system
3. Create a ship repair system that consumes resources

## Next Steps

In the next lesson, we'll:
1. Implement the star system exploration mechanics
2. Create trading functionality using Stellar's DEX
3. Add mission system using smart contracts

Remember: Always test your smart contracts thoroughly before deploying to mainnet! 
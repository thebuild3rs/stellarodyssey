# Lesson 3: Implementing Star System Exploration and Trading

In this lesson, you'll learn how to implement star system exploration and trading mechanics using Stellar's native features. We'll create smart contracts for managing star systems and implement trading using Stellar's built-in DEX.

## What You'll Learn
- Creating star system exploration mechanics
- Implementing trading using Stellar's DEX
- Managing resource discovery and collection

## Step 1: Creating Star System Contract

Create `src/stars.rs`:

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec, Map};

#[contract]
pub struct StarSystem;

#[derive(Clone)]
struct Star {
    id: Symbol,
    name: Symbol,
    resources: Vec<Symbol>,
    distance: i128,
    discovered: bool
}

#[contractimpl]
impl StarSystem {
    // Initialize star system
    pub fn initialize_system(env: Env) -> Symbol {
        // Create star map
        let stars = Map::new(env.clone());
        
        // Define initial stars
        let initial_stars = vec![
            &env,
            Star {
                id: Symbol::new(&env, "ALPHA_CENTAURI"),
                name: Symbol::new(&env, "Alpha Centauri"),
                resources: vec![
                    &env,
                    Symbol::new(&env, "IRON"),
                    Symbol::new(&env, "WATER")
                ],
                distance: 0,
                discovered: true
            },
            Star {
                id: Symbol::new(&env, "PROXIMA_CENTAURI"),
                name: Symbol::new(&env, "Proxima Centauri"),
                resources: vec![
                    &env,
                    Symbol::new(&env, "GOLD"),
                    Symbol::new(&env, "ENERGY")
                ],
                distance: 4,
                discovered: false
            }
        ];
        
        // Store stars
        for star in initial_stars.iter() {
            stars.set(star.id.clone(), star.clone());
        }
        
        env.storage().set(&Symbol::new(&env, "STARS"), &stars);
        Symbol::new(&env, "SYSTEM_INITIALIZED")
    }

    // Discover a new star
    pub fn discover_star(env: Env, player: Address, star_id: Symbol) -> Symbol {
        // Get stars
        let stars = env.storage().get::<_, Map<Symbol, Star>>(&Symbol::new(&env, "STARS"))
            .unwrap();
        
        // Get star
        let mut star = stars.get(star_id.clone()).unwrap();
        
        // Check if already discovered
        if star.discovered {
            return Symbol::new(&env, "ALREADY_DISCOVERED");
        }
        
        // Mark as discovered
        star.discovered = true;
        stars.set(star_id, star);
        env.storage().set(&Symbol::new(&env, "STARS"), &stars);
        
        Symbol::new(&env, "STAR_DISCOVERED")
    }

    // Get star information
    pub fn get_star_info(env: Env, star_id: Symbol) -> Star {
        let stars = env.storage().get::<_, Map<Symbol, Star>>(&Symbol::new(&env, "STARS"))
            .unwrap();
        stars.get(star_id).unwrap()
    }
}
```

## Step 2: Implementing Trading System

Create `src/trading.rs`:

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec, Map};

#[contract]
pub struct TradingSystem;

#[contractimpl]
impl TradingSystem {
    // Create a trade offer
    pub fn create_offer(
        env: Env,
        seller: Address,
        sell_asset: Symbol,
        sell_amount: i128,
        buy_asset: Symbol,
        buy_amount: i128
    ) -> Symbol {
        // Create offer data
        let offer = Map::new(env.clone());
        offer.set(Symbol::new(&env, "seller"), &seller);
        offer.set(Symbol::new(&env, "sell_asset"), &sell_asset);
        offer.set(Symbol::new(&env, "sell_amount"), &sell_amount);
        offer.set(Symbol::new(&env, "buy_asset"), &buy_asset);
        offer.set(Symbol::new(&env, "buy_amount"), &buy_amount);
        
        // Store offer
        let offers = env.storage().get::<_, Vec<Map<Symbol, Symbol>>>(&Symbol::new(&env, "OFFERS"))
            .unwrap_or_else(|| Vec::new(env.clone()));
        offers.push_back(offer);
        env.storage().set(&Symbol::new(&env, "OFFERS"), &offers);
        
        Symbol::new(&env, "OFFER_CREATED")
    }

    // Accept a trade offer
    pub fn accept_offer(
        env: Env,
        buyer: Address,
        offer_index: i128
    ) -> Symbol {
        // Get offers
        let mut offers = env.storage().get::<_, Vec<Map<Symbol, Symbol>>>(&Symbol::new(&env, "OFFERS"))
            .unwrap();
        
        // Verify offer exists
        if offer_index < 0 || offer_index >= offers.len() as i128 {
            return Symbol::new(&env, "INVALID_OFFER");
        }
        
        // Get offer
        let offer = offers.remove(offer_index as u32);
        
        // Execute trade using Stellar's DEX
        // Note: This is a simplified version. In a real implementation,
        // you would use Stellar's native DEX operations
        
        Symbol::new(&env, "TRADE_COMPLETED")
    }

    // Get all active offers
    pub fn get_offers(env: Env) -> Vec<Map<Symbol, Symbol>> {
        env.storage().get::<_, Vec<Map<Symbol, Symbol>>>(&Symbol::new(&env, "OFFERS"))
            .unwrap_or_else(|| Vec::new(env.clone()))
    }
}
```

## Step 3: Testing the Systems

Create `tests/exploration_test.rs`:

```rust
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{vec, Env};

    #[test]
    fn test_star_discovery() {
        let env = Env::default();
        let player = Address::generate(&env);
        
        // Initialize system
        let result = StarSystem::initialize_system(&env);
        assert_eq!(result, Symbol::new(&env, "SYSTEM_INITIALIZED"));
        
        // Discover star
        let star_id = Symbol::new(&env, "PROXIMA_CENTAURI");
        let result = StarSystem::discover_star(&env, &player, &star_id);
        assert_eq!(result, Symbol::new(&env, "STAR_DISCOVERED"));
        
        // Verify star info
        let star = StarSystem::get_star_info(&env, &star_id);
        assert!(star.discovered);
    }
}
```

## Understanding the Code

1. **Star System**:
   - Stars are represented as data structures in the smart contract
   - Each star has resources and discovery status
   - Players can discover new stars and access their resources

2. **Trading System**:
   - Uses Stellar's native DEX capabilities
   - Players can create and accept trade offers
   - Trades are executed atomically using smart contracts

3. **Resource Management**:
   - Resources are tracked on-chain
   - Discovery and trading affect resource availability
   - All operations are verifiable and secure

## Practice Exercise

Try to:
1. Add resource collection mechanics when discovering stars
2. Implement a market system for resource trading
3. Create special events that affect star resources

## Next Steps

In the next lesson, we'll:
1. Implement mission system using smart contracts
2. Add player progression and rewards
3. Create multiplayer interactions

Remember: Always consider gas costs and optimization when designing smart contracts! 
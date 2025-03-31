# Building a Space Game on Stellar

Welcome to the Stellar Space Explorer tutorial! In this series, you'll learn how to build a space exploration game using Stellar's native smart contract capabilities. We'll use Stellar's built-in features and smart contracts to create game mechanics.

## Lesson 1: Understanding Stellar Smart Contracts

### Prerequisites
- Basic understanding of blockchain concepts
- Familiarity with Stellar's basic concepts (accounts, assets, operations)
- A Stellar testnet account

### What You'll Learn
- Stellar's native smart contract capabilities
- How to create and deploy smart contracts on Stellar
- Basic game mechanics using Stellar's built-in features

### Step 1: Setting Up Your Development Environment

First, install the Stellar Development Environment:

```bash
# Install Stellar CLI
curl -s https://get.stellar.org | bash

# Install Stellar Smart Contract tools
curl -s https://get.stellar.org/scc | bash
```

### Step 2: Creating Your First Smart Contract

Create a new directory for your game contract:

```bash
mkdir stellar-space-game
cd stellar-space-game
scc init space-game
```

This will create a new smart contract project with the following structure:
```
space-game/
├── src/
│   └── contract.rs
├── tests/
│   └── contract_test.rs
└── Cargo.toml
```

### Step 3: Writing Your First Game Contract

Edit `src/contract.rs`:

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct SpaceGame;

#[contractimpl]
impl SpaceGame {
    // Initialize a new player
    pub fn initialize_player(env: Env, player: Address) -> Symbol {
        // Create player data structure
        let player_data = Symbol::new(&env, "PLAYER");
        
        // Store player data
        env.storage().set(&player, &player_data);
        
        Symbol::new(&env, "SUCCESS")
    }

    // Create a new ship
    pub fn create_ship(env: Env, player: Address, ship_type: Symbol) -> Symbol {
        // Verify player exists
        if !env.storage().has(&player) {
            return Symbol::new(&env, "PLAYER_NOT_FOUND");
        }

        // Create ship data
        let ship_data = Symbol::new(&env, "SHIP");
        
        // Store ship data
        env.storage().set(&player, &ship_data);
        
        Symbol::new(&env, "SHIP_CREATED")
    }

    // Get player's ships
    pub fn get_player_ships(env: Env, player: Address) -> Symbol {
        if !env.storage().has(&player) {
            return Symbol::new(&env, "PLAYER_NOT_FOUND");
        }

        // Return player's ships
        Symbol::new(&env, "SHIPS")
    }
}
```

### Step 4: Understanding the Code

Let's break down what this code does:

1. We use `soroban-sdk`, Stellar's official smart contract SDK
2. The contract defines basic game operations:
   - `initialize_player`: Creates a new player account
   - `create_ship`: Creates a new ship for a player
   - `get_player_ships`: Retrieves a player's ships

3. We use Stellar's native storage system to persist data

### Step 5: Deploying Your Contract

Build and deploy your contract:

```bash
# Build the contract
scc build

# Deploy to testnet
scc deploy --network testnet
```

### Step 6: Testing Your Contract

Create a test file `tests/contract_test.rs`:

```rust
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{vec, Env};

    #[test]
    fn test_create_player() {
        let env = Env::default();
        let player = Address::generate(&env);
        
        let result = SpaceGame::initialize_player(&env, &player);
        assert_eq!(result, Symbol::new(&env, "SUCCESS"));
    }
}
```

Run the tests:

```bash
scc test
```

## Understanding Stellar's Smart Contract Features

1. **Native Storage**:
   - Stellar provides built-in storage for smart contracts
   - Data is stored on-chain and accessible to all contract operations

2. **Account Management**:
   - Stellar handles account creation and management
   - Smart contracts can interact with accounts through the Address type

3. **Asset Management**:
   - Stellar's native asset system can be used for game resources
   - Smart contracts can create and manage custom assets

## Practice Exercise

Try to:
1. Add a function to transfer ships between players
2. Implement a resource collection system using Stellar assets
3. Create a ship upgrade system that requires specific resources

## Next Steps

In the next lesson, we'll:
1. Implement the star system exploration mechanics
2. Create trading functionality using Stellar's built-in DEX
3. Add mission system using Stellar's native features

Remember: Stellar smart contracts are deterministic and must be carefully designed to handle all possible states! 
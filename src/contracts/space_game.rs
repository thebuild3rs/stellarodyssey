#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, Symbol, Vec, Map,
};

#[contracttype]
pub enum DataKey {
    Player(Address),
    Ship(Address, u32),
    Resource(Symbol),
}

#[contracttype]
pub struct Player {
    ships: Vec<Ship>,
    resources: Vec<Resource>,
    initialized: bool,
}

#[contracttype]
pub struct Ship {
    name: Symbol,
    ship_type: Symbol,
    resources: Vec<Resource>,
    active: bool,
}

#[contracttype]
pub struct Resource {
    name: Symbol,
    amount: i128,
    issuer: Address,
}

#[contract]
pub struct SpaceGame;

#[contractimpl]
impl SpaceGame {
    // Initialize a new player
    pub fn initialize_player(env: Env, player: Address) -> Symbol {
        // Check if player already exists
        if let Some(_) = env.storage().get(&DataKey::Player(player.clone())) {
            return Symbol::new(&env, "PLAYER_EXISTS");
        }

        // Create new player
        let player_data = Player {
            ships: Vec::new(&env),
            resources: Vec::new(&env),
            initialized: true,
        };

        // Store player data
        env.storage().set(&DataKey::Player(player), &player_data);

        Symbol::new(&env, "PLAYER_INITIALIZED")
    }

    // Create a new ship
    pub fn create_ship(env: Env, player: Address, name: Symbol, ship_type: Symbol) -> Symbol {
        // Get player data
        let mut player_data: Player = env.storage()
            .get(&DataKey::Player(player.clone()))
            .ok_or_else(|| Symbol::new(&env, "PLAYER_NOT_FOUND"))?;

        // Create new ship
        let ship = Ship {
            name,
            ship_type,
            resources: Vec::new(&env),
            active: true,
        };

        // Add ship to player's ships
        player_data.ships.push_back(ship);
        env.storage().set(&DataKey::Player(player), &player_data);

        Symbol::new(&env, "SHIP_CREATED")
    }

    // Collect resources
    pub fn collect_resources(
        env: Env,
        player: Address,
        resource_name: Symbol,
        amount: i128
    ) -> Symbol {
        // Get player data
        let mut player_data: Player = env.storage()
            .get(&DataKey::Player(player.clone()))
            .ok_or_else(|| Symbol::new(&env, "PLAYER_NOT_FOUND"))?;

        // Create resource
        let resource = Resource {
            name: resource_name,
            amount,
            issuer: env.current_contract_address(),
        };

        // Add resource to player's resources
        player_data.resources.push_back(resource);
        env.storage().set(&DataKey::Player(player), &player_data);

        Symbol::new(&env, "RESOURCE_COLLECTED")
    }

    // Get player's ships
    pub fn get_player_ships(env: Env, player: Address) -> Vec<Ship> {
        let player_data: Player = env.storage()
            .get(&DataKey::Player(player))
            .ok_or_else(|| Vec::new(&env))?;

        player_data.ships
    }

    // Get player's resources
    pub fn get_player_resources(env: Env, player: Address) -> Vec<Resource> {
        let player_data: Player = env.storage()
            .get(&DataKey::Player(player))
            .ok_or_else(|| Vec::new(&env))?;

        player_data.resources
    }
} 
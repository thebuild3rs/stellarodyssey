use soroban_sdk::{
    contract, contractimpl, symbol_short,
    Address, Env, Symbol, Vec, Map, U256,
};

#[derive(Clone)]
enum DataKey {
    Ship(u64),
    PlayerShips(Address),
    ShipCounter,
    UpgradeCosts,
    RepairCosts,
}

#[derive(Clone)]
struct Ship {
    id: u64,
    owner: Address,
    name: Symbol,
    type_: Symbol,
    health: u64,
    max_health: u64,
    cargo_capacity: u64,
    level: u64,
    resources: Map<Symbol, u64>,
}

#[contract]
pub struct ShipyardContract;

#[contractimpl]
impl ShipyardContract {
    // Ship Management
    pub fn create_ship(
        env: Env,
        owner: Address,
        name: Symbol,
        type_: Symbol,
    ) -> u64 {
        let counter = Self::get_ship_counter(&env);
        let new_counter = counter + 1;

        let ship = Ship {
            id: new_counter,
            owner: owner.clone(),
            name,
            type_,
            health: 100,
            max_health: 100,
            cargo_capacity: 1000,
            level: 1,
            resources: Map::new(env),
        };

        env.storage().set(&DataKey::Ship(new_counter), &ship);
        env.storage().set(&DataKey::ShipCounter, &new_counter);

        // Add ship to player's ships
        let mut player_ships = Self::get_player_ships(&env, owner);
        player_ships.push_back(new_counter);
        env.storage().set(&DataKey::PlayerShips(owner), &player_ships);

        // Emit event
        env.events().publish(
            (symbol_short!("SHIP"), symbol_short!("CREATED")),
            (owner, new_counter),
        );

        new_counter
    }

    pub fn get_ship(env: Env, id: u64) -> Ship {
        env.storage()
            .get::<_, Ship>(&DataKey::Ship(id))
            .unwrap()
    }

    pub fn get_player_ships(env: Env, player: Address) -> Vec<u64> {
        env.storage()
            .get::<_, Vec<u64>>(&DataKey::PlayerShips(player))
            .unwrap_or(Vec::new(env))
    }

    // Ship Upgrades
    pub fn upgrade_ship(env: Env, player: Address, ship_id: u64) -> bool {
        let mut ship = Self::get_ship(&env, ship_id);
        
        if ship.owner != player {
            return false;
        }

        let upgrade_cost = Self::calculate_upgrade_cost(&env, ship.level);
        
        // Check if player has enough resources
        // This would involve checking the player's resource balance
        // through cross-contract calls to the resource management contract

        ship.level += 1;
        ship.max_health += 20;
        ship.cargo_capacity += 200;
        ship.health = ship.max_health;

        env.storage().set(&DataKey::Ship(ship_id), &ship);

        // Emit event
        env.events().publish(
            (symbol_short!("SHIP"), symbol_short!("UPGRADED")),
            (player, ship_id, ship.level),
        );

        true
    }

    // Ship Repairs
    pub fn repair_ship(env: Env, player: Address, ship_id: u64) -> bool {
        let mut ship = Self::get_ship(&env, ship_id);
        
        if ship.owner != player {
            return false;
        }

        if ship.health >= ship.max_health {
            return false;
        }

        let repair_cost = Self::calculate_repair_cost(&env, ship.max_health - ship.health);
        
        // Check if player has enough resources
        // This would involve checking the player's resource balance
        // through cross-contract calls to the resource management contract

        ship.health = ship.max_health;
        env.storage().set(&DataKey::Ship(ship_id), &ship);

        // Emit event
        env.events().publish(
            (symbol_short!("SHIP"), symbol_short!("REPAIRED")),
            (player, ship_id),
        );

        true
    }

    // Cost Calculations
    fn calculate_upgrade_cost(env: &Env, current_level: u64) -> u64 {
        // Base cost * (level ^ 1.5)
        let base_cost = 1000;
        let level_factor = (current_level as f64).powf(1.5) as u64;
        base_cost * level_factor
    }

    fn calculate_repair_cost(env: &Env, damage: u64) -> u64 {
        // Cost per health point * damage
        let cost_per_health = 10;
        cost_per_health * damage
    }

    // Helper functions
    fn get_ship_counter(env: &Env) -> u64 {
        env.storage()
            .get::<_, u64>(&DataKey::ShipCounter)
            .unwrap_or(0)
    }
} 
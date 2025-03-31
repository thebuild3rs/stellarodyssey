#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, Symbol, Vec, Map,
};

#[contracttype]
pub enum DataKey {
    Star(Symbol),
    StarList,
    ResourceAmount(Symbol, Symbol), // (star_name, resource_name)
}

#[contracttype]
pub struct Star {
    name: Symbol,
    resources: Vec<Symbol>,
    distance: i128,
    discovered: bool,
}

#[contract]
pub struct StarSystem;

#[contractimpl]
impl StarSystem {
    // Initialize star system
    pub fn initialize_system(env: Env) -> Symbol {
        // Create initial stars
        Self::create_star(&env, Symbol::new(&env, "ALPHA_CENTAURI"), 0);
        Self::create_star(&env, Symbol::new(&env, "PROXIMA_CENTAURI"), 4);
        Self::create_star(&env, Symbol::new(&env, "BARNARDS_STAR"), 6);

        Symbol::new(&env, "SYSTEM_INITIALIZED")
    }

    // Create a new star
    fn create_star(env: &Env, name: Symbol, distance: i128) -> Symbol {
        // Check if star already exists
        if let Some(_) = env.storage().get(&DataKey::Star(name.clone())) {
            return Symbol::new(&env, "STAR_EXISTS");
        }

        // Create star
        let star = Star {
            name: name.clone(),
            resources: Vec::new(&env),
            distance,
            discovered: false,
        };

        // Store star
        env.storage().set(&DataKey::Star(name.clone()), &star);

        // Add to star list
        let mut star_list: Vec<Symbol> = env.storage()
            .get(&DataKey::StarList)
            .unwrap_or_else(|| Vec::new(&env));
        star_list.push_back(name);
        env.storage().set(&DataKey::StarList, &star_list);

        Symbol::new(&env, "STAR_CREATED")
    }

    // Discover a star
    pub fn discover_star(env: Env, player: Address, star_name: Symbol) -> Symbol {
        // Get star data
        let mut star: Star = env.storage()
            .get(&DataKey::Star(star_name.clone()))
            .ok_or_else(|| Symbol::new(&env, "STAR_NOT_FOUND"))?;

        // Check if already discovered
        if star.discovered {
            return Symbol::new(&env, "ALREADY_DISCOVERED");
        }

        // Mark as discovered
        star.discovered = true;
        env.storage().set(&DataKey::Star(star_name), &star);

        Symbol::new(&env, "STAR_DISCOVERED")
    }

    // Add resource to a star
    pub fn add_resource(env: Env, star_name: Symbol, resource_name: Symbol, amount: i128) -> Symbol {
        // Get star data
        let mut star: Star = env.storage()
            .get(&DataKey::Star(star_name.clone()))
            .ok_or_else(|| Symbol::new(&env, "STAR_NOT_FOUND"))?;

        // Add resource
        star.resources.push_back(resource_name.clone());
        env.storage().set(&DataKey::Star(star_name.clone()), &star);

        // Set resource amount
        env.storage().set(
            &DataKey::ResourceAmount(star_name, resource_name),
            &amount
        );

        Symbol::new(&env, "RESOURCE_ADDED")
    }

    // Get star information
    pub fn get_star_info(env: Env, star_name: Symbol) -> Star {
        env.storage()
            .get(&DataKey::Star(star_name))
            .ok_or_else(|| panic!("Star not found"))
    }

    // Get resource amount
    pub fn get_resource_amount(env: Env, star_name: Symbol, resource_name: Symbol) -> i128 {
        env.storage()
            .get(&DataKey::ResourceAmount(star_name, resource_name))
            .unwrap_or(0)
    }

    // Get all stars
    pub fn get_all_stars(env: Env) -> Vec<Symbol> {
        env.storage()
            .get(&DataKey::StarList)
            .unwrap_or_else(|| Vec::new(&env))
    }
} 
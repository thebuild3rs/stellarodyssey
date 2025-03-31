#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, Symbol, Vec, Map,
};

#[contracttype]
pub enum DataKey {
    Mission(Symbol),
    PlayerMissions(Address),
    MissionList,
}

#[contracttype]
pub struct Mission {
    id: Symbol,
    name: Symbol,
    description: Symbol,
    reward_resource: Symbol,
    reward_amount: i128,
    required_stars: Vec<Symbol>,
    required_resources: Map<Symbol, i128>,
    completed: bool,
}

#[contract]
pub struct MissionSystem;

#[contractimpl]
impl MissionSystem {
    // Initialize mission system
    pub fn initialize_missions(env: Env) -> Symbol {
        // Create initial missions
        Self::create_mission(
            &env,
            Symbol::new(&env, "FIRST_STEPS"),
            Symbol::new(&env, "First Steps"),
            Symbol::new(&env, "Discover your first star system"),
            Symbol::new(&env, "ENERGY"),
            100,
            Vec::new(&env),
            Map::new(&env)
        );

        Self::create_mission(
            &env,
            Symbol::new(&env, "RESOURCE_COLLECTOR"),
            Symbol::new(&env, "Resource Collector"),
            Symbol::new(&env, "Collect 1000 units of resources"),
            Symbol::new(&env, "IRON"),
            500,
            Vec::new(&env),
            {
                let mut requirements = Map::new(&env);
                requirements.set(Symbol::new(&env, "WATER"), 1000);
                requirements
            }
        );

        Symbol::new(&env, "MISSIONS_INITIALIZED")
    }

    // Create a new mission
    fn create_mission(
        env: &Env,
        id: Symbol,
        name: Symbol,
        description: Symbol,
        reward_resource: Symbol,
        reward_amount: i128,
        required_stars: Vec<Symbol>,
        required_resources: Map<Symbol, i128>,
    ) -> Symbol {
        let mission = Mission {
            id: id.clone(),
            name,
            description,
            reward_resource,
            reward_amount,
            required_stars,
            required_resources,
            completed: false,
        };

        env.storage().set(&DataKey::Mission(id.clone()), &mission);

        // Add to mission list
        let mut mission_list: Vec<Symbol> = env.storage()
            .get(&DataKey::MissionList)
            .unwrap_or_else(|| Vec::new(&env));
        mission_list.push_back(id);
        env.storage().set(&DataKey::MissionList, &mission_list);

        Symbol::new(&env, "MISSION_CREATED")
    }

    // Check mission completion
    pub fn check_mission_completion(
        env: Env,
        player: Address,
        mission_id: Symbol,
        player_resources: Map<Symbol, i128>,
        discovered_stars: Vec<Symbol>
    ) -> Symbol {
        // Get mission
        let mut mission: Mission = env.storage()
            .get(&DataKey::Mission(mission_id.clone()))
            .ok_or_else(|| Symbol::new(&env, "MISSION_NOT_FOUND"))?;

        if mission.completed {
            return Symbol::new(&env, "ALREADY_COMPLETED");
        }

        // Check star requirements
        for required_star in mission.required_stars.iter() {
            if !discovered_stars.contains(&required_star) {
                return Symbol::new(&env, "REQUIREMENTS_NOT_MET");
            }
        }

        // Check resource requirements
        for (resource, amount) in mission.required_resources.iter() {
            let player_amount = player_resources.get(resource).unwrap_or(0);
            if player_amount < amount {
                return Symbol::new(&env, "REQUIREMENTS_NOT_MET");
            }
        }

        // Mark mission as completed
        mission.completed = true;
        env.storage().set(&DataKey::Mission(mission_id), &mission);

        // Add to player's completed missions
        let mut player_missions: Vec<Symbol> = env.storage()
            .get(&DataKey::PlayerMissions(player.clone()))
            .unwrap_or_else(|| Vec::new(&env));
        player_missions.push_back(mission_id);
        env.storage().set(&DataKey::PlayerMissions(player), &player_missions);

        Symbol::new(&env, "MISSION_COMPLETED")
    }

    // Get mission details
    pub fn get_mission_details(env: Env, mission_id: Symbol) -> Option<Mission> {
        env.storage().get(&DataKey::Mission(mission_id))
    }

    // Get player's completed missions
    pub fn get_player_missions(env: Env, player: Address) -> Vec<Symbol> {
        env.storage()
            .get(&DataKey::PlayerMissions(player))
            .unwrap_or_else(|| Vec::new(&env))
    }

    // Get all available missions
    pub fn get_all_missions(env: Env) -> Vec<Symbol> {
        env.storage()
            .get(&DataKey::MissionList)
            .unwrap_or_else(|| Vec::new(&env))
    }
} 
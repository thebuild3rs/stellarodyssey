use soroban_sdk::{
    contract, contractimpl, symbol_short,
    Address, Env, Symbol, Vec, Map, U256,
};

#[derive(Clone)]
enum DataKey {
    MissionCounter,
    Mission(u64),
    PlayerMissions(Address),
    AchievementCounter,
    Achievement(u64),
    PlayerAchievements(Address),
}

#[derive(Clone)]
struct Mission {
    id: u64,
    name: Symbol,
    description: Symbol,
    reward: Reward,
    requirements: Requirements,
    completed: bool,
}

#[derive(Clone)]
struct Reward {
    resource: Symbol,
    amount: u64,
}

#[derive(Clone)]
struct Requirements {
    stars: u64,
    resources: Map<Symbol, u64>,
}

#[derive(Clone)]
struct Achievement {
    id: u64,
    name: Symbol,
    description: Symbol,
    reward: Reward,
    completed: bool,
}

#[contract]
pub struct MissionContract;

#[contractimpl]
impl MissionContract {
    // Mission Management
    pub fn create_mission(
        env: Env,
        name: Symbol,
        description: Symbol,
        reward: Reward,
        requirements: Requirements,
    ) -> u64 {
        let counter = Self::get_mission_counter(&env);
        let new_counter = counter + 1;
        
        let mission = Mission {
            id: new_counter,
            name,
            description,
            reward,
            requirements,
            completed: false,
        };

        env.storage().set(&DataKey::Mission(new_counter), &mission);
        env.storage().set(&DataKey::MissionCounter, &new_counter);

        new_counter
    }

    pub fn get_mission(env: Env, id: u64) -> Mission {
        env.storage()
            .get::<_, Mission>(&DataKey::Mission(id))
            .unwrap()
    }

    pub fn get_player_missions(env: Env, player: Address) -> Vec<u64> {
        env.storage()
            .get::<_, Vec<u64>>(&DataKey::PlayerMissions(player))
            .unwrap_or(Vec::new(env))
    }

    pub fn complete_mission(env: Env, player: Address, mission_id: u64) -> bool {
        let mut mission = Self::get_mission(&env, mission_id);
        
        if mission.completed {
            return false;
        }

        // Check if player meets requirements
        // This would involve checking star discovery and resource collection
        // through cross-contract calls to the star system contract

        mission.completed = true;
        env.storage().set(&DataKey::Mission(mission_id), &mission);

        // Add mission to player's completed missions
        let mut player_missions = Self::get_player_missions(&env, player.clone());
        player_missions.push_back(mission_id);
        env.storage().set(&DataKey::PlayerMissions(player), &player_missions);

        // Emit event
        env.events().publish(
            (symbol_short!("MISSION"), symbol_short!("COMPLETED")),
            (player, mission_id),
        );

        true
    }

    // Achievement Management
    pub fn create_achievement(
        env: Env,
        name: Symbol,
        description: Symbol,
        reward: Reward,
    ) -> u64 {
        let counter = Self::get_achievement_counter(&env);
        let new_counter = counter + 1;
        
        let achievement = Achievement {
            id: new_counter,
            name,
            description,
            reward,
            completed: false,
        };

        env.storage().set(&DataKey::Achievement(new_counter), &achievement);
        env.storage().set(&DataKey::AchievementCounter, &new_counter);

        new_counter
    }

    pub fn get_achievement(env: Env, id: u64) -> Achievement {
        env.storage()
            .get::<_, Achievement>(&DataKey::Achievement(id))
            .unwrap()
    }

    pub fn get_player_achievements(env: Env, player: Address) -> Vec<u64> {
        env.storage()
            .get::<_, Vec<u64>>(&DataKey::PlayerAchievements(player))
            .unwrap_or(Vec::new(env))
    }

    pub fn complete_achievement(env: Env, player: Address, achievement_id: u64) -> bool {
        let mut achievement = Self::get_achievement(&env, achievement_id);
        
        if achievement.completed {
            return false;
        }

        achievement.completed = true;
        env.storage().set(&DataKey::Achievement(achievement_id), &achievement);

        // Add achievement to player's completed achievements
        let mut player_achievements = Self::get_player_achievements(&env, player.clone());
        player_achievements.push_back(achievement_id);
        env.storage().set(&DataKey::PlayerAchievements(player), &player_achievements);

        // Emit event
        env.events().publish(
            (symbol_short!("ACHIEVEMENT"), symbol_short!("COMPLETED")),
            (player, achievement_id),
        );

        true
    }

    // Helper functions
    fn get_mission_counter(env: &Env) -> u64 {
        env.storage()
            .get::<_, u64>(&DataKey::MissionCounter)
            .unwrap_or(0)
    }

    fn get_achievement_counter(env: &Env) -> u64 {
        env.storage()
            .get::<_, u64>(&DataKey::AchievementCounter)
            .unwrap_or(0)
    }
} 
use soroban_sdk::{
    contract, contractimpl, symbol_short,
    Address, Env, Symbol, Vec, Map, U256,
};

#[derive(Clone)]
enum DataKey {
    ResourcePrice(Symbol),
    PlayerResources(Address),
    ResourceMarket,
    TransactionHistory(Address),
}

#[derive(Clone)]
struct ResourcePrice {
    base_price: u64,
    volatility: u64,
    last_update: u64,
}

#[derive(Clone)]
struct Transaction {
    timestamp: u64,
    type_: Symbol,
    resource: Symbol,
    amount: u64,
    price: u64,
    counterparty: Address,
}

#[contract]
pub struct EconomyContract;

#[contractimpl]
impl EconomyContract {
    // Resource Management
    pub fn initialize_resource(env: Env, resource: Symbol, base_price: u64) {
        let price = ResourcePrice {
            base_price,
            volatility: 10, // 10% volatility
            last_update: env.ledger().timestamp(),
        };
        env.storage().set(&DataKey::ResourcePrice(resource), &price);
    }

    pub fn get_resource_price(env: Env, resource: Symbol) -> u64 {
        let price = env.storage()
            .get::<_, ResourcePrice>(&DataKey::ResourcePrice(resource))
            .unwrap();
        
        // Calculate current price based on time and volatility
        let time_passed = env.ledger().timestamp() - price.last_update;
        let volatility_factor = (time_passed as f64 / 3600.0).sin() * (price.volatility as f64 / 100.0);
        let current_price = (price.base_price as f64 * (1.0 + volatility_factor)) as u64;
        
        current_price
    }

    pub fn get_player_resources(env: Env, player: Address) -> Map<Symbol, u64> {
        env.storage()
            .get::<_, Map<Symbol, u64>>(&DataKey::PlayerResources(player))
            .unwrap_or(Map::new(env))
    }

    // Resource Transactions
    pub fn transfer_resources(
        env: Env,
        from: Address,
        to: Address,
        resource: Symbol,
        amount: u64,
    ) -> bool {
        let mut from_resources = Self::get_player_resources(&env, from.clone());
        let mut to_resources = Self::get_player_resources(&env, to.clone());

        if from_resources.get(resource.clone()).unwrap_or(0) < amount {
            return false;
        }

        from_resources.set(resource.clone(), from_resources.get(resource.clone()).unwrap() - amount);
        to_resources.set(resource.clone(), to_resources.get(resource.clone()).unwrap_or(0) + amount);

        env.storage().set(&DataKey::PlayerResources(from), &from_resources);
        env.storage().set(&DataKey::PlayerResources(to), &to_resources);

        // Record transaction
        let transaction = Transaction {
            timestamp: env.ledger().timestamp(),
            type_: symbol_short!("TRANSFER"),
            resource: resource.clone(),
            amount,
            price: 0,
            counterparty: to.clone(),
        };

        let mut from_history = Self::get_transaction_history(&env, from);
        from_history.push_back(transaction);
        env.storage().set(&DataKey::TransactionHistory(from), &from_history);

        // Emit event
        env.events().publish(
            (symbol_short!("RESOURCE"), symbol_short!("TRANSFERRED")),
            (from, to, resource, amount),
        );

        true
    }

    pub fn buy_resources(
        env: Env,
        buyer: Address,
        seller: Address,
        resource: Symbol,
        amount: u64,
    ) -> bool {
        let price = Self::get_resource_price(&env, resource.clone());
        let total_cost = price * amount;

        let mut buyer_resources = Self::get_player_resources(&env, buyer.clone());
        let mut seller_resources = Self::get_player_resources(&env, seller.clone());

        if seller_resources.get(resource.clone()).unwrap_or(0) < amount {
            return false;
        }

        // Transfer resources
        seller_resources.set(resource.clone(), seller_resources.get(resource.clone()).unwrap() - amount);
        buyer_resources.set(resource.clone(), buyer_resources.get(resource.clone()).unwrap_or(0) + amount);

        env.storage().set(&DataKey::PlayerResources(seller), &seller_resources);
        env.storage().set(&DataKey::PlayerResources(buyer), &buyer_resources);

        // Record transaction
        let transaction = Transaction {
            timestamp: env.ledger().timestamp(),
            type_: symbol_short!("BUY"),
            resource: resource.clone(),
            amount,
            price,
            counterparty: seller.clone(),
        };

        let mut buyer_history = Self::get_transaction_history(&env, buyer);
        buyer_history.push_back(transaction);
        env.storage().set(&DataKey::TransactionHistory(buyer), &buyer_history);

        // Emit event
        env.events().publish(
            (symbol_short!("RESOURCE"), symbol_short!("BOUGHT")),
            (buyer, seller, resource, amount, price),
        );

        true
    }

    pub fn sell_resources(
        env: Env,
        seller: Address,
        buyer: Address,
        resource: Symbol,
        amount: u64,
    ) -> bool {
        let price = Self::get_resource_price(&env, resource.clone());
        let total_revenue = price * amount;

        let mut seller_resources = Self::get_player_resources(&env, seller.clone());
        let mut buyer_resources = Self::get_player_resources(&env, buyer.clone());

        if seller_resources.get(resource.clone()).unwrap_or(0) < amount {
            return false;
        }

        // Transfer resources
        seller_resources.set(resource.clone(), seller_resources.get(resource.clone()).unwrap() - amount);
        buyer_resources.set(resource.clone(), buyer_resources.get(resource.clone()).unwrap_or(0) + amount);

        env.storage().set(&DataKey::PlayerResources(seller), &seller_resources);
        env.storage().set(&DataKey::PlayerResources(buyer), &buyer_resources);

        // Record transaction
        let transaction = Transaction {
            timestamp: env.ledger().timestamp(),
            type_: symbol_short!("SELL"),
            resource: resource.clone(),
            amount,
            price,
            counterparty: buyer.clone(),
        };

        let mut seller_history = Self::get_transaction_history(&env, seller);
        seller_history.push_back(transaction);
        env.storage().set(&DataKey::TransactionHistory(seller), &seller_history);

        // Emit event
        env.events().publish(
            (symbol_short!("RESOURCE"), symbol_short!("SOLD")),
            (seller, buyer, resource, amount, price),
        );

        true
    }

    // Transaction History
    pub fn get_transaction_history(env: Env, player: Address) -> Vec<Transaction> {
        env.storage()
            .get::<_, Vec<Transaction>>(&DataKey::TransactionHistory(player))
            .unwrap_or(Vec::new(env))
    }

    // Market Analysis
    pub fn get_market_trend(env: Env, resource: Symbol) -> i64 {
        // This would analyze recent transactions to determine market trend
        // Returns a value between -100 and 100 indicating bearish to bullish trend
        0 // Placeholder
    }
} 
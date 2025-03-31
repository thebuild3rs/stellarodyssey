#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, Symbol, Vec, Map,
};

#[contracttype]
pub enum DataKey {
    Offer(u32),
    OfferCounter,
    PlayerOffers(Address),
}

#[contracttype]
pub struct TradeOffer {
    id: u32,
    seller: Address,
    sell_resource: Symbol,
    sell_amount: i128,
    buy_resource: Symbol,
    buy_amount: i128,
    active: bool,
}

#[contract]
pub struct Trading;

#[contractimpl]
impl Trading {
    // Create a new trade offer
    pub fn create_offer(
        env: Env,
        seller: Address,
        sell_resource: Symbol,
        sell_amount: i128,
        buy_resource: Symbol,
        buy_amount: i128
    ) -> Symbol {
        // Validate amounts
        if sell_amount <= 0 || buy_amount <= 0 {
            return Symbol::new(&env, "INVALID_AMOUNTS");
        }

        // Get and increment offer counter
        let counter: u32 = env.storage()
            .get(&DataKey::OfferCounter)
            .unwrap_or(0);
        let offer_id = counter + 1;
        env.storage().set(&DataKey::OfferCounter, &offer_id);

        // Create offer
        let offer = TradeOffer {
            id: offer_id,
            seller: seller.clone(),
            sell_resource,
            sell_amount,
            buy_resource,
            buy_amount,
            active: true,
        };

        // Store offer
        env.storage().set(&DataKey::Offer(offer_id), &offer);

        // Add to player's offers
        let mut player_offers: Vec<u32> = env.storage()
            .get(&DataKey::PlayerOffers(seller.clone()))
            .unwrap_or_else(|| Vec::new(&env));
        player_offers.push_back(offer_id);
        env.storage().set(&DataKey::PlayerOffers(seller), &player_offers);

        Symbol::new(&env, "OFFER_CREATED")
    }

    // Accept a trade offer
    pub fn accept_offer(env: Env, buyer: Address, offer_id: u32) -> Symbol {
        // Get offer
        let mut offer: TradeOffer = env.storage()
            .get(&DataKey::Offer(offer_id))
            .ok_or_else(|| Symbol::new(&env, "OFFER_NOT_FOUND"))?;

        // Validate offer
        if !offer.active {
            return Symbol::new(&env, "OFFER_NOT_ACTIVE");
        }
        if offer.seller == buyer {
            return Symbol::new(&env, "CANNOT_ACCEPT_OWN_OFFER");
        }

        // Mark offer as inactive
        offer.active = false;
        env.storage().set(&DataKey::Offer(offer_id), &offer);

        // Here you would implement the actual resource transfer
        // This would involve calling the SpaceGame contract to transfer resources

        Symbol::new(&env, "TRADE_COMPLETED")
    }

    // Cancel a trade offer
    pub fn cancel_offer(env: Env, seller: Address, offer_id: u32) -> Symbol {
        // Get offer
        let mut offer: TradeOffer = env.storage()
            .get(&DataKey::Offer(offer_id))
            .ok_or_else(|| Symbol::new(&env, "OFFER_NOT_FOUND"))?;

        // Validate seller
        if offer.seller != seller {
            return Symbol::new(&env, "NOT_OFFER_CREATOR");
        }
        if !offer.active {
            return Symbol::new(&env, "OFFER_NOT_ACTIVE");
        }

        // Mark offer as inactive
        offer.active = false;
        env.storage().set(&DataKey::Offer(offer_id), &offer);

        Symbol::new(&env, "OFFER_CANCELLED")
    }

    // Get active offers
    pub fn get_active_offers(env: Env) -> Vec<TradeOffer> {
        let counter: u32 = env.storage()
            .get(&DataKey::OfferCounter)
            .unwrap_or(0);

        let mut active_offers = Vec::new(&env);
        for i in 1..=counter {
            if let Some(offer) = env.storage().get(&DataKey::Offer(i)) {
                let offer: TradeOffer = offer;
                if offer.active {
                    active_offers.push_back(offer);
                }
            }
        }

        active_offers
    }

    // Get player's offers
    pub fn get_player_offers(env: Env, player: Address) -> Vec<u32> {
        env.storage()
            .get(&DataKey::PlayerOffers(player))
            .unwrap_or_else(|| Vec::new(&env))
    }

    // Get offer details
    pub fn get_offer_details(env: Env, offer_id: u32) -> Option<TradeOffer> {
        env.storage().get(&DataKey::Offer(offer_id))
    }
} 
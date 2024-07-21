use near_sdk::{env, json_types::U128, near, store::LookupMap, AccountId};

#[near(contract_state)]
pub struct Contract {
    users_bets: LookupMap<AccountId, U128>,
}

#[near]
impl Contract {         
    #[init]
    #[private] 
    pub fn init() -> Self {
        Self {
            users_bets: LookupMap::new(b"m"),
        }
    }

    #[payable]
    pub fn place_bet(&mut self) {
        let sender_id = env::predecessor_account_id();
        let attached_deposit = env::attached_deposit().as_yoctonear(); 

        assert!(attached_deposit > 0, "Must attach some NEAR tokens to place a bet");

        if self.users_bets.get(&sender_id).is_some() {
            panic!("The user has already placed a bet")
        }

        let new_bet = U128(attached_deposit);
        self.users_bets.insert(sender_id, new_bet);
    }
}

#[cfg(test)]
mod tests { }

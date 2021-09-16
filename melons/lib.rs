#![cfg_attr(not(feature = "std"), no_std)]

pub use self::melons::Melon;
use ink_lang as ink;

#[ink::contract]
mod melons {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
    };

    #[ink(storage)]
    pub struct Melon {
        total_supply: Lazy<Balance>,
        balances: StorageHashMap<AccountId, Balance>,
    }

    impl Melon {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut initial_storage = StorageHashMap::new();
            initial_storage.insert(caller, initial_supply);

            Self {
                balances: initial_storage,
                total_supply: Lazy::new(initial_supply),
            }
        }

        #[ink(message)]
        pub fn withdraw(&mut self, account: AccountId, amount: Balance) -> Balance {
            let acc_balance = self.balance_of(account);
            if acc_balance < amount {
                ()
            }

            self.balances.insert(account, acc_balance - amount);
            amount
        }

        #[ink(message)]
        pub fn topup(&mut self, account: AccountId, amount: Balance) -> Balance {
            let acc_balance = self.balance_of(account);

            self.balances.insert(account, acc_balance + amount);
            amount
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(&owner).copied().unwrap_or(0)
        }
    }
}

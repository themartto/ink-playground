#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod exchange {
    use ink_storage::Lazy;
    use melons::Melon;
    use pumpkins::Pumpkin;

    #[ink(storage)]
    pub struct Exchange {
        melons: Lazy<Melon>,
        pumpkins: Lazy<Pumpkin>
    }

    impl Exchange {

        #[ink(constructor)]
        pub fn new(
            melons: Hash,
            pumpkins: Hash
        ) -> Self {
            let total_balance = Self::env().balance();
            let version: i32 = 3;
            let salt = version.to_le_bytes();
            let melons = Melon::new(10000)
                .endowment(total_balance / 4)
                .code_hash(melons)
                .salt_bytes(salt)
                .instantiate()
                .expect("failed at instantiating the `Melons` contract");

            let pumpkins = Pumpkin::new(10000)
                .endowment(total_balance / 4)
                .code_hash(pumpkins)
                .salt_bytes(salt)
                .instantiate()
                .expect("failed at instantiating the `Pumpkins` contract");

            Self {
                melons: Lazy::new(melons),
                pumpkins: Lazy::new(pumpkins)
            }
        }

        #[ink(message)]
        pub fn trade_melons_for_pumpkins(
            &mut self,
            amount: Balance
        ) -> Balance {
            let acc = Self::env().caller();
            self.melons.withdraw(
                acc,
                amount
            );
            self.pumpkins.topup(
                acc,
                amount
            );
            amount
        }

        #[ink(message)]
        pub fn show_balance(&mut self) -> Balance {
            let acc = Self::env().caller();
            self.melons.balance_of(acc)
        }
    }
}

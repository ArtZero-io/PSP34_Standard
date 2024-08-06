#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod balances;
mod data;
mod errors;
pub mod metadata;
pub mod ownable;
pub mod psp34_standard;
mod traits;

pub use data::{Id, PSP34Data, PSP34Event};
pub use errors::{Error, OwnableError, PSP34Error};
pub use traits::{Ownable, PSP34Burnable, PSP34Metadata, PSP34Mintable, Psp34Traits, PSP34};

#[cfg(not(feature = "enumerable"))]
pub use traits::PSP34Enumerable;

#[cfg(not(feature = "contract"))]
#[ink::contract]
mod psp34_nft {
    use crate::{
        ownable, psp34_standard, Error, Id, Ownable, OwnableError, PSP34Burnable, PSP34Data,
        PSP34Error, PSP34Event, PSP34Metadata, PSP34Mintable, Psp34Traits, PSP34,
    };
    use ink::prelude::{string::String, vec::Vec};

    #[cfg(not(feature = "enumerable"))]
    use crate::PSP34Enumerable;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Psp34Nft {
        data: PSP34Data,
        ownable: ownable::Data,
        manager_psp34_standard: psp34_standard::manager::Manager,
    }

    impl Psp34Nft {
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId, name: String, symbol: String) -> Self {
            let mut instance = Self::default();
            instance.ownable._init_with_owner(contract_owner);
            instance
                .manager_psp34_standard
                .metadata
                .set_attribute(
                    Id::U8(0),
                    String::from("name").into_bytes(),
                    name.into_bytes(),
                )
                .expect("Failed to set attribute");
            instance
                .manager_psp34_standard
                .metadata
                .set_attribute(
                    Id::U8(0),
                    String::from("symbol").into_bytes(),
                    symbol.into_bytes(),
                )
                .expect("Failed to set attribute");
            instance
        }

        /// This function let NFT Contract Owner to mint a new NFT without providing NFT Traits/Attributes
        #[ink(message)]
        pub fn mint(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            self.ownable._check_owner(Some(caller))?;
            if let Some(last_token_id) = self.manager_psp34_standard.last_token_id.checked_add(1) {
                self.manager_psp34_standard.last_token_id = last_token_id;
                let events = self
                    .data
                    .mint(caller, Id::U64(self.manager_psp34_standard.last_token_id))?;
                self.emit_events(events);
                return Ok(());
            } else {
                return Err(Error::Custom(String::from("Cannot increase last token id")));
            }
        }

        /// This function let NFT Contract Owner to mint a new NFT with NFT Traits/Attributes
        #[ink(message)]
        pub fn mint_with_attributes(
            &mut self,
            metadata: Vec<(String, String)>,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            self.ownable._check_owner(Some(caller))?;
            if let Some(last_token_id) = self.manager_psp34_standard.last_token_id.checked_add(1) {
                self.manager_psp34_standard.last_token_id = last_token_id;
                let events = self
                    .data
                    .mint(caller, Id::U64(self.manager_psp34_standard.last_token_id))?;
                self.emit_events(events);
                if self
                    .set_multiple_attributes(
                        Id::U64(self.manager_psp34_standard.last_token_id),
                        metadata,
                    )
                    .is_err()
                {
                    return Err(Error::Custom(String::from("Cannot set attributes")));
                }
                return Ok(());
            } else {
                return Err(Error::Custom(String::from("Cannot increase last token id")));
            }
        }

        fn emit_events(&self, events: ink::prelude::vec::Vec<PSP34Event>) {
            for event in events {
                match event {
                    PSP34Event::Approval {
                        owner,
                        operator,
                        id,
                        approved,
                    } => self.env().emit_event(Approval {
                        owner,
                        operator,
                        id,
                        approved,
                    }),
                    PSP34Event::Transfer { from, to, id } => {
                        self.env().emit_event(Transfer { from, to, id })
                    }
                    PSP34Event::AttributeSet { id, key, data } => {
                        self.env().emit_event(AttributeSet { id, key, data })
                    }
                }
            }
        }
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        #[ink(topic)]
        id: Option<Id>,
        approved: bool,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    #[ink(event)]
    pub struct AttributeSet {
        id: Id,
        key: Vec<u8>,
        data: Vec<u8>,
    }

    #[ink(event)]
    pub struct OwnershipTransferred {
        #[ink(topic)]
        old_owner: Option<AccountId>,
        #[ink(topic)]
        new_owner: Option<AccountId>,
    }

    impl PSP34 for Psp34Nft {
        #[ink(message)]
        fn collection_id(&self) -> Id {
            self.data.collection_id(self.env().account_id())
        }

        #[ink(message)]
        fn total_supply(&self) -> u128 {
            self.data.total_supply()
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u32 {
            self.data.balance_of(owner)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
            self.data.allowance(owner, operator, id.as_ref())
        }

        #[ink(message)]
        fn transfer(
            &mut self,
            to: AccountId,
            id: Id,
            data: ink::prelude::vec::Vec<u8>,
        ) -> Result<(), PSP34Error> {
            let events = self.data.transfer(self.env().caller(), to, id, data)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn approve(
            &mut self,
            operator: AccountId,
            id: Option<Id>,
            approved: bool,
        ) -> Result<(), PSP34Error> {
            let events = self
                .data
                .approve(self.env().caller(), operator, id, approved)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn owner_of(&self, id: Id) -> Option<AccountId> {
            self.data.owner_of(&id)
        }
    }

    // impl PSP34Mintable for Psp34Nft {
    //     #[ink(message)]
    //     fn mint(&mut self, id: Id) -> Result<(), PSP34Error> {
    //         let events = self.data.mint(self.env().caller(), id)?;
    //         self.emit_events(events);
    //         Ok(())
    //     }
    // }

    impl PSP34Burnable for Psp34Nft {
        #[ink(message)]
        fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            let caller = Self::env().caller();

            if let Some(token_owner) = self.owner_of(id.clone()) {
                if token_owner != account {
                    return Err(PSP34Error::Custom(String::from("not token owner")));
                }

                let allowance = self.allowance(account, caller, Some(id.clone()));

                if caller == account || allowance {
                    self.manager_psp34_standard.locked_tokens.remove(&id);
                    if let Some(locked_token_count) = self
                        .manager_psp34_standard
                        .locked_token_count
                        .checked_sub(1)
                    {
                        self.manager_psp34_standard.locked_token_count = locked_token_count;
                        let events = self.data.burn(caller, account, id)?;
                        self.emit_events(events);
                    } else {
                        return Err(PSP34Error::Custom(String::from("Locked token count error")));
                    }
                } else {
                    return Err(PSP34Error::Custom(String::from(
                        "caller is not token owner or approved",
                    )));
                }
            } else {
                return Err(PSP34Error::Custom(String::from("No token owner found")));
            }
            Ok(())
        }
    }

    impl PSP34Metadata for Psp34Nft {
        #[ink(message)]
        fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>> {
            self.manager_psp34_standard.metadata.get_attribute(id, key)
        }
    }

    impl PSP34Enumerable for Psp34Nft {
        #[ink(message)]
        fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error> {
            self.data.owners_token_by_index(owner, index)
        }

        #[ink(message)]
        fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error> {
            self.data.token_by_index(index)
        }
    }

    impl Ownable for Psp34Nft {
        #[ink(message)]
        fn owner(&self) -> Option<AccountId> {
            self.ownable.owner()
        }
        #[ink(message)]
        fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
            self.ownable._check_owner(Some(self.env().caller()))?;
            self.ownable.renounce_ownership()?;
            self.env().emit_event(OwnershipTransferred {
                old_owner: Some(self.env().caller()),
                new_owner: None,
            });

            Ok(())
        }
        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: Option<AccountId>) -> Result<(), OwnableError> {
            self.ownable._check_owner(Some(self.env().caller()))?;
            self.ownable.transfer_ownership(new_owner)?;
            self.env().emit_event(OwnershipTransferred {
                old_owner: self.owner(),
                new_owner,
            });

            Ok(())
        }
    }

    impl Psp34Traits for Psp34Nft {
        #[ink(message)]
        fn set_base_uri(&mut self, uri: String) -> Result<(), Error> {
            self.ownable._check_owner(Some(self.env().caller()))?;
            self.manager_psp34_standard.set_base_uri(uri)
        }
        #[ink(message)]
        fn set_multiple_attributes(
            &mut self,
            token_id: Id,
            metadata: Vec<(String, String)>,
        ) -> Result<(), Error> {
            self.ownable._check_owner(Some(self.env().caller()))?;
            self.manager_psp34_standard
                .set_multiple_attributes(token_id, metadata)
        }
        #[ink(message)]
        fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String> {
            self.manager_psp34_standard
                .get_attributes(token_id, attributes)
        }
        #[ink(message)]
        fn get_attribute_count(&self) -> u32 {
            self.manager_psp34_standard.get_attribute_count()
        }
        #[ink(message)]
        fn get_attribute_name(&self, index: u32) -> String {
            self.manager_psp34_standard.get_attribute_name(index)
        }
        #[ink(message)]
        fn token_uri(&self, token_id: u64) -> String {
            self.manager_psp34_standard.token_uri(token_id)
        }
        #[ink(message)]
        fn get_last_token_id(&self) -> u64 {
            self.manager_psp34_standard.get_last_token_id()
        }
        #[ink(message)]
        fn lock(&mut self, token_id: Id) -> Result<(), Error> {
            if self.owner_of(token_id.clone()) != Some(self.env().caller()) {
                return Err(Error::OwnableError(OwnableError::CallerIsNotOwner));
            }
            self.manager_psp34_standard.lock(token_id)?;
            Ok(())
        }
        #[ink(message)]
        fn is_locked_nft(&self, token_id: Id) -> bool {
            self.manager_psp34_standard.is_locked_nft(token_id)
        }
        #[ink(message)]
        fn get_locked_token_count(&self) -> u64 {
            self.manager_psp34_standard.get_locked_token_count()
        }

        #[ink(message)]
        fn get_owner(&self) -> AccountId {
            self.ownable.owner().unwrap()
        }
    }
}

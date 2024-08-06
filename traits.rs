use ink::prelude::string::String;
use ink::{prelude::vec::Vec, primitives::AccountId};

use crate::data::Id;
use crate::errors::{Error, OwnableError, PSP34Error};

#[ink::trait_definition]
pub trait PSP34 {
    /// Returns the collection `Id` of the NFT token.
    ///
    /// This can represents the relationship between tokens/contracts/pallets.
    #[ink(message)]
    fn collection_id(&self) -> Id;

    /// Returns the current total supply of the NFT.
    #[ink(message)]
    fn total_supply(&self) -> u128;

    /// Returns the account balance for the specified `owner`.
    ///
    /// This represents the amount of unique tokens the owner has.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> u32;

    /// Returns `true` if the operator is approved by the owner to withdraw `id` token.
    ///
    /// If `id` is `None`, returns `true` if the operator is approved to withdraw all owner's tokens.
    #[ink(message)]
    fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool;

    /// Transfer approved or owned token from caller.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `TokenNotExists` error if `id` does not exist.
    ///
    /// Returns `NotApproved` error if `from` doesn't have allowance for transferring.
    ///
    /// Returns `SafeTransferCheckFailed` error if `to` doesn't accept transfer.
    #[ink(message)]
    fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error>;

    /// Approves `operator` to withdraw  the `id` token from the caller's account.
    /// If `id` is `None` approves or disapproves the operator for all tokens of the caller.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `SelfApprove` error if it is self approve.
    ///
    /// Returns `NotApproved` error if caller is not owner of `id`.
    #[ink(message)]
    fn approve(
        &mut self,
        operator: AccountId,
        id: Option<Id>,
        approved: bool,
    ) -> Result<(), PSP34Error>;

    /// Returns the owner of the token if any.
    #[ink(message)]
    fn owner_of(&self, id: Id) -> Option<AccountId>;
}

#[ink::trait_definition]
pub trait PSP34Metadata {
    /// Returns the attribute of `id` for the given `key`.
    ///
    /// If `id` is a collection id of the token, it returns attributes for collection.
    #[ink(message)]
    fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>>;
}

#[ink::trait_definition]
pub trait PSP34Mintable {
    /// Mints a token to the sender's account.
    ///
    /// # Events
    ///
    /// On success a `Transfer` event is emitted with `None` sender.
    ///
    /// # Errors
    ///
    /// Reverts with `TokenExists`` if token id is already in the library.
    ///
    /// Reverts with `Custom (max supply exceeded)` if the incremented by 1 total
    /// supply exceeds maximal value of `u128` type.
    #[ink(message)]
    fn mint(&mut self, id: Id) -> Result<(), PSP34Error>;
}

#[ink::trait_definition]
pub trait PSP34Burnable {
    /// Burns token from the selected account.
    ///
    /// # Events
    ///
    /// On success a `Transfer` event is emitted with `None` recipient.
    ///
    /// # Errors
    ///
    /// Reverts with `TokenExists` if token id is already in the library.
    #[ink(message)]
    fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error>;
}

#[cfg(not(feature = "enumerable"))]
#[ink::trait_definition]
pub trait PSP34Enumerable {
    /// Returns a token `Id` owned by `owner` at a given `index` of its token list.
    /// Use along with `balance_of` to enumerate all of ``owner``'s tokens.
    #[ink(message)]
    fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error>;

    /// Returns a token `Id` at a given `index` of all the tokens stored by the contract.
    /// Use along with `total_supply` to enumerate all tokens.
    #[ink(message)]
    fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error>;
}

#[ink::trait_definition]
pub trait Ownable {
    #[ink(message)]
    fn owner(&self) -> Option<AccountId>;
    #[ink(message)]
    fn renounce_ownership(&mut self) -> Result<(), OwnableError>;
    #[ink(message)]
    fn transfer_ownership(&mut self, new_owner: Option<AccountId>) -> Result<(), OwnableError>;
}

#[ink::trait_definition]
pub trait Psp34Traits {
    /// This function sets the baseURI for the NFT contract. Only Contract Owner can perform this function. baseURI is the location of the metadata files if the NFT collection use external source to keep their NFT artwork. ArtZero uses IPFS by default, the baseURI can have format like this: ipfs://<hash_ID>/
    #[ink(message)]
    fn set_base_uri(&mut self, uri: String) -> Result<(), Error>;
    /// This function set the attributes to each NFT. Only Contract Owner can perform this function. The metadata input is an array of [(attribute, value)]. The attributes in ArtZero platform are the NFT traits.
    #[ink(message)]
    fn set_multiple_attributes(
        &mut self,
        token_id: Id,
        metadata: Vec<(String, String)>,
    ) -> Result<(), Error>;
    /// This function returns all available attributes of each NFT
    #[ink(message)]
    fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String>;
    /// This function return how many unique attributes in the contract
    #[ink(message)]
    fn get_attribute_count(&self) -> u32;
    /// This function return the attribute name using attribute index. Beacause attributes of an NFT can be set to anything by Contract Owner, AztZero uses this function to get all attributes of an NFT
    #[ink(message)]
    fn get_attribute_name(&self, index: u32) -> String;
    /// This function return the metadata location of an NFT. The format is baseURI/<token_id>.json
    #[ink(message)]
    fn token_uri(&self, token_id: u64) -> String;
    /// This function return the owner of the NFT Contract
    #[ink(message)]
    fn get_last_token_id(&self) -> u64;
    /// This function lets NFT owner to lock their NFT. Once locked, the NFT traits (attributes) can not be changed
    #[ink(message)]
    fn lock(&mut self, token_id: Id) -> Result<(), Error>;
    /// This function check if an NFT is locked or not
    #[ink(message)]
    fn is_locked_nft(&self, token_id: Id) -> bool;
    /// This function returns how many NFTs have been locked by its owners
    #[ink(message)]
    fn get_locked_token_count(&self) -> u64;

    #[ink(message)]
    fn get_owner(&self) -> AccountId;
}

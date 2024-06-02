pub mod manager {
    use crate::{data::Id, metadata, Error};
    use ink::prelude::string::ToString;
    use ink::{
        prelude::{string::String, vec::Vec},
        storage::Mapping,
    };
    #[ink::storage_item]
    #[derive(Default, Debug)]
    pub struct Manager {
        pub last_token_id: u64,
        pub attribute_count: u32,
        pub attribute_names: Mapping<u32, Vec<u8>>,
        pub is_attribute: Mapping<String, bool>,
        pub locked_tokens: Mapping<Id, bool>,
        pub locked_token_count: u64,
        pub metadata: metadata::Data,
        _reserved: Option<()>,
    }

    impl Manager {
        pub fn new() -> Manager {
            Default::default()
        }

        /// Get Token Count
        pub fn get_last_token_id(&self) -> u64 {
            return self.last_token_id;
        }

        /// Lock nft - Only owner token
        pub fn lock(&mut self, token_id: Id) -> Result<(), Error> {
            if let Some(locked_token_count) = self.locked_token_count.checked_add(1) {
                self.locked_token_count = locked_token_count;
                self.locked_tokens.insert(&token_id, &true);
                return Ok(());
            } else {
                return Err(Error::Custom(String::from(
                    "Cannot increase locked token count",
                )));
            }
        }

        /// Check token is locked or not
        pub fn is_locked_nft(&self, token_id: Id) -> bool {
            if self.locked_tokens.get(&token_id).is_some() {
                return true;
            }
            return false;
        }

        /// Get Locked Token Count
        pub fn get_locked_token_count(&self) -> u64 {
            self.locked_token_count
        }

        /// Change baseURI
        pub fn set_base_uri(&mut self, uri: String) -> Result<(), Error> {
            self.metadata.set_attribute(
                Id::U8(0),
                String::from("baseURI").into_bytes(),
                uri.into_bytes(),
            )?;
            Ok(())
        }

        /// Only Owner can set multiple attributes to a token
        pub fn set_multiple_attributes(
            &mut self,
            token_id: Id,
            metadata: Vec<(String, String)>,
        ) -> Result<(), Error> {
            if token_id == Id::U64(0) {
                return Err(Error::InvalidInput);
            }
            if self.is_locked_nft(token_id.clone()) {
                return Err(Error::Custom(String::from("Token is locked")));
            }
            for (attribute, value) in &metadata {
                self.add_attribute_name(&attribute.clone().into_bytes())?;
                self.metadata.set_attribute(
                    token_id.clone(),
                    attribute.clone().into_bytes(),
                    value.clone().into_bytes(),
                )?;
            }
            Ok(())
        }

        /// Get multiple  attributes
        pub fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String> {
            let length = attributes.len();
            let mut ret = Vec::<String>::new();
            for i in 0..length {
                let attribute = attributes[i].clone();
                let value = self
                    .metadata
                    .get_attribute(token_id.clone(), attribute.into_bytes());

                if let Some(value_in_bytes) = value {
                    if let Ok(value_in_string) = String::from_utf8(value_in_bytes) {
                        ret.push(value_in_string);
                    } else {
                        ret.push(String::from(""));
                    }
                } else {
                    ret.push(String::from(""));
                }
            }
            ret
        }

        /// Get Attribute Count
        pub fn get_attribute_count(&self) -> u32 {
            self.attribute_count
        }
        /// Get Attribute Name
        pub fn get_attribute_name(&self, index: u32) -> String {
            let attribute = self.attribute_names.get(&index);

            if let Some(value_in_bytes) = attribute {
                if let Ok(value_in_string) = String::from_utf8(value_in_bytes) {
                    return value_in_string;
                } else {
                    return String::from("");
                }
            } else {
                return String::from("");
            }
        }

        /// Get URI from token ID
        pub fn token_uri(&self, token_id: u64) -> String {
            let value = self
                .metadata
                .get_attribute(Id::U8(0), String::from("baseURI").into_bytes());
            let mut token_uri = String::from("");

            if let Some(value_in_bytes) = value {
                if let Ok(value_in_string) = String::from_utf8(value_in_bytes) {
                    token_uri = value_in_string;
                }
            }

            token_uri = token_uri + &token_id.to_string() + &String::from(".json");
            token_uri
        }

        fn add_attribute_name(&mut self, attribute_input: &Vec<u8>) -> Result<(), Error> {
            if let Ok(attr_input) = String::from_utf8((*attribute_input).clone()) {
                let exist: bool = self.is_attribute.get(&attr_input).is_some();

                if !exist {
                    if let Some(attribute_count) = self.attribute_count.checked_add(1) {
                        self.attribute_count = attribute_count;
                        self.attribute_names
                            .insert(&self.attribute_count, attribute_input);
                        self.is_attribute.insert(&attr_input, &true);
                        return Ok(());
                    } else {
                        return Err(Error::Custom(String::from(
                            "Fail to increase attribute count",
                        )));
                    }
                } else {
                    return Err(Error::Custom(String::from("Attribute input exists")));
                }
            } else {
                return Err(Error::Custom(String::from("Attribute input error")));
            }
        }
    }
}

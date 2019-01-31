// Users zome
#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry, dna::entry_types::Sharing, error::HolochainError, json::JsonString,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct User {
    first_name: String,
    last_name: String,
    position: String,
    title: String,
    orcid: String,
}

pub fn handle_create_user(user: User) -> ZomeApiResult<Address> {
    let entry = Entry::App("user".into(), user.into());
    hdk::commit_entry(&entry)
}

pub fn handle_get_user(user_addr: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&user_addr)
}

fn definition() -> ValidatingEntryType {
    entry!(
        name: "user",
        description: "A user of Omni",
        sharing: Sharing::Public,
        native_type: User,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_user: User, _validation_data: hdk::ValidationData| {
            Ok(())
        }
    )
}
define_zome! {
    entries: [
       definition()
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            create_user: {
                inputs: |user: User|,
                outputs: |result: ZomeApiResult<Address>|,
                handler: handle_create_user
            }
            get_user: {
                inputs: |user_addr: Address|,
                outputs: |result: ZomeApiResult<Option<Entry>>|,
                handler: handle_get_user
            }
        }
    }
}

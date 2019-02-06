// Users zome
#![feature(try_from)]
#![feature(uniform_paths)]

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
    error::{
        ZomeApiResult, ZomeApiError
    }
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry, dna::entry_types::Sharing, error::HolochainError, json::JsonString,
};

pub mod affiliation;
use affiliation::Org;
pub mod user;
use user::NewUser;

pub fn handle_affiliate_user(user_addr: Address, org_addr: Address) -> Result<(), ZomeApiError> {
    hdk::link_entries(&user_addr, &org_addr, "user_from_org")
}

define_zome! {
    entries: [

        entry!(
            name: "user",
            description: "A user of Omni",
            sharing: Sharing::Public,
            native_type: User,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_user: NewUser, _validation_data: hdk::ValidationData| {
                Ok(())
            },
            links: [
                from!(
                    "org",
                    tag: "user_from_org",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: |_base: Address, _target: Address, _validation_data: hdk::ValidationData | {
                        Ok(())
                    }
                )
            ]
        ),

        entry!(
            name: "org",
            description: "A research organisation",
            sharing: Sharing::Public,
            native_type: Org,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_org: Org, _validation_data: hdk::ValidationData| {
                Ok(())
            }
        )
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            create_user: {
                inputs: |user: NewUser|,
                outputs: |result: ZomeApiResult<Address>|,
                handler: user::create_user
            }
            get_user: {
                inputs: |user_addr: Address|,
                outputs: |result: ZomeApiResult<Option<Entry>>|,
                handler: user::get_user
            }
            create_organisation: {
                inputs: |org: Org|,
                outputs: |result: ZomeApiResult<Address>|,
                handler: affiliation::create_organisation
            }
            get_organisation: {
                inputs: |org_addr: Address|,
                outputs: |result: ZomeApiResult<Option<Entry>>|,
                handler: affiliation::get_organisation
            }
            affiliate_user: {
                inputs: |user_addr: Address, org_addr: Address|,
                outputs: |result: Result<(), ZomeApiError>|,
                handler: handle_affiliate_user
            }
        }
    }

    // capabilities: {
    //     public (Public) [
    //         create_user,
    //         get_user,
    //         create_organisation,
    //         get_organisation,
    //         affiliate_user
    //     ]
    // }
}

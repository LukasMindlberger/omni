#![feature(try_from)]

#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate holochain_core_types_derive;
#[macro_use]
extern crate serde_json;

pub mod community;
use self::community::CommunityMeta;

use hdk::{
    holochain_core_types::{
        hash::HashString,
        dna::zome::entry_types::Sharing,
    },
};

define_zome! {
    entries: [
    entry!(
        name: "community",
        description: "An Omni community",
        sharing: Sharing::Public,
        native_type: String,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |article: String, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            create_community {
                inputs: |community: String|,
                outputs: |result: JsonString|,
                handler: community::handle_create_community
            },
            get_community {
                inputs: |community: String|,
                outputs: |result: JsonString|,
                handler: community::handle_get_community
            }
        }
    }
}

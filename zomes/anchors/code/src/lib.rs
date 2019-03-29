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
    error::{ZomeApiResult, ZomeApiError},
    AGENT_ADDRESS,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry, dna::entry_types::Sharing, error::HolochainError, json::JsonString,
};
use holochain_wasm_utils::api_serialization::{
    get_links::GetLinksResult
};

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Anchor {
    anchor: String
}

fn handle_link_agent_to_anchor(anchor: String) -> ZomeApiResult<Address> {
    let anchor_entry = Entry::App("anchor".into(), Anchor{anchor}.into());
    let anchor_address = hdk::commit_entry(&anchor_entry)?;
    hdk::link_entries(&anchor_address, &AGENT_ADDRESS, "anchor_to_users")?;
    Ok(anchor_address)
}

fn handle_get_users(anchor_address: Address) -> ZomeApiResult<GetLinksResult> {
    hdk::get_links(&anchor_address, "anchor_to_users")
}

// see https://developer.holochain.org/api/0.0.6-alpha/hdk/ for info on using the hdk library
define_zome! {
    entries: [
        entry!(
            name: "anchor",
            description: "An anchor, known by all users, to link data from",
            sharing: Sharing::Public,
            native_type: Anchor,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_anchor: Anchor, _validation_data: hdk::ValidationData| {
                Ok(())
            },
            links: [
                to!(
                    "%agent_id",
                    tag: "anchor_to_users",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::ChainFull
                    },
                    validation: |_agent_addr: Address, _anchor_addr: Address, _validation_data: hdk::ValidationData | {
                        Ok(())
                    }
                )
            ]
        )
    ]

    genesis: || { Ok(()) }

    functions: [

    register_self: {
        inputs: |anchor: String|,
        outputs: |result: ZomeApiResult<Address>|,
        handler: handle_link_agent_to_anchor
    }

    get_users: {
        inputs: |anchor_address: Address|,
        outputs: |result: ZomeApiResult<GetLinksResult>|,
        handler: handle_get_users
    }
    ]

    traits: {
        hc_public [
            get_users, register_self
        ]
    }
}

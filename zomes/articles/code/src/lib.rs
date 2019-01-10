#![feature(try_from)]
use std::convert::TryFrom;

#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate holochain_core_types_derive;
#[macro_use]
extern crate serde_json;

use hdk::holochain_core_types::{
    hash::HashString,
    error::HolochainError,
    entry::Entry,
    dna::zome::entry_types::Sharing,
    entry::entry_type::EntryType,
    json::JsonString,
    cas::content::Address
};


// Data structs
#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct Article {
    title: String,
    abst: String,
    body: String,
}


// CRUD for zome
fn handle_create_article(article: Article) -> JsonString {
    let article_entry = Entry::new(EntryType::App("article".into()), article);

    match hdk::commit_entry(&article_entry) {
        Ok(_) => json!({"success": true}).into(),
        Err(hdk_err) => hdk_err.into()
    }
}


// Validation logic & links
define_zome! {
    entries: [
    entry!(
        name: "article",
        description: "An article",
        sharing: Sharing::Public,
        native_type: Article,
        validation_package: || hdk::ValidationPackageDefinition::Entry,
        validation: |article: Article, _ctx: hdk::ValidationData| {
            Ok(())
        }//,
        // links: [
        // from!(
        //     "AGENT_ADDRESS", // is this correct?
        //     tag: "author",
        //     validation_package: || hdk::ValidationPackageDefinition::Entry,
        //     validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
        //         Ok(())
        //     }
        // )
        // ]
    )
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            create_article: {
                inputs: |article: Article|,
                outputs: |result: JsonString|,
                handler: handle_create_article
            }
            // get_article: {
            //     inputs: |article_addr: HashString|,
            //     outputs: |result: JsonString|,
            //     handler: handle_get_article
            // }
            // update_article: {
            //     inputs: |article: Article, article_addr: HashString|,
            //     outputs: |result: JsonString|,
            //     handler: handle_update_article
            // }
        }
    }
}

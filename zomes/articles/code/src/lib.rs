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

use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        hash::HashString,
        error::HolochainError,
        entry::Entry,
        dna::zome::entry_types::Sharing,
        entry::entry_type::EntryType,
        json::JsonString,
        cas::content::Address
    },
};


// Data structs
#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct Article {
    title: String,
    abst: String,
    body: String,
}


// CRUD for zome

/*  POSSIBLE ZOME API RESULTS:
        JsonString
        Address
        GetLinksResult
        Option<Foo>
        Vec<Foo>
*/

fn handle_create_article(article: Article) -> JsonString  {
    let article_entry = Entry::new(EntryType::App("article".into()), article);

    match hdk::commit_entry(&article_entry) {
        Ok(article_addr) => json!({"success": true, "address": article_addr}).into(),
        Err(hdk_err) => hdk_err.into()
    }
}

fn handle_get_article(article_addr: HashString) -> JsonString {
    match hdk::get_entry(article_addr) {
        Ok(Some(entry)) => entry.value().to_owned(),
        Ok(None) => {}.into(),
        Err(hdk_err) => hdk_err.into()
    }
}

// fn handle_update_article(article: Article, article_addr: HashString) -> JsonString {
//     let article_entry = Entry::new(EntryType::App("article".into()), article);
//
//     match hdk::update_entry(&article_entry, article_addr) {
//         Ok(article_addr) => json!({"success": true, "address": article_addr}).into(),
//         Err(hdk_err) => hdk_err.into()
//     }
// }

// fn handle_delete_article(article_addr: HashString) -> JsonString {
//     match hdk::remove_entry(article_addr) {
//         Ok(_) => json!({"success": true}).into(),
//         Err(hdk_err) => hdk_err.into()
//     }
// }


// Validation logic & links
define_zome! {
    entries: [
    entry!(
        name: "article",
        description: "An article",
        sharing: Sharing::Public,
        native_type: Article,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |article: Article, _ctx: hdk::ValidationData| {
            Ok(())
        }
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
            get_article: {
                inputs: |article_addr: HashString|,
                outputs: |result: JsonString|,
                handler: handle_get_article
            }
            // update_article: {
            //     inputs: |article_addr: HashString, article: Article|,
            //     outputs: |result: JsonString|,
            //     handler: handle_update_article
            // }
            // delete_article: {
            //     inputs: |article_addr: HashString|,
            //     outputs: |result: JsonString|,
            //     handler: handle_delete_article
            // }
        }
    }
}

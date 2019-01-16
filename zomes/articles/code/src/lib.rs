#![feature(try_from)]

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

pub mod article;
use self::article::Article;

use hdk::{
    holochain_core_types::{
        hash::HashString,
        dna::zome::entry_types::Sharing,
    },
};


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
                handler: article::handle_create_article
            }
            get_article: {
                inputs: |article_addr: HashString|,
                outputs: |result: JsonString|,
                handler: article::handle_get_article
            }
            // update_article: {
            //     inputs: |article_addr: HashString, article: Article|,
            //     outputs: |result: JsonString|,
            //     handler: article::handle_update_article
            // }
            // delete_article: {
            //     inputs: |article_addr: HashString|,
            //     outputs: |result: JsonString|,
            //     handler: article::handle_delete_article
            // }
        }
    }
}

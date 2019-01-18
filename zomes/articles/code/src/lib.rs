#![feature(try_from)]

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate holochain_core_types_derive;

pub mod article;
use self::article::Article;

use hdk::{
    error::ZomeApiResult,
    holochain_core_types::
    {
        cas::content::Address,
        dna::entry_types::Sharing,
        entry::Entry,
    }
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
                inputs: |title: String, abst: String, body: String|,
                outputs: |result: ZomeApiResult<Address>|,
                handler: article::create_article
            }
            get_article: {
                inputs: |article_addr: Address|,
                outputs: |result: ZomeApiResult<Option<Entry>>|,
                handler: article::get_article
            }
            update_article: {
                inputs: |article_addr: Address, title: String, abst: String, body: String|,
                outputs: |result: ZomeApiResult<Address>|,
                handler: article::update_article
            }
            delete_article: {
                inputs: |article_addr: Address|,
                outputs: |result: ZomeApiResult<()>|,
                handler: article::delete_article
            }
        }
    }
}

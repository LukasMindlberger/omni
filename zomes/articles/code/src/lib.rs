// Article zome
#![feature(try_from)]

#[macro_use]
extern crate hdk;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate holochain_core_types_derive;

pub mod article;
use article::Article;

use boolinator::Boolinator;
use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        cas::content::Address,
        dna::entry_types::Sharing,
        entry::Entry,
    }
};
use holochain_wasm_utils::api_serialization::{
    get_links::GetLinksResult,
    get_entry::{
        GetEntryResult, GetEntryOptions, StatusRequestKind
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
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: |article: Article, _ctx: hdk::ValidationData| {(
                (article.title().len() < 300) & (article.abst().len() < 500)
            )
            .ok_or_else(|| String::from("Article struct is invalid"))
        },
        links: [
            from!(
                "%agent_id",
                tag: "authored_article",
                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },
                validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData | {
                    Ok(())
                }
            )
        ]
    )
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            get_sources_latest: {
                inputs: |address: Address|,
                outputs: |result: ZomeApiResult<GetEntryResult>|,
                handler: handle_get_sources_latest
            }
            get_sources_initial: {
                inputs: |address: Address|,
                outputs: |result: ZomeApiResult<GetEntryResult>|,
                handler: handle_get_sources_initial
            }
            article_address: {
                inputs: |title: String, abst: String, body: String|,
                outputs: |article: ZomeApiResult<Address>|,
                handler: article::article_address
            }
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
            get_authored_articles: {
                inputs: |agent_addr: Address|,
                outputs: |result: ZomeApiResult<GetLinksResult>|,
                handler: article::get_authored_articles
            }
        }
    }
}

fn handle_get_sources_latest(address: Address) -> ZomeApiResult<GetEntryResult> {
    let options = GetEntryOptions::new(StatusRequestKind::Latest, false, false, true);

    hdk::get_entry_result(&address, options)
}

fn handle_get_sources_initial(address: Address) -> ZomeApiResult<GetEntryResult> {
    let options = GetEntryOptions::new(StatusRequestKind::Initial, false, false, true);

    hdk::get_entry_result(&address, options)
}

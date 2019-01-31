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

use boolinator::Boolinator;
use hdk::{
    error::{
        ZomeApiResult, ZomeApiError
    },
    holochain_core_types::{
        cas::content::Address,
        dna::entry_types::Sharing,
        entry::Entry,
        error::HolochainError,
        json::JsonString,
    }
};
use holochain_wasm_utils::api_serialization::{
    get_links::GetLinksResult,
    get_entry::{
        GetEntryResult, GetEntryOptions, StatusRequestKind
    },
};

pub mod article;
pub mod keyword;

use article::Article;
use keyword::Keyword;


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
            validation: |article: Article, _validation_data: hdk::ValidationData| {(
                    (article.title().len() < 300) & (article.abst().len() < 500)
                )
                .ok_or_else(|| String::from("Article struct is invalid"))
            },
            links: [
                from!(
                    "%agent_id",
                    tag: "articles_from_agent",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: |_base: Address, _target: Address, _validation_data: hdk::ValidationData | {
                        Ok(())
                    }
                ),
                from!(
                    "keyword",
                    tag: "articles_from_keyword",
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
            name: "keyword",
            description: "A keyword to help categorise content",
            sharing: Sharing::Public,
            native_type: Keyword,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |article: Keyword, _validation_data: hdk::ValidationData| {
                Ok(())
            }
        )
    ]

    genesis: || { Ok(()) }

    functions: [

        // get_sources_latest: {
        //     inputs: |address: Address|,
        //     outputs: |result: ZomeApiResult<GetEntryResult>|,
        //     handler: handle_get_sources_latest
        // }
        //
        // get_sources_initial: {
        //     inputs: |address: Address|,
        //     outputs: |result: ZomeApiResult<GetEntryResult>|,
        //     handler: handle_get_sources_initial
        // }

        create_article: {
            inputs: |title: String, abst: String, body: String, keywords: Vec<String>|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: article::create_article
        }

        get_article: {
            inputs: |article_addr: Address|,
            outputs: |result: ZomeApiResult<Option<Entry>>|,
            handler: article::get_article
        }

        update_article: {
            inputs: |article_addr: Address, title: String, abst: String, body: String, keywords: Vec<String>|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: article::update_article
        }

        delete_article: {
            inputs: |article_addr: Address|,
            outputs: |result: ZomeApiResult<()>|,
            handler: article::delete_article
        }

        article_address: {
            inputs: |title: String, abst: String, body: String, keywords: Vec<String>|,
            outputs: |article: ZomeApiResult<Address>|,
            handler: article::article_address
        }

        get_authored_articles: {
            inputs: |agent_addr: Address|,
            outputs: |result: ZomeApiResult<GetLinksResult>|,
            handler: article::get_authored_articles
        }

        create_keyword: {
            inputs: |keyword: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: keyword::create_keyword
        }

        get_keyword: {
            inputs: |keyword_addr: Address|,
            outputs: |result: ZomeApiResult<Option<Entry>>|,
            handler: keyword::get_keyword
        }

        link_article_from_keyword: {
            inputs: |keyword_addr: Address, article_addr: Address|,
            outputs: |result: Result<(), ZomeApiError>|,
            handler: keyword::link_article_from_keyword
        }

        get_articles_from_keyword: {
            inputs: |keyword_address: Address|,
            outputs: |result: ZomeApiResult<GetLinksResult>|,
            handler: keyword::get_articles_from_keyword
        }
    ]

    capabilities: {
        public (Public) [
            create_article,
            get_article,
            update_article,
            delete_article,
            article_address,
            get_authored_articles,
            create_keyword,
            get_keyword,
            get_articles_from_keyword
        ]
    }
}

// fn handle_get_sources_latest(address: Address) -> ZomeApiResult<GetEntryResult> {
//     let options = GetEntryOptions::new(StatusRequestKind::Latest, false, false, true);
//
//     hdk::get_entry_result(&address, options)
// }
//
// fn handle_get_sources_initial(address: Address) -> ZomeApiResult<GetEntryResult> {
//     let options = GetEntryOptions::new(StatusRequestKind::Initial, false, false, true);
//
//     hdk::get_entry_result(&address, options)
// }

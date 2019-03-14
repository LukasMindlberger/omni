#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::{
    error::{
        ZomeApiResult, ZomeApiError
    },
    holochain_core_types::{
        cas::content::Address,
        dna::entry_types::Sharing,
        entry::Entry,
        json::JsonString,
        error::HolochainError
    }
};
use holochain_wasm_utils::api_serialization::{
    get_links::GetLinksResult
};
use boolinator::Boolinator;

pub mod article;
pub mod keyword;
pub mod utils;
use article::Article;
use keyword::Keyword;
use utils::Env;

// The Articles DNA
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
                    (article.title().len() < 300) & (article.abst().len() < 900)
                )
                .ok_or_else(|| String::from("Article struct is invalid"))
            },
            links: [
                from!(
                    "%agent_id",
                    tag: "articles_from_agent",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::ChainFull
                    },
                    validation: |_agent_addr: Address, _article_addr: Address, _validation_data: hdk::ValidationData | {
                        Ok(())
                    }
                ),
                from!(
                    "keyword",
                    tag: "articles_from_keyword",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::ChainFull
                    },
                    validation: |_keyword_addr: Address, _article_addr: Address, _validation_data: hdk::ValidationData | {
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
            validation: |_keyword: Keyword, _validation_data: hdk::ValidationData| {
                Ok(())
            }
        )
    ]

    genesis: || { Ok(()) }

    functions: [

    show_env: {
        inputs: | |,
        outputs: |result: ZomeApiResult<Env>|,
        handler: utils::handle_show_env
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
        outputs: |result: ZomeApiResult<Vec<ZomeApiResult<Entry>>>|,
        handler: article::get_authored_articles
    }

    get_keyword: {
        inputs: |keyword_addr: Address|,
        outputs: |result: ZomeApiResult<Option<Entry>>|,
        handler: keyword::get_keyword
    }

    create_and_link_keyword_to_article: {
        inputs: |keyword: String, article_addr: Address|,
        outputs: |result: Result<(), ZomeApiError>|,
        handler: keyword::create_and_link_keyword_to_article
    }

    get_articles_from_keyword: {
        inputs: |keyword: String|,
        outputs: |result: ZomeApiResult<Vec<ZomeApiResult<Entry>>>|,
        handler: keyword::get_articles_from_keyword
    }
    ]

    traits: {
        hc_public [
            show_env,
            get_meta_latest,
            get_meta_initial,
            create_article,
            get_article,
            update_article,
            delete_article,
            author_article,
            get_authored_articles,
            get_keyword,
            create_and_link_keyword_to_article,
            get_articles_from_keyword
        ]
    }
}

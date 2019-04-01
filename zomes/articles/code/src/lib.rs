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

            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Article>| {
                 Ok(())
            },
            links: [
                from!(
                    "%agent_id",
                    tag: "articles_from_agent",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: |_validation_data: hdk::LinkValidationData | {
                        Ok(())
                    }
                ),
                from!(
                    "keyword",
                    tag: "articles_from_keyword",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: |_validation_data: hdk::LinkValidationData | {
                        Ok(())
                    }
                )
            ]
        ),

        entry!(
            name: "keyword",
            description: "A keyword to help categorise content",
            sharing: Sharing::Public,

            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Keyword>| {
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
        outputs: |result: ZomeApiResult<GetLinksResult>|,
        handler: article::get_authored_articles
    }

    get_my_articles: {
        inputs: | |,
        outputs: |result: ZomeApiResult<GetLinksResult>|,
        handler: article::get_my_articles
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
            get_my_articles,
            get_keyword,
            create_and_link_keyword_to_article,
            get_articles_from_keyword
        ]
    }
}

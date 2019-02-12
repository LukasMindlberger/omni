// Article zome
#![feature(try_from)]
#![feature(uniform_paths)]

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
        json::JsonString,
        error::HolochainError
    },
    AGENT_ADDRESS,
    AGENT_ID_STR,
    DNA_NAME,
    DNA_ADDRESS
};
use holochain_wasm_utils::api_serialization::{
    get_links::GetLinksResult,
    get_entry::{
        GetEntryResult, GetEntryOptions, StatusRequestKind
    },
};

pub mod article;
use article::Article;
pub mod keyword;
use keyword::Keyword;

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Env {
    dna_name: String,
    dna_address: String,
    agent_id: String,
    agent_address: String,
}

fn handle_show_env() -> ZomeApiResult<Env> {
    Ok(Env{
        dna_name: DNA_NAME.to_string(),
        dna_address: DNA_ADDRESS.to_string(),
        agent_id: AGENT_ID_STR.to_string(),
        agent_address: AGENT_ADDRESS.to_string(),
    })
}

fn handle_get_sources_latest(address: Address) -> ZomeApiResult<GetEntryResult> {
    let options = GetEntryOptions{status_request: StatusRequestKind::Latest, entry: false, header: false, sources: true};

    hdk::get_entry_result(&address, options)
}

fn handle_get_sources_initial(address: Address) -> ZomeApiResult<GetEntryResult> {
    let options = GetEntryOptions{status_request: StatusRequestKind::Initial, entry: false, header: false, sources: true};

    hdk::get_entry_result(&address, options)
}

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
                    (article.title().len() < 300) & (article.abst().len() < 900)
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
                    validation: |_agent_addr: Address, _article_addr: Address, _validation_data: hdk::ValidationData | {
                        Ok(())
                    }
                ),
                from!(
                    "keyword",
                    tag: "articles_from_keyword",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
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

    functions: {
        main (Public) {
            show_env: {
                inputs: | |,
                outputs: |result: ZomeApiResult<Env>|,
                handler: handle_show_env
            }

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

            // create_keyword: {
            //     inputs: |keyword: Keyword|,
            //     outputs: |result: ZomeApiResult<Address>|,
            //     handler: keyword::create_keyword
            // }

            get_keyword: {
                inputs: |keyword_addr: Address|,
                outputs: |result: ZomeApiResult<Option<Entry>>|,
                handler: keyword::get_keyword
            }

            link_article_from_keyword: {
                inputs: |keyword: Keyword, article_addr: Address|,
                outputs: |result: Result<(), ZomeApiError>|,
                handler: keyword::link_article_from_keyword
            }

            get_articles_from_keyword: {
                inputs: |keyword_addr: Address|,
                outputs: |result: ZomeApiResult<GetLinksResult>|,
                handler: keyword::get_articles_from_keyword
            }
        }
    }

    // capabilities: {
    //     public (Public) [
    //         create_article,
    //         get_article,
    //         update_article,
    //         delete_article,
    //         author_article,
    //         get_authored_articles,
    //         create_keyword,
    //         get_keyword,
    //         link_article_from_keyword,
    //         get_articles_from_keyword
    //     ]
    // }
}

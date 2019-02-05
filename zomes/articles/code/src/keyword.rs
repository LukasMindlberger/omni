use hdk::{
    error::{
        ZomeApiResult, ZomeApiError
    },
    holochain_core_types::{
        entry::Entry,
        error::HolochainError,
        cas::content::Address,
        json::JsonString
    },
};
use holochain_wasm_utils::api_serialization::get_links::GetLinksResult;

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Keyword {
    keyword: String,
}

impl Keyword {
    pub fn keyword(&self) -> String {
        self.keyword.clone()
    }
}

fn create_keyword(keyword: Keyword) -> ZomeApiResult<Address> {
    let keyword_entry = Entry::App("keyword".into(), keyword.into());
    hdk::commit_entry(&keyword_entry)
}

pub fn get_keyword(keyword_addr: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&keyword_addr)
}

pub fn link_article_from_keyword(keyword: Keyword, article_addr: Address) -> Result<(), ZomeApiError> {
    let keyword_addr = create_keyword(keyword)?;

    hdk::link_entries(&article_addr, &keyword_addr, "article_from_keyword")
}

pub fn get_articles_from_keyword(keyword_addr: Address) -> ZomeApiResult<GetLinksResult> {
    hdk::get_links(&keyword_addr, "articles_from_keyword")
}

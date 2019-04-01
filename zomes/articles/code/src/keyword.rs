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

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Keyword {
    keyword: String,
}

impl Keyword {
    pub fn keyword(&self) -> String {
        self.keyword.clone()
    }
}

fn create_keyword(keyword: String) -> ZomeApiResult<Address> {
    let keyword_entry = Entry::App("keyword".into(), Keyword{
        keyword
    }.into());

    hdk::commit_entry(&keyword_entry)
}

pub fn get_keyword(keyword_addr: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&keyword_addr)
}

pub fn create_and_link_keyword_to_article(keyword: String, article_addr: Address) -> Result<(), ZomeApiError> {
    let keyword_addr = create_keyword(keyword)?;

    hdk::link_entries(&keyword_addr, &article_addr, "articles_from_keyword")
}

pub fn get_articles_from_keyword(keyword: String) -> ZomeApiResult<Vec<ZomeApiResult<Entry>>> {
    let keyword_entry = Entry::App("keyword".into(), Keyword{
        keyword
    }.into());

    let keyword_addr = hdk::entry_address(&keyword_entry)?;

    hdk::get_links_and_load(&keyword_addr, "articles_from_keyword")
}

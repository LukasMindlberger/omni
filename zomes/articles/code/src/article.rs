use hdk::{
    error::{
        ZomeApiResult
    },
    holochain_core_types::{
        entry::Entry,
        error::{
            HolochainError
        },
        cas::content::Address,
        json::JsonString,
    },
    AGENT_ADDRESS
};
use holochain_wasm_utils::api_serialization::{
    get_links::{
        GetLinksResult, GetLinksOptions
    }
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Article {
    title: String,
    abst: String,
    body: String,
}

impl Article {
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn abst(&self) -> String {
        self.abst.clone()
    }
    pub fn body(&self) -> String {
        self.body.clone()
    }
}

pub fn create_article(title: String, abst: String, body: String) -> ZomeApiResult<Address> {
    let article_entry = Entry::App("article".into(), Article{
        title, abst, body
    }.into());

    let article_addr = hdk::commit_entry(&article_entry)?;

    hdk::link_entries(&AGENT_ADDRESS, &article_addr, "articles_from_agent")?;

    Ok(article_addr)
}

pub fn get_article(article_addr: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&article_addr)
}

pub fn update_article(article_addr: Address, title: String, abst: String, body: String) -> ZomeApiResult<Address> {
    let article_entry = Entry::App("article".into(), Article{
        title, abst, body
    }.into());

    let new_addr = hdk::update_entry(article_entry, &article_addr)?;

    hdk::link_entries(&AGENT_ADDRESS, &new_addr, "articles_from_agent")?;

    Ok(new_addr)
}

pub fn delete_article(article_addr: Address) -> ZomeApiResult<()> {
    hdk::remove_entry(&article_addr)
}

pub fn get_authored_articles(agent_addr: Address) -> ZomeApiResult<GetLinksResult> {
    hdk::get_links(&agent_addr, "articles_from_agent")
}

pub fn get_my_articles() -> ZomeApiResult<GetLinksResult> {
    hdk::get_links(&AGENT_ADDRESS, "articles_from_agent")
}

use hdk::{
    error::{
        ZomeApiResult, ZomeApiError
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
use holochain_wasm_utils::api_serialization::get_links::GetLinksResult;

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Article {
    title: String,
    abst: String,
    body: String,
}

impl Article {
    pub fn new(title: &str, abst: &str, body: &str) -> Article {
        Article {
            title: title.to_owned(),
            abst: abst.to_owned(),
            body: body.to_owned()
        }
    }
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
    let article_entry = Entry::App("article".into(), Article::new(&title, &abst, &body).into());
    hdk::commit_entry(&article_entry)
}

pub fn get_article(article_addr: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&article_addr)
}

pub fn update_article(article_addr: Address, title: String, abst: String, body: String) -> ZomeApiResult<Address> {
    let article_entry = Entry::App("article".into(), Article::new(&title, &abst, &body).into());
    hdk::update_entry(article_entry, &article_addr)
}

pub fn delete_article(article_addr: Address) -> ZomeApiResult<()> {
    hdk::remove_entry(&article_addr)
}

pub fn author_article(article_addr: Address) -> Result<(), ZomeApiError> {
    hdk::link_entries(&AGENT_ADDRESS, &article_addr, "articles_from_agent")
}

pub fn get_authored_articles(agent_addr: Address) -> ZomeApiResult<GetLinksResult> {
    hdk::get_links(&agent_addr, "articles_from_agent")
}

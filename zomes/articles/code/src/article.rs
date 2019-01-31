use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        entry::Entry,
        error::HolochainError,
        cas::content::Address,
        json::JsonString,
    },
};
use holochain_wasm_utils::api_serialization::get_links::GetLinksResult;

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Article {
    title: String,
    abst: String,
    body: String,
    keywords: Vec<String>,
}

impl Article {
    pub fn new(title: &str, abst: &str, body: &str, keywords: Vec<String>) -> Article {
        Article {
            title: title.to_owned(),
            abst: abst.to_owned(),
            body: body.to_owned(),
            keywords: keywords,
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
    pub fn keywords(&self) -> Vec<String> {
        self.keywords.clone()
    }
}

pub fn create_article(article: Article) -> ZomeApiResult<Address> {
    let article_entry = Entry::App("article".into(), article.into());
    hdk::commit_entry(&article_entry)
}

pub fn get_article(article_addr: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&article_addr)
}

pub fn update_article(article_addr: Address, article: Article) -> ZomeApiResult<Address> {
    let article_entry = Entry::App("article".into(), article.into());
    hdk::update_entry(article_entry, &article_addr)
}

pub fn delete_article(article_addr: Address) -> ZomeApiResult<()> {
    hdk::remove_entry(&article_addr)
}

pub fn article_address(article: Article) -> ZomeApiResult<Address> {
    let article_entry = Entry::App("article".into(), article.into());
    hdk::entry_address(&article_entry)
}

pub fn link_author_to_article() {
    unimplemented!()
}

pub fn get_authored_articles(agent_addr: Address) -> ZomeApiResult<GetLinksResult> {
    hdk::get_links(&agent_addr, "articles_from_agent")
}

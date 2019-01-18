use hdk::{
    holochain_core_types::{
        entry::Entry,
        error::HolochainError,
        json::JsonString,
        cas::content::Address,
        },
    };
use hdk::error::ZomeApiResult;

// Data structs
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

// CRUD for zome
pub fn create_article(title: String, abst: String, body: String) -> ZomeApiResult<Address>  {
    let article_entry = Entry::App("article".into(), Article::new(&title, &abst, &body).into());

    hdk::commit_entry(&article_entry)
}

pub fn get_article(article_addr: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&article_addr)
}

pub fn delete_article(article_addr: Address) -> ZomeApiResult<()> {
    hdk::remove_entry(&article_addr)
}

use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        entry::Entry,
        error::HolochainError,
        json::JsonString,
        cas::content::Address,
        }
    };

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
pub fn create_article(title: String, abst: String, body: String) -> JsonString  {
    let article_entry = Entry::App("article".into(), Article::new(&title, &abst, &body).into());

    match hdk::commit_entry(&article_entry) {
        Ok(address) => json!({"success": true, "address": address}).into(),
        Err(hdk_err) => hdk_err.into()
    }
}

pub fn get_article(article_addr: Address) -> JsonString {
    match hdk::get_entry(&article_addr) {
        Ok(Some(entry)) => json!({"success": true, "entry": entry}).into(),
        Ok(None) => json!({"success": true, "entry": {}}).into(),
        Err(hdk_err) => hdk_err.into()
    }
}

pub fn update_article(article_addr: Address, title: String, abst: String, body: String) -> JsonString {
    let article_entry = Entry::App("article".into(), Article::new(&title, &abst, &body).into());

    match hdk::update_entry(article_entry, &article_addr) {
        Ok(address) => json!({"success": true, "address": address}).into(),
        Err(hdk_err) => hdk_err.into()
    }
}

pub fn delete_article(article_addr: Address) -> JsonString {
    match hdk::remove_entry(&article_addr) {
        Ok(_) => json!({ "success": true }).into(),
        Err(hdk_err) => hdk_err.into()
    }
}

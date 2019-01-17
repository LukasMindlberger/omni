use hdk::{
    holochain_core_types::{
        entry::Entry,
        error::HolochainError,
        json::JsonString,
        hash::HashString,
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
}

// CRUD for zome
pub fn handle_create_article(title: String, abst: String, body: String) -> JsonString  {
    let article_entry = Entry::App("article".into(), Article::new(&title, &abst, &body).into());

    match hdk::commit_entry(&article_entry) {
        Ok(article_addr) => json!({"success": true, "address": article_addr}).into(),
        Err(hdk_err) => hdk_err.into()
    }
}

pub fn handle_get_article(article_addr: HashString) -> JsonString {
    match hdk::get_entry(&article_addr) {
        Ok(Some(entry)) => entry.into(),
        Ok(None) => {}.into(),
        Err(hdk_err) => hdk_err.into()
    }
}

// pub fn handle_update_article(article: Article, article_addr: HashString) -> JsonString {
//     let article_entry = Entry::new(EntryType::App("article".into()), article);
//
//     match hdk::update_entry(&article_entry, article_addr) {
//         Ok(article_addr) => json!({"success": true, "address": article_addr}).into(),
//         Err(hdk_err) => hdk_err.into()
//     }
// }
//
// pub fn handle_delete_article(article_addr: HashString) -> JsonString {
//     match hdk::remove_entry(article_addr) {
//         Ok(_) => json!({"success": true}).into(),
//         Err(hdk_err) => hdk_err.into()
//     }
// }

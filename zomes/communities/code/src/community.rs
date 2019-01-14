use hdk::{
    holochain_core_types::{
        entry::Entry,
        error::HolochainError,
        json::JsonString,
        entry::entry_type::EntryType,
        hash::HashString,
    }
};

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct CommunityMeta {
    name: String,
    title: String,
    description: String,
    language: String,
    sidebar: String,
}

pub fn handle_create_community(community_name: String) -> JsonString {
    let new_community = Entry::new(EntryType::App("community".into()), community_name)

    match hdk::commit_entry(&new_community) {
        Ok(community_addr) => json!({"success": true, "address": community_addr}).into(),
        Err(hdk_err) => hdk_err.into()
    }
}

pub fn handle_get_community(community_name: String) -> JsonString {
    let community_entry = Entry::new(EntryType::App("community".into()), &community_name)

    let community_addr = hdk::entry_address(community_name)

    match community_addr {
        Ok(address) => hdk::get_entry(address) {
            Ok(Some(entry)) => entry.value().to_owned(),
            Ok(None) => {}.into(),
            Err(hdk_err) => hdk_err.into()
        },
        Err(hdk_err) => hdk_err.into()
    }
}

pub fn handle_add_community_metadata() {
    unimplemented!()
}

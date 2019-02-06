use hdk::{
    entry_definition::ValidatingEntryType,
    error::{
        ZomeApiResult,ZomeApiError
    },
    AGENT_ADDRESS
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry, dna::entry_types::Sharing, error::HolochainError, json::JsonString,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct NewUser {
    first_name: String,
    last_name: String,
    position: String,
    title: String,
    orcid: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct User {
    first_name: String,
    last_name: String,
    position: String,
    title: String,
    orcid: String,
    agent_addr: Address,
}

impl NewUser {
    pub fn new(&self) -> User {
        User {
            first_name: self.first_name.to_owned(),
            last_name: self.last_name.to_owned(),
            position: self.position.to_owned(),
            title: self.title.to_owned(),
            orcid: self.orcid.to_owned(),
            agent_addr: AGENT_ADDRESS.clone()
        }
    }
}

pub fn create_user(user: NewUser) -> ZomeApiResult<Address> {
    let entry = Entry::App("user".into(), NewUser::new(&user).into());
    hdk::commit_entry(&entry)
}

pub fn get_user(user_addr: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&user_addr)
}

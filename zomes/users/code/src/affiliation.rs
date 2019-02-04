use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry, dna::entry_types::Sharing, error::HolochainError, json::JsonString,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Org {
    entity: String,
    country: String,
}

impl Org {
    pub fn entity(&self) -> String {
        self.entity.clone()
    }
    pub fn country(&self) -> String {
        self.country.clone()
    }
}

pub fn create_organisation(org: Org) -> ZomeApiResult<Address> {
    let org_entry = Entry::App("org".into(), org.into());
    hdk::commit_entry(&org_entry)
}

pub fn get_organisation(org_addr: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&org_addr)
}

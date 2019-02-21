use hdk::{
    error::{
        ZomeApiResult
    },
    holochain_core_types::{
        cas::content::Address,
        json::JsonString,
        error::HolochainError,
        time::Timeout
    },
    AGENT_ADDRESS,
    AGENT_ID_STR,
    DNA_NAME,
    DNA_ADDRESS
};
use holochain_wasm_utils::api_serialization::{
    get_entry::{
        GetEntryResult, GetEntryOptions, StatusRequestKind
    },
};

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Env {
    dna_name: String,
    dna_address: String,
    agent_id: String,
    agent_address: String,
}

pub fn handle_show_env() -> ZomeApiResult<Env> {
    Ok(Env{
        dna_name: DNA_NAME.to_string(),
        dna_address: DNA_ADDRESS.to_string(),
        agent_id: AGENT_ID_STR.to_string(),
        agent_address: AGENT_ADDRESS.to_string(),
    })
}

pub fn handle_get_meta_latest(address: Address) -> ZomeApiResult<GetEntryResult> {
    let options = GetEntryOptions{status_request: StatusRequestKind::Latest, entry: false, headers: false, timeout: Timeout::new(10000)};

    hdk::get_entry_result(&address, options)
}

pub fn handle_get_meta_initial(address: Address) -> ZomeApiResult<GetEntryResult> {
    let options = GetEntryOptions{status_request: StatusRequestKind::Initial, entry: false, headers: false, timeout: Timeout::new(10000)};

    hdk::get_entry_result(&address, options)
}

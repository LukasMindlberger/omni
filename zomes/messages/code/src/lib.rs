#![feature(try_from)]

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use hdk::holochain_core_types::{
    cas::content::Address,
    json::JsonString,
};

fn handle_send_message(to_agent: Address, message: String) -> JsonString  {
    match hdk::send(to_agent, message) {
        Ok(response) => response.into(),
        Err(hdk_err) => hdk_err.into(),
    }
}

define_zome! {
    entries: []

    genesis: || { Ok(()) }

    receive: |payload| {
        format!("Received: {}", payload)
    }

    functions: {
        main (Public) {
            send_message: {
                inputs: |to_agent: Address, message: String|,
                outputs: |response: JsonString|,
                handler: handle_send_message
            }
        }
    }
}

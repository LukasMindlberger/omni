// Messages zome
#![feature(try_from)]

#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate holochain_core_types_derive;

use hdk::holochain_core_types::{
    cas::content::Address,
    json::JsonString,
};

// Direct node-to-node message for when peer is online only
fn handle_send_message(to_agent: Address, message: String) -> JsonString {
    match hdk::send(to_agent, message) {
        Ok(result) => json!({"success": true, "payload": result}).into(),
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
                outputs: |result: JsonString|,
                handler: handle_send_message
            }
        }
    }

    // capabilities: {
    //     public (Public) [send_message]
    // }
}

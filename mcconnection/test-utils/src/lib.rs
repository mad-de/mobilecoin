// Copyright (c) 2018-2020 MobileCoin Inc.

//! Connection mock and test utilities

use mcuri::ConsensusClientUri;
use std::str::FromStr;

mod blockchain;
mod user_tx;

pub fn test_client_uri(node_id: u32) -> ConsensusClientUri {
    ConsensusClientUri::from_str(&format!("mc://node{}.test.com/", node_id))
        .expect("Could not construct client uri from string")
}

pub use self::{blockchain::MockBlockchainConnection, user_tx::MockUserTxConnection};

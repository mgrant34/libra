// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use client::{client_proxy::ClientProxy, commands::*};
use logger::set_default_global_logger;
use rustyline::{config::CompletionType, error::ReadlineError, Config, Editor};
use std::option;

#[allow(clippy::new_without_default)]
pub fn new() -> std::io::Result<()> {


    let mut client_proxy = ClientProxy::new(
        "ac.testnet.libra.org",
        "8000",
        "trusted_peers.config.toml",
        "",
        false,
        Some("".to_string()),
        Some("".to_string()),
    ).unwrap();

    // Test connection to validator
    let test_ret = client_proxy.test_validator_connection();

    Ok(())
}

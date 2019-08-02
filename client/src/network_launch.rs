// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use client::{client_proxy::ClientProxy, commands::*};
use logger::set_default_global_logger;
use rustyline::{config::CompletionType, error::ReadlineError, Config, Editor};
use std::option;
use std::env;

#[allow(clippy::new_without_default)]
pub fn new() -> std::io::Result<()> {

    let path = env::current_dir()?;
    let static_files = format!("{}/client/src/trusted_peers.config.toml", path.display());
    let mut client_proxy = ClientProxy::new(
        "ac.testnet.libra.org",
        "8000",
        &static_files,
        "",
        false,
        Some("".to_string()),
        Some("".to_string()),
    ).unwrap();

    // Test connection to validator
    let test_ret = client_proxy.test_validator_connection();

    Ok(())
}

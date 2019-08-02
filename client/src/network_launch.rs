// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use client::{client_proxy::ClientProxy, commands::*};
use logger::set_default_global_logger;
use rustyline::{config::CompletionType, error::ReadlineError, Config, Editor};
use std::option;
use std::env;

#[allow(clippy::new_without_default)]
pub fn new() -> std::io::Result<()> {

    println!("tst 1");
    let path = env::current_dir()?;
    let static_files = format!("{}/scripts/cli/trusted_peers.config.toml", path.display());
    println!("{}", static_files);
    let mut client_proxy = ClientProxy::new(
        "ac.testnet.libra.org",
        "8000",
        &static_files,
        "",
        false,
        None,
        None,
    ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, &format!("{}", e)[..]))?;

    println!("tst 2");
    // Test connection to validator
    let test_ret = client_proxy.test_validator_connection();

    println!("tst 3");
    if let Err(e) = test_ret {
        println!(
            "Not able to connect to network launch validator"
        );
        return Ok(());
    }

    let cli_info = format!("Connected to network launch validator at");
    println!("{}", cli_info);


    Ok(())
}

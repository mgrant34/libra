// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use client::{client_proxy::ClientProxy, commands::*};
use logger::set_default_global_logger;
use rustyline::{config::CompletionType, error::ReadlineError, Config, Editor};
use std::option;
use std::env;

#[allow(clippy::new_without_default)]
pub fn new() -> Option<ClientProxy> {

    println!("tst 1");
    let currentDir;
    match env::current_dir() {
        Ok(v) => currentDir = v,
        Err(e) => return None
    }
    let static_files = format!("{}/scripts/cli/trusted_peers.config.toml", currentDir.display());
    println!("{}", static_files);
    let client_proxy;
    let client_proxy_optional = ClientProxy::new(
        "ac.testnet.libra.org",
        "8000",
        &static_files,
        "",
        false,
        None,
        None,
    ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, &format!("{}", e)[..]));
    match client_proxy_optional{
        Ok(v) => client_proxy = v,
        Err(e) => return None
    }

    println!("tst 2");
    // Test connection to validator
    let test_ret = client_proxy.test_validator_connection();

    println!("tst 3");
    if let Err(e) = test_ret {
        println!(
            "Not able to connect to network launch validator"
        );
        return None;
    }

    let cli_info = format!("Connected to network launch validator at");
    println!("{}", cli_info);
    Some(client_proxy)
}

pub fn executeCommand(
    mut clientProxy: ClientProxy
) {

    let proxy = &mut clientProxy;
    let address;
    match proxy.create_next_account(true) {
        Ok(account_data) =>  {
        address = account_data.index.to_owned();
        println!(
            "Created/retrieved account #{} address {}",
            account_data.index,
            hex::encode(account_data.address)
        );
        },
        Err(e) => {
            report_error("Error creating account", e);
            return;
        }
    }

    // let my_string = String::from(address);
    // let my_immutable_string = &my_string;
    // let my_str: &str = &my_string;

    let coins: &str = "100";
    let index: &str = "0";
    let params = &["mint", index, coins];
    match proxy.mint_coins(params, true) {
            Ok(_) => {
                println!("Finished minting!");
            }
            Err(e) => report_error("Error minting coins", e),
        }
}

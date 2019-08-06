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
    let accountData;
    match proxy.create_next_account(true) {
        Ok(account_data) =>  {
        accountData = account_data;
        println!(
            "Created/retrieved account #{} address {}",
            account_data.index,
            hex::encode(account_data.address)
        );
        },
        Err(e) => report_error("Error creating account", e),
    }

    let address = &mut accountData.address.short_str();
    let my_string = String::from(address);
    let my_immutable_string = &my_string; //This is a &String type
    let my_str: &str = &my_string;

    let coins: &str = "100.00";
    let params = &["mint", my_str, coins];
    match proxy.mint_coins(params, true) {
            Ok(_) => {
                println!("Finished minting!");
            }
            Err(e) => report_error("Error minting coins", e),
        }

    // let (commands, alias_to_cmd) = get_commands(false);
    // match alias_to_cmd.get(&params[0]) {
    //     Some(cmd) => {
    //         cmd.execute(proxy, &params);
    //     },
    //     None => match params[0] {
    //         "quit" | "q!" => println!("q"),
    //         "help" | "h" => println!("help"),
    //         "" => println!("empty command"),
    //         x => println!("Unknown command: {:?}", x),
    //     }
    // }
}

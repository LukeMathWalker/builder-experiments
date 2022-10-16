/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

// This program is exported as a binary named `pokemon-service`.
use std::{net::SocketAddr, sync::Arc};

use aws_smithy_http_server::AddExtensionLayer;
use clap::Parser;
use pokemon_service::plugin::PrintPlugin;
use pokemon_service::{
    capture_pokemon, do_nothing, get_pokemon_species, get_server_statistics, get_storage,
    setup_tracing, State,
};
use pokemon_service_server_sdk::service::PokemonService;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Hyper server bind address.
    #[clap(short, long, action, default_value = "127.0.0.1")]
    address: String,
    /// Hyper server bind port.
    #[clap(short, long, action, default_value = "13734")]
    port: u16,
}

async fn run_server(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let app = PokemonService::builder(PrintPlugin)
        .get_pokemon_species(get_pokemon_species)
        .get_storage(get_storage)
        .get_server_statistics(get_server_statistics)
        .capture_pokemon(capture_pokemon)
        .do_nothing(do_nothing)
        // .check_health(check_health)
        .build()?;

    // Setup shared state and middlewares.
    let shared_state = Arc::new(State::default());
    let app = app.layer(&AddExtensionLayer::new(shared_state));

    // Start the [`hyper::Server`].
    let bind: SocketAddr = format!("{}:{}", args.address, args.port)
        .parse()
        .expect("unable to parse the server bind address and port");
    let server = hyper::Server::bind(&bind).serve(app.into_make_service());

    // Run forever-ish...
    server.await?;
    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    setup_tracing();
    if let Err(e) = run_server(args).await {
        tracing::error!("{}", e);
    };
    Ok(())
}

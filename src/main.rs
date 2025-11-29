/*
 * OLG Server - OnLine Gaming Server Management Tool
 * Copyright (C) 2025 Luca Cireddu <sardylan@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free Software
 * Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program. If not, see <https://www.gnu.org/licenses/>.
 *
 */

use crate::server::CodServer;
use clap::FromArgMatches;
use tracing::info;

mod cli;
mod config;
mod db;
mod error;
mod http;
mod log;
mod maps;
mod server;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ui::header();

    info!("Parsing command line arguments");
    let matches = cli::generate_matches().get_matches();
    let cli = cli::Cli::from_arg_matches(&matches)?;

    info!("Parsing configuration file");
    let configuration = config::parse(&cli.config_file).await?;

    info!("Configuring logging");
    log::configure(cli.log_level);

    info!("Creating database pool");
    let db_pool = db::create_pool(
        &configuration.db_host,
        configuration.db_port,
        Some(configuration.db_user),
        Some(configuration.db_password),
        &configuration.db_name,
    )?;

    info!("Creating server");
    let cod_server = CodServer::new(
        &configuration.server_host,
        configuration.server_port,
        &configuration.server_rconpassword,
    );

    tokio::select! {
        result = http::run(
            &configuration.http_bind_host,
            configuration.http_bind_port,
            db_pool,
            cod_server
        ) => result?,
        result = shutdown_signal() => result?,
    }

    Ok(())
}

async fn shutdown_signal() -> anyhow::Result<()> {
    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received. Exiting...");
    Ok(())
}

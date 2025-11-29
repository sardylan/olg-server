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
use std::env;
use tracing::{Level, info};

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

    let log_level: Level = env::var("LOG_LEVEL")
        .unwrap_or_default()
        .parse()
        .unwrap_or_else(|_| Level::WARN);
    log::configure(log_level);

    info!("Parsing configuration file");
    let configuration = config::parse().await?;

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

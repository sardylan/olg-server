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

use std::env;
use tracing::{info, instrument};

pub(crate) struct Configuration {
    pub(crate) http_bind_host: String,
    pub(crate) http_bind_port: u16,
    pub(crate) server_host: String,
    pub(crate) server_port: u16,
    pub(crate) server_rconpassword: String,
    pub(crate) db_host: String,
    pub(crate) db_port: u16,
    pub(crate) db_user: String,
    pub(crate) db_password: String,
    pub(crate) db_name: String,
}

#[instrument]
pub async fn parse() -> anyhow::Result<Configuration> {
    info!("Loading configuration");

    Ok(Configuration {
        http_bind_host: env::var("HTTP_BIND_HOST").unwrap_or_else(|_| "::".to_string()),
        http_bind_port: env::var("HTTP_BIND_PORT")
            .unwrap_or_else(|_| "7000".to_string())
            .parse::<u16>()?,
        server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        server_port: env::var("SERVER_PORT")
            .unwrap_or_else(|_| "28960".to_string())
            .parse::<u16>()?,
        server_rconpassword: env::var("SERVER_RCONPASSWORD")
            .unwrap_or_else(|_| "127.0.0.1".to_string()),
        db_host: env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        db_port: env::var("DB_PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse::<u16>()?,
        db_user: env::var("DB_USER").unwrap_or_else(|_| "olg".to_string()),
        db_password: env::var("DB_PASSWORD").unwrap_or_else(|_| "olg".to_string()),
        db_name: env::var("DB_NAME").unwrap_or_else(|_| "olg".to_string()),
    })
}

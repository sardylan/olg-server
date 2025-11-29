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

use serde::Deserialize;
use std::path::PathBuf;
use toml::Value;
use tracing::{info, instrument};

#[derive(Debug, Deserialize)]
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
pub async fn parse(file_path: &PathBuf) -> anyhow::Result<Configuration> {
    info!("Loading configuration");

    let file_content = tokio::fs::read_to_string(file_path).await?;
    let file: Value = toml::from_str(&file_content)?;

    Ok(Configuration {
        http_bind_host: file
            .get("http")
            .and_then(|x| x.get("bind"))
            .and_then(|x| x.get("host"))
            .and_then(|x| x.as_str())
            .unwrap_or("::")
            .to_string(),
        http_bind_port: file
            .get("http")
            .and_then(|x| x.get("bind"))
            .and_then(|x| x.get("port"))
            .and_then(|x| x.as_integer())
            .unwrap_or(7000) as u16,
        server_host: file
            .get("server")
            .and_then(|x| x.get("host"))
            .and_then(|x| x.as_str())
            .unwrap_or("127.0.0.1")
            .to_string(),
        server_port: file
            .get("server")
            .and_then(|x| x.get("port"))
            .and_then(|x| x.as_integer())
            .unwrap_or(28960) as u16,
        server_rconpassword: file
            .get("server")
            .and_then(|x| x.get("rconpassword"))
            .and_then(|x| x.as_str())
            .unwrap_or("password")
            .to_string(),
        db_host: file
            .get("db")
            .and_then(|x| x.get("host"))
            .and_then(|x| x.as_str())
            .unwrap_or("localhost")
            .to_string(),
        db_port: file
            .get("db")
            .and_then(|x| x.get("port"))
            .and_then(|x| x.as_integer())
            .unwrap_or(5432) as u16,
        db_user: file
            .get("db")
            .and_then(|x| x.get("user"))
            .and_then(|x| x.as_str())
            .unwrap_or("postgres")
            .to_string(),
        db_password: file
            .get("db")
            .and_then(|x| x.get("password"))
            .and_then(|x| x.as_str())
            .unwrap_or("password")
            .to_string(),
        db_name: file
            .get("db")
            .and_then(|x| x.get("name"))
            .and_then(|x| x.as_str())
            .unwrap_or("olg")
            .to_string(),
    })
}

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

use crate::maps::Map;
use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

pub(crate) type DBClient = deadpool_postgres::Client;
pub(crate) type DBPool = Pool;

pub(crate) fn create_pool(
    host: &str,
    port: u16,
    user: Option<String>,
    password: Option<String>,
    name: &str,
) -> anyhow::Result<DBPool> {
    let mut pg_cfg = Config::new();
    pg_cfg.host = Some(host.to_string());
    pg_cfg.port = Some(port);
    pg_cfg.user = user;
    pg_cfg.password = password;
    pg_cfg.dbname = Some(name.to_string());
    Ok(pg_cfg.create_pool(Some(Runtime::Tokio1), NoTls)?)
}

pub(crate) async fn get_active_maps(db_client: &DBClient) -> anyhow::Result<Vec<Map>> {
    let stmt = db_client
        .prepare(
            "SELECT m.tag AS tag, m.name AS name FROM codmap m WHERE m.active = TRUE ORDER BY name",
        )
        .await?;

    Ok(db_client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Map::new(row.get(0), row.get(1)))
        .collect::<Vec<Map>>())
}

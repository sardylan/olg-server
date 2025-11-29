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

use crate::db::{self, DBPool};
use crate::error::ApiError;
use crate::models::Gametype;
use crate::server::CodServer;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};

async fn maps(db_pool: Data<DBPool>) -> Result<HttpResponse, ApiError> {
    let db_client = db_pool.get().await?;
    let maps = db::get_active_maps(&db_client).await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(maps))
}

#[derive(Deserialize, Serialize)]
struct GametypeMapRequest {
    gametype: Gametype,
    map: String,
}

async fn map_restart(cod_server: Data<CodServer>) -> Result<HttpResponse, ApiError> {
    cod_server.map_restart().await?;
    Ok(HttpResponse::NoContent().finish())
}

async fn fast_restart(cod_server: Data<CodServer>) -> Result<HttpResponse, ApiError> {
    cod_server.fast_restart().await?;
    Ok(HttpResponse::NoContent().finish())
}

async fn gametype_map(
    cod_server: Data<CodServer>,
    request_body: web::Json<GametypeMapRequest>,
) -> Result<HttpResponse, ApiError> {
    cod_server
        .gametype_map(&request_body.gametype, &request_body.map)
        .await?;
    Ok(HttpResponse::NoContent().finish())
}

async fn health() -> impl Responder {
    HttpResponse::NoContent().finish()
}

pub async fn run(
    http_host: &str,
    http_port: u16,
    db_pool: DBPool,
    cod_server: CodServer,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(cod_server.clone()))
            .wrap(Logger::default())
            .route("/health", web::get().to(health))
            .route("/api/public/v1/maps", web::get().to(maps))
            .route(
                "/api/public/v1/server/map_restart",
                web::get().to(map_restart),
            )
            .route(
                "/api/public/v1/server/fast_restart",
                web::get().to(fast_restart),
            )
            .route(
                "/api/public/v1/server/gametype_map",
                web::post().to(gametype_map),
            )
    })
    .bind((http_host, http_port))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::App;
    use actix_web::test::{TestRequest, call_service, init_service};

    #[actix_web::test]
    async fn test_map_restart() {
        let mock_server = crate::server::tests::MockCodServer::new().await;
        let server = CodServer::new("127.0.0.1", mock_server.port(), "test_password");
        let app = init_service(App::new().app_data(Data::new(server)).route(
            "/api/public/v1/server/map_restart",
            web::get().to(map_restart),
        ))
        .await;
        let req = TestRequest::get()
            .uri("/api/public/v1/server/map_restart")
            .to_request();
        let resp = call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(mock_server.payload_size().await, 1);
    }

    #[actix_web::test]
    async fn test_fast_restart() {
        let mock_server = crate::server::tests::MockCodServer::new().await;
        let server = CodServer::new("127.0.0.1", mock_server.port(), "test_password");
        let app = init_service(App::new().app_data(Data::new(server)).route(
            "/api/public/v1/server/fast_restart",
            web::get().to(fast_restart),
        ))
        .await;
        let req = TestRequest::get()
            .uri("/api/public/v1/server/fast_restart")
            .to_request();
        let resp = call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(mock_server.payload_size().await, 1);
    }

    #[actix_web::test]
    async fn test_gametype_map() {
        let mock_server = crate::server::tests::MockCodServer::new().await;
        let server = CodServer::new("127.0.0.1", mock_server.port(), "test_password");
        let app = init_service(App::new().app_data(Data::new(server)).route(
            "/api/public/v1/server/gametype_map",
            web::post().to(gametype_map),
        ))
        .await;
        let req = TestRequest::post()
            .uri("/api/public/v1/server/gametype_map")
            .set_json(&GametypeMapRequest {
                gametype: Gametype::SearchAndDestroy,
                map: "mp_crash".to_string(),
            })
            .to_request();
        let resp = call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(mock_server.payload_size().await, 2);
    }

    #[actix_web::test]
    async fn test_health() {
        let app = init_service(App::new().route("/health", web::get().to(health))).await;
        let req = TestRequest::get().uri("/health").to_request();
        let resp = call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}

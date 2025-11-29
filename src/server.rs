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

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use tokio::net::UdpSocket;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum Gametype {
    FreeForAll,
    TeamDeathmatch,
    Domination,
    SearchAndDestroy,
    Headquarters,
    Sabotage,
}

impl FromStr for Gametype {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Gametype::from_tag(s).ok_or(())
    }
}

impl Gametype {
    pub(crate) fn to_tag(&self) -> &str {
        match self {
            Gametype::FreeForAll => "dm",
            Gametype::TeamDeathmatch => "war",
            Gametype::Domination => "dom",
            Gametype::SearchAndDestroy => "sd",
            Gametype::Headquarters => "koth",
            Gametype::Sabotage => "sab",
        }
    }

    pub(crate) fn from_tag(tag: &str) -> Option<Self> {
        match tag {
            "dm" => Some(Gametype::FreeForAll),
            "war" => Some(Gametype::TeamDeathmatch),
            "dom" => Some(Gametype::Domination),
            "sd" => Some(Gametype::SearchAndDestroy),
            "koth" => Some(Gametype::Headquarters),
            "sab" => Some(Gametype::Sabotage),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CodServer {
    host: String,
    port: u16,
    rcon_password: String,
}

impl Display for CodServer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

impl CodServer {
    pub fn new(host: &str, port: u16, rcon_password: &str) -> Self {
        Self {
            host: host.to_string(),
            port,
            rcon_password: rcon_password.to_string(),
        }
    }

    pub async fn map_restart(&self) -> anyhow::Result<()> {
        self.rcon("map_restart").await?;
        Ok(())
    }

    pub async fn fast_restart(&self) -> anyhow::Result<()> {
        self.rcon("fast_restart").await?;
        Ok(())
    }

    pub async fn gametype_map(&self, gametype: &Gametype, map: &str) -> anyhow::Result<()> {
        let cmd = format!("g_gametype {}", gametype.to_tag());
        self.rcon(&cmd).await?;

        let cmd = format!("map {}", map);
        self.rcon(&cmd).await?;

        Ok(())
    }

    async fn rcon(&self, command: &str) -> anyhow::Result<String> {
        let rcon_command = format!("rcon {} {}", self.rcon_password, command);
        self.send(&rcon_command).await
    }

    async fn send(&self, command: &str) -> anyhow::Result<String> {
        let mut payload = vec![0xff, 0xff, 0xff, 0xff];
        payload.extend_from_slice(command.as_bytes());

        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        let server_addr = format!("{}:{}", self.host, self.port);
        socket.send_to(&payload, &server_addr).await?;

        let mut buf = [0; 4096];
        let (len, _) = socket.recv_from(&mut buf).await?;

        if len > 4 && &buf[..4] == b"\xff\xff\xff\xff" {
            let response_data = &buf[4..len];
            let response = if response_data.starts_with(b"\n") {
                &response_data[1..]
            } else {
                response_data
            };
            Ok(String::from_utf8_lossy(response).to_string())
        } else {
            Err(anyhow::anyhow!("Invalid or empty response from server"))
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    pub(crate) struct MockCodServer {
        socket: Arc<UdpSocket>,
        payloads: Arc<RwLock<VecDeque<Vec<u8>>>>,
    }

    impl MockCodServer {
        pub async fn new() -> Self {
            let socket = Arc::new(UdpSocket::bind("127.0.0.1:0").await.unwrap());
            let payloads: Arc<RwLock<VecDeque<Vec<u8>>>> = Arc::new(RwLock::new(VecDeque::new()));

            tokio::spawn({
                let socket = socket.clone();
                let payloads = payloads.clone();
                async move {
                    let mut buf = [0u8; 4096];
                    loop {
                        let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
                        let payload = &buf[..len];
                        payloads.write().await.push_back(payload.to_vec());
                        socket.send_to(payload, &addr).await.unwrap();
                    }
                }
            });

            Self { socket, payloads }
        }

        pub fn port(&self) -> u16 {
            self.socket.local_addr().unwrap().port()
        }

        pub async fn payload_size(&self) -> usize {
            self.payloads.read().await.len()
        }

        pub async fn read_payload(&self) -> Option<Vec<u8>> {
            self.payloads.write().await.pop_front()
        }
    }

    #[test]
    fn test_new() {
        let server = CodServer::new("localhost", 28960, "my_rcon_password");
        assert_eq!(server.host, "localhost");
        assert_eq!(server.port, 28960);
        assert_eq!(server.rcon_password, "my_rcon_password");
    }

    #[tokio::test]
    async fn test_send() -> anyhow::Result<()> {
        let mock_server = MockCodServer::new().await;
        let server = CodServer::new("127.0.0.1", mock_server.port(), "test_password");
        let response = server.send("test_command").await?;
        assert_eq!(response, "test_command");
        Ok(())
    }

    #[tokio::test]
    async fn test_rcon() -> anyhow::Result<()> {
        let mock_server = MockCodServer::new().await;
        let server = CodServer::new("127.0.0.1", mock_server.port(), "test_password");
        let response = server.rcon("test_command").await?;
        assert_eq!(response, "rcon test_password test_command");
        Ok(())
    }

    #[tokio::test]
    async fn test_map_restart() -> anyhow::Result<()> {
        let mock_server = MockCodServer::new().await;
        CodServer::new("127.0.0.1", mock_server.port(), "test_password")
            .map_restart()
            .await?;
        assert_eq!(mock_server.payload_size().await, 1);
        assert_eq!(
            mock_server.read_payload().await,
            Some(b"\xff\xff\xff\xffrcon test_password map_restart".to_vec())
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_fast_restart() -> anyhow::Result<()> {
        let mock_server = MockCodServer::new().await;
        CodServer::new("127.0.0.1", mock_server.port(), "test_password")
            .fast_restart()
            .await?;
        assert_eq!(mock_server.payload_size().await, 1);
        assert_eq!(
            mock_server.read_payload().await,
            Some(b"\xff\xff\xff\xffrcon test_password fast_restart".to_vec())
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_gametype_map() -> anyhow::Result<()> {
        let mock_server = MockCodServer::new().await;
        CodServer::new("127.0.0.1", mock_server.port(), "test_password")
            .gametype_map(&Gametype::SearchAndDestroy, "mp_crash")
            .await?;
        assert_eq!(mock_server.payload_size().await, 2);
        assert_eq!(
            mock_server.read_payload().await,
            Some(b"\xff\xff\xff\xffrcon test_password g_gametype sd".to_vec())
        );
        assert_eq!(
            mock_server.read_payload().await,
            Some(b"\xff\xff\xff\xffrcon test_password map mp_crash".to_vec())
        );
        Ok(())
    }
}

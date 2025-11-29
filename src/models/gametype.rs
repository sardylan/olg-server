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
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug)]
pub struct InvalidGametype(String);

impl std::fmt::Display for InvalidGametype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid gametype: {}", self.0)
    }
}

impl std::error::Error for InvalidGametype {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(try_from = "String", into = "String")]
pub enum Gametype {
    FreeForAll,
    TeamDeathmatch,
    Domination,
    SearchAndDestroy,
    Headquarters,
    Sabotage,
}

impl FromStr for Gametype {
    type Err = InvalidGametype;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Gametype::from_tag(s).ok_or_else(|| InvalidGametype(s.to_string()))
    }
}

impl From<Gametype> for String {
    fn from(gt: Gametype) -> Self {
        gt.to_tag().to_string()
    }
}

impl TryFrom<String> for Gametype {
    type Error = InvalidGametype;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Gametype::from_tag(&s).ok_or_else(|| InvalidGametype(s))
    }
}

impl Gametype {
    pub fn to_tag(&self) -> &str {
        match self {
            Gametype::FreeForAll => "dm",
            Gametype::TeamDeathmatch => "war",
            Gametype::Domination => "dom",
            Gametype::SearchAndDestroy => "sd",
            Gametype::Headquarters => "koth",
            Gametype::Sabotage => "sab",
        }
    }

    pub fn from_tag(tag: &str) -> Option<Self> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_gametype_to_tag() {
        assert_eq!(Gametype::FreeForAll.to_tag(), "dm");
        assert_eq!(Gametype::TeamDeathmatch.to_tag(), "war");
        assert_eq!(Gametype::Domination.to_tag(), "dom");
        assert_eq!(Gametype::SearchAndDestroy.to_tag(), "sd");
        assert_eq!(Gametype::Headquarters.to_tag(), "koth");
        assert_eq!(Gametype::Sabotage.to_tag(), "sab");
    }

    #[test]
    fn test_gametype_from_tag() {
        assert_eq!(Gametype::from_tag("dm"), Some(Gametype::FreeForAll));
        assert_eq!(Gametype::from_tag("war"), Some(Gametype::TeamDeathmatch));
        assert_eq!(Gametype::from_tag("dom"), Some(Gametype::Domination));
        assert_eq!(Gametype::from_tag("sd"), Some(Gametype::SearchAndDestroy));
        assert_eq!(Gametype::from_tag("koth"), Some(Gametype::Headquarters));
        assert_eq!(Gametype::from_tag("sab"), Some(Gametype::Sabotage));
        assert_eq!(Gametype::from_tag("invalid"), None);
    }

    #[test]
    fn test_serde_serialization_deserialization() {
        let gametype = Gametype::FreeForAll;
        let serialized = serde_json::to_string(&gametype).unwrap();
        assert_eq!(serialized, "\"dm\"");

        let deserialized: Gametype = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, gametype);

        let gametype = Gametype::TeamDeathmatch;
        let serialized = serde_json::to_string(&gametype).unwrap();
        assert_eq!(serialized, "\"war\"");

        let deserialized: Gametype = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, gametype);

        let invalid_json = "\"invalid\"";
        let deserialized_result: Result<Gametype, _> = serde_json::from_str(invalid_json);
        assert!(deserialized_result.is_err());
    }
}
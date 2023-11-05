use fusion_datatypes::*;
use serde::{Deserialize, Serialize};

fn main() {
    let raw_pokemon: Vec<RawPokemon> =
        serde_json::from_str(include_str!("infinite_dex.json")).unwrap();
    let pokemon = raw_pokemon
        .into_iter()
        .map(Pokemon::from)
        .collect::<Vec<_>>();
    let bytes = bincode::serialize(&pokemon).unwrap();
    if std::path::Path::new("infinite_dex.bin").exists() {
        let existing_bytes = std::fs::read("infinite_dex.bin").unwrap();
        if existing_bytes != bytes {
            std::fs::write("infinite_dex.bin", bytes).unwrap();
        }
    } else {
        std::fs::write("infinite_dex.bin", bytes).unwrap();
    }
}

impl From<RawPokemon> for Pokemon {
    fn from(raw: RawPokemon) -> Self {
        Pokemon {
            id: raw.id,
            name: raw.name.english,
            hp: raw.base.hp,
            attack: raw.base.attack,
            defense: raw.base.defense,
            special_attack: raw.base.special_attack,
            special_defense: raw.base.special_defense,
            speed: raw.base.speed,
            primary_type: raw.types[0],
            secondary_type: raw.types.get(1).copied(),
            fusion_type_override: raw.fusion_type_override,
            abilities: raw.profile.ability,
        }
    }
}

#[derive(Deserialize, Debug)]
struct RawPokemon {
    pub id: u16,
    pub name: RawNames,
    #[serde(rename = "type")]
    pub types: Vec<Type>,
    pub base: RawStats,
    pub species: String,
    pub description: String,
    pub evolution: RawEvolutions,
    pub profile: RawProfile,
    pub image: RawImage,
    pub fusion_type_override: Option<Type>,
}

#[derive(Deserialize, Debug)]
struct RawNames {
    pub english: String,
    pub japanese: String,
    pub chinese: String,
    pub french: String,
}

#[derive(Deserialize, Debug)]
struct RawStats {
    #[serde(rename = "HP")]
    pub hp: u8,
    #[serde(rename = "Attack")]
    pub attack: u8,
    #[serde(rename = "Defense")]
    pub defense: u8,
    #[serde(rename = "Sp. Attack")]
    pub special_attack: u8,
    #[serde(rename = "Sp. Defense")]
    pub special_defense: u8,
    #[serde(rename = "Speed")]
    pub speed: u8,
}

#[derive(Deserialize, Debug)]
struct RawProfile {
    pub height: String,
    pub weight: String,
    pub egg: Vec<String>,
    pub ability: Vec<(String, bool)>,
    pub gender: String,
}

#[derive(Deserialize, Debug)]
struct RawImage {
    pub sprite: String,
    pub thumbnail: String,
    pub hires: String,
}

#[derive(Deserialize, Debug)]
struct RawEvolutions {
    pub prev: Option<(String, String)>,
    pub next: Option<Vec<(String, String)>>,
}

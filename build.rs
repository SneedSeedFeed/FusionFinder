use std::path::Path;

use build_const::*;
use fusion_datatypes::{*, pokemon::Ability};
use indexmap::IndexSet;
use serde::Deserialize;

fn main() {
    // Generate dex
    let raw_pokemon: Vec<RawPokemon> =
        serde_json::from_str(include_str!("infinite_dex.json")).unwrap();
    let pokemon = raw_pokemon
        .into_iter()
        .map(Pokemon::from)
        .collect::<Vec<_>>();
    let pokemon_bytes = bincode::serialize(&pokemon).unwrap();

    let number_of_pokemon = pokemon.len();
    let number_of_fusions = (number_of_pokemon * 2) - 1;
    create_if_not_exists("infinite_dex.bin", pokemon_bytes);

    // Hash all the abilities
    let mut abilities_set: IndexSet<String> = IndexSet::new();
    for pokemon in &pokemon {
        for ability in &pokemon.abilities {
            abilities_set.insert(ability.0.clone());
        }
    }

    // Put them in order
    let mut abilities = abilities_set.into_iter().collect::<Vec<_>>();
    abilities.sort();

    // Re-hash them
    let abilities_set = abilities
        .iter()
        .map(|s| s.as_str())
        .collect::<IndexSet<_>>();
    let abilities_bytes = bincode::serialize(&abilities_set).unwrap();
    create_if_not_exists("abilities.bin", abilities_bytes);

    // Define constants
    let consts = ConstWriter::for_build("constants").unwrap();
    let mut consts = consts.finish_dependencies();
    consts.add_value("NUMBER_OF_POKEMON", "usize", number_of_pokemon);
    consts.add_value("NUMBER_OF_FUSIONS", "usize", number_of_fusions);
    consts.finish();
}

fn create_if_not_exists(path: &str, bytes: Vec<u8>) {
    if std::path::Path::new(path).exists() {
        let existing_bytes = std::fs::read(path).unwrap();
        if existing_bytes != bytes {
            std::fs::write(path, bytes).unwrap();
        }
    } else {
        std::fs::write(path, bytes).unwrap();
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
            bst: raw.base.hp as u16
                + raw.base.attack as u16
                + raw.base.defense as u16
                + raw.base.special_attack as u16
                + raw.base.special_defense as u16
                + raw.base.speed as u16,
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
    pub ability: Vec<Ability>,
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

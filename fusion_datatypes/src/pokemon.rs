use super::Type;
use serde::{Deserialize, Serialize};

type Ability = (String, bool);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pokemon {
    pub id: u16,
    pub name: String,
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub special_attack: u8,
    pub special_defense: u8,
    pub speed: u8,
    pub primary_type: Type,
    pub secondary_type: Option<Type>,
    pub fusion_type_override: Option<Type>,
    pub abilities: Vec<Ability>,
}

#[derive(Debug, Clone)]
pub struct FusedPokemon {
    pub name: String,
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub special_attack: u8,
    pub special_defense: u8,
    pub speed: u8,
    pub primary_type: Type,
    pub secondary_type: Option<Type>,
    pub abilities: Vec<Ability>,
    pub head_id: u16,
    pub body_id: u16,
}

impl FusedPokemon {
    pub fn fuse(head: &Pokemon, body: &Pokemon) -> FusedPokemon {
        let name = format!("{}/{}", head.name, body.name);
        let hp = calc_head_stat(head.hp, body.hp);
        let attack = calc_body_stat(head.attack, body.attack);
        let defense = calc_body_stat(head.defense, body.defense);
        let special_attack = calc_head_stat(head.special_attack, body.special_attack);
        let special_defense = calc_head_stat(head.special_defense, body.special_defense);
        let speed = calc_body_stat(head.speed, body.speed);

        let (primary_type, secondary_type) = {
            let head_provided = match head.fusion_type_override {
                Some(t) => t,
                None => head.primary_type,
            };
            let body_provided = match body.fusion_type_override {
                Some(t) => t,
                None => match body.secondary_type {
                    Some(t) => {
                        if t == head.primary_type {
                            body.primary_type
                        } else {
                            t
                        }
                    }
                    None => body.primary_type,
                },
            };
            if head_provided == body_provided {
                (head_provided, None)
            } else {
                (head_provided, Some(body_provided))
            }
        };

        // Quick and dirty clone
        let mut abilities = head.abilities.clone();
        abilities.append(&mut body.abilities.clone());

        FusedPokemon {
            name,
            hp,
            attack,
            defense,
            special_attack,
            special_defense,
            speed,
            primary_type,
            secondary_type,
            abilities,
            head_id: head.id,
            body_id: body.id,
        }
    }

    pub fn get_bst(&self) -> u16 {
        self.hp as u16
            + self.attack as u16
            + self.defense as u16
            + self.special_attack as u16
            + self.special_defense as u16
            + self.speed as u16
    }
}

fn calc_body_stat(head: u8, body: u8) -> u8 {
    let head = head as f32;
    let body = body as f32;
    (2f32 * (body / 3f32) + (head / 3f32)).floor() as u8
}

fn calc_head_stat(head: u8, body: u8) -> u8 {
    let head = head as f32;
    let body = body as f32;
    (2f32 * (head / 3f32) + (body / 3f32)).floor() as u8
}

impl Pokemon {
    pub fn get_bst(&self) -> u16 {
        self.hp as u16
            + self.attack as u16
            + self.defense as u16
            + self.special_attack as u16
            + self.special_defense as u16
            + self.speed as u16
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn fuse() {
        let kyogre = Pokemon {
            id: 382,
            name: "Kyogre".to_string(),
            hp: 100,
            attack: 100,
            defense: 90,
            special_attack: 150,
            special_defense: 140,
            speed: 90,
            primary_type: Type::Water,
            secondary_type: None,
            fusion_type_override: None,
            abilities: vec![],
        };

        let arceus = Pokemon {
            id: 493,
            name: "Arceus".to_string(),
            hp: 120,
            attack: 120,
            defense: 120,
            special_attack: 120,
            special_defense: 120,
            speed: 120,
            primary_type: Type::Normal,
            secondary_type: None,
            fusion_type_override: None,
            abilities: vec![],
        };
        let fusion = FusedPokemon::fuse(&kyogre, &arceus);

        assert_eq!(fusion.name, "Kyogre/Arceus".to_string());
        assert_eq!(fusion.hp, 106);
        assert_eq!(fusion.attack, 113);
        assert_eq!(fusion.defense, 110);
        assert_eq!(fusion.special_attack, 140);
        assert_eq!(fusion.special_defense, 133);
        assert_eq!(fusion.speed, 110);
        assert_eq!(fusion.primary_type, Type::Water);
        assert_eq!(fusion.secondary_type, Some(Type::Normal));
        assert_eq!(fusion.get_bst(), 712);

        let fusion = FusedPokemon::fuse(&arceus, &kyogre);

        assert_eq!(fusion.name, "Arceus/Kyogre".to_string());
        assert_eq!(fusion.hp, 113);
        assert_eq!(fusion.attack, 106);
        assert_eq!(fusion.defense, 100);
        assert_eq!(fusion.special_attack, 130);
        assert_eq!(fusion.special_defense, 126);
        assert_eq!(fusion.speed, 100);
        assert_eq!(fusion.primary_type, Type::Normal);
        assert_eq!(fusion.secondary_type, Some(Type::Water));
        assert_eq!(fusion.get_bst(), 675);
    }
}

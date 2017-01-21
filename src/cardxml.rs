use std::collections::HashMap;
use std::str;

use serde_xml;
use num::FromPrimitive;
use enums::{GameTag, PlayReq};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct CardDefs {
    Entity: Vec<Entity>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Entity {
    CardID: String,
    ID: i32,
    MasterPower: Option<String>,
    Tag: Vec<Tag>,
    ReferencedTag: Vec<ReferencedTag>,
    EntourageCard: Vec<EntourageCard>,
    Power: Vec<Power>,
    TriggeredPowerHistoryInfo: Vec<TriggeredPowerHistoryInfo>,
}

trait Taggable {
    fn enum_id(&self) -> i32;

    fn card_id(&self) -> Option<String>;

    fn tag_type(&self) -> String;

    fn tag_value(&self) -> i32;

    fn unpack(&self) -> Option<(GameTag, Tagged)> {
        match GameTag::from_i32(self.enum_id()) {
            Some(game_tag) => {
                let tagged = Tagged(self.tag_value(), self.card_id());
                Some((game_tag, tagged))
            },
            None => {
                // warn!("No enums::GameTag for: {}", self.enum_id());
                error!("Error enums::GameTag missing: {}", self.enum_id());
                panic!("No enums::GameTag defined.");
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Tag {
    enumID: i32,
    cardID: Option<String>,
    #[serde(rename="type")]
    valueType: String,
    value: Option<i32>,
    deDE: String,
    enUS: String,
    esES: String,
    esMX: String,
    frFR: String,
    itIT: String,
    jaJP: String,
    koKR: String,
    plPL: String,
    ptBR: String,
    ruRU: String,
    thTH: String,
    zhCN: String,
    zhTW: String,
}

impl Taggable for Tag {
    fn enum_id(&self) -> i32 {
        self.enumID
    }

    fn card_id(&self) -> Option<String> {
        self.cardID.clone()
    }

    fn tag_type(&self) -> String {
        self.valueType.clone()
    }

    fn tag_value(&self) -> i32 {
        match self.value {
            Some(v) => v,
            None => 0
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ReferencedTag {
    enumID: i32,
    cardID: Option<String>,
    valueType: String,
    value: Option<i32>,
}

impl Taggable for ReferencedTag {
    fn enum_id(&self) -> i32 {
        self.enumID
    }

    fn card_id(&self) -> Option<String> {
        self.cardID.clone()
    }

    fn tag_type(&self) -> String {
        self.valueType.clone()
    }

    fn tag_value(&self) -> i32 {
        match self.value {
            Some(v) => v,
            None => 0
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct EntourageCard {
    cardID: String
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Power {
    definition: String,
    PlayRequirement: Vec<PlayRequirement>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct PlayRequirement {
    reqID: i32,
    param: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TriggeredPowerHistoryInfo {
}

type Cards = HashMap<String, Card>;
type Tags = HashMap<GameTag, Tagged>;
type Powers = Vec<Powered>;
type Requirements = HashMap<PlayReq, i32>;

#[derive(Debug)]
struct Tagged(i32, Option<String>);

#[derive(Debug)]
struct Powered {
    definition: String,
    requirements: Requirements,
}

impl Powered {
    fn new(data: Power) -> Powered {
        let mut requirements = Requirements::new();
        for pr in data.PlayRequirement {
            match PlayReq::from_i32(pr.reqID) {
                Some(req) => {
                    let param = match pr.param.parse::<i32>() {
                        Ok(n) => n,
                        Err(_) => 0,
                    };

                    requirements.insert(req, param);
                },
                None => {
                    error!("Error enums::PlayReq missing: {:?}", pr);
                    panic!("No enums::PlayReq defined.");
                },
            }
        }

        Powered {
            definition: data.definition.to_owned(),
            requirements: requirements,
        }
    }
}

#[derive(Debug)]
pub struct Card {
    id: String,
    dbf_id: i32,
    master_power: Option<String>,
    hero_power: Option<String>,
    tags: Tags,
    referenced_tags: Tags,
    powers: Powers,
    entourage: Vec<String>,
}

impl Card {
    fn new(data: Entity) -> Card {
        let mut tags = Tags::new();
        for tag in data.Tag {
            if let Some(t) = tag.unpack() {
                tags.insert(t.0, t.1);
            }
        }

        let mut referenced_tags = Tags::new();
        for tag in data.ReferencedTag {
            if let Some(t) = tag.unpack() {
                referenced_tags.insert(t.0, t.1);
            }
        }

        let hero_power = match tags.get(&GameTag::HeroPower) {
            Some(t) => t.1.clone(),
            _ => None,
        };

        let mut powers = Powers::new();
        for power in data.Power {
            let p = Powered::new(power);
            powers.push(p);
        }

        let mut entourage = vec![];
        for ec in data.EntourageCard {
            entourage.push(ec.cardID);
        }

        Card {
            id: data.CardID.to_owned(),
            dbf_id: data.ID.to_owned(),
            master_power: data.MasterPower.to_owned(),
            hero_power: hero_power,
            tags: tags,
            referenced_tags: referenced_tags,
            powers: powers,
            entourage: entourage,
        }
    }
}

pub fn load() -> Cards {
    info!("loading CardDefs.xml");

    let b = include_bytes!("CardDefs.xml");
    let s = str::from_utf8(b).unwrap();
    let card_defs : CardDefs = serde_xml::from_str(&s).unwrap();

    let mut cards = Cards::new();
    for entity in card_defs.Entity {
        let card = Card::new(entity);
        cards.insert(card.id.clone(), card);
    }

    info!("finished with CardDefs.xml");

    cards
}

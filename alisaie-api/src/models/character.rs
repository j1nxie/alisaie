use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use ffxiv_types::World;
use serde::Deserialize;
use url::Url;

use crate::enum_numbers;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Character {
    #[serde(rename = "ID")]
    pub id: u64,
    pub name: String,
    pub nameday: String,
    pub parse_date: DateTime<Utc>,
    #[serde(rename = "PvPTeam")]
    pub pvp_team: Option<serde_json::Value>,
    pub race: Race,
    pub tribe: Tribe,
    pub server: World,
    pub town: Town,
    #[serde(with = "url_serde")]
    pub avatar: Url,
    pub bio: String,
    pub free_company_id: Option<u64>,
    pub gender: Gender,
    pub guardian_deity: GuardianDeity,
    pub minions: Vec<u64>,
    pub mounts: Vec<u64>,
    pub class_jobs: BTreeMap<String, ClassJob>,
    pub gear_set: GearSet,
    pub grand_company: GrandCompany,
    pub active_class_job: ClassJob,
    #[serde(with = "url_serde")]
    pub portrait: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassJob {
    #[serde(rename = "ClassID")]
    pub class_id: u64,
    #[serde(rename = "JobID")]
    pub job_id: u64,
    pub exp_level: u64,
    pub exp_level_max: u64,
    pub exp_level_remaining: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearSet {
    #[serde(rename = "ClassID")]
    pub class_id: u64,
    #[serde(rename = "JobID")]
    pub job_id: u64,
    pub level: u64,
    pub gear_key: String,
    pub attributes: BTreeMap<String, u64>,
    pub gear: BTreeMap<String, Gear>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Gear {
    #[serde(rename = "ID")]
    pub id: Option<u64>,
    pub dye: Option<u64>,
    pub mirage: Option<serde_json::Value>,
    pub materia: Vec<u64>,
    pub creatore: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GrandCompany {
    #[serde(rename = "NameID")]
    pub name_id: u64,
    #[serde(rename = "RankID")]
    pub rank_id: u64,
}

enum_numbers!(Race {
    Hyur = 1,
    Elezen = 2,
    Lalafell = 3,
    Miqote = 4,
    Roegadyn = 5,
    AuRa = 6,
    Viera = 7,
});

enum_numbers!(Gender {
    Male = 1,
    Female = 2,
});

enum_numbers!(GuardianDeity {
    Halone = 1,
    Menphina = 2,
    Thaliak = 3,
    Nymeia = 4,
    Llymlaen = 5,
    Oschon = 6,
    Byregot = 7,
    Rhalgr = 8,
    Azeyma = 9,
    NaldThal = 10,
    Nophica = 11,
    Althyk = 12,
});

enum_numbers!(Town {
    LimsaLominsa = 1,
    Gridania = 2,
    UlDah = 3,
    Ishgard = 4,
    Kugane = 7,
});

enum_numbers!(Tribe {
    Midlander = 1,
    Highlander = 2,
    Wildwood = 3,
    Duskwight = 4,
    Plainsfolk = 5,
    Dunesfolk = 6,
    SeekerOfTheSun = 7,
    SeekerOfTheMoon = 8,
    SeaWolf = 9,
    Hellsguard = 10,
    Raen = 11,
    Xaela = 12,
    Rava = 13,
    Veena = 14,
});

// FIXME: check between 19 and 46 to see which is Tenacity and which is Spell Speed.
enum_numbers!(Attributes {
    Strength = 1,
    Dexterity = 2,
    Vitality = 3,
    Intelligence = 4,
    Mind = 5,
    Piety = 6,
    Hp = 7,
    Mp = 8,
    Tenacity = 19,
    AttackPower = 20,
    Defense = 21,
    DirectHitRate = 22,
    MagicDefense = 24,
    CriticalHitRate = 27,
    AttackMagicPotency = 33,
    HealingMagicPotency = 34,
    Determination = 44,
    SkillSpeed = 45,
    SpellSpeed = 46,
});

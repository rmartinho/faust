mod lexer;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser, "/parse/export_descr_unit/parser.rs");

pub use lexer::Token;
pub use parser::UnitsParser as Parser;

#[derive(Debug)]
pub struct Unit {
    pub id: String,
    pub key: String,
    pub category: String,
    pub class: String,
    pub stats: StatBlock,
    pub ownership: Vec<String>,
    pub rebalanced_stats: Option<StatBlock>,
}

#[derive(Debug)]
pub struct StatBlock {
    pub soldiers: u32,
    pub officers: u32,
    pub attributes: Vec<Attr>,
    pub formations: Vec<Formation>,
    pub hp: u32,
    pub hp_mount: u32,
    pub primary_weapon: Weapon,
    pub secondary_weapon: Weapon,
    pub defense: Defense,
    pub defense_mount: Defense,
    pub heat: i32,
    pub ground_bonus: GroundBonus,
    pub morale: u32,
    pub discipline: Discipline,
    pub turns: u32,
    pub cost: u32,
    pub upkeep: u32,
}

#[derive(Debug)]
pub struct Weapon {
    pub factor: u32,
    pub charge: u32,
    pub missile: String,
    pub range: u32,
    pub ammo: u32,
    pub lethality: f64,
    pub attributes: Vec<WeaponAttr>,
}

#[derive(Debug)]
pub struct GroundBonus {
    pub scrub: i32,
    pub sand: i32,
    pub forest: i32,
    pub snow: i32,
}

#[derive(Debug)]
pub struct Defense {
    pub armor: u32,
    pub skill: u32,
    pub shield: u32,
}

#[derive(Debug)]
pub enum Attr {
    SeaFaring,
    HideForest,
    HideImprovedForest,
    HideLongGrass,
    HideAnywhere,
    CanSap,
    FrightenFoot,
    FrightenMounted,
    CanRunAmok,
    GeneralUnit,
    GeneralUnitUpgrade(String),
    CantabrianCircle,
    NoCustom,
    Command,
    ScreechingWomen,
    MercenaryUnit,
    Hardy,
    VeryHardy,
    ExtremelyHardy,
    Inexhaustible,
    Warcry,
    Druid,
    PowerCharge,
    CanSwim,
    IsPeasant,
    CanHorde,
    LegionaryName,
    InfiniteAmmo,
    NonScaling,
}

#[derive(Debug)]
pub enum Formation {
    Square,
    Horde,
    Phalanx,
    Testudo,
    Wedge,
    Schiltrom,
    ShieldWall,
}

#[derive(Debug)]
pub enum Discipline {
    Low,
    Normal,
    Disciplined,
    Impetuous,
    Berserker,
}

#[derive(Debug)]
pub enum WeaponAttr {
    ArmorPiercing,
    BodyPiercing,
    Spear,
    LongPike,
    ShortPike,
    LightSpear,
    Precharge,
    Thrown,
    Launching,
    Area,
    SpearBonus(u32),
    Fire,
}

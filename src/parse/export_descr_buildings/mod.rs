mod lexer;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser, "/parse/export_descr_buildings/parser.rs");

pub use lexer::Token;
pub use parser::BuildingsFileParser as Parser;

#[derive(Debug)]
pub struct Building {
    req: Requires,
    caps: Vec<RecruitOption>,
    min: String,
}

#[derive(Debug)]
pub struct RecruitOption {
    unit: String,
    exp: u32,
    req: Requires,
}

#[derive(Debug, Default)]
pub enum Requires {
    #[default]
    None,
    Resource {
        id: String,
        factionwide: bool,
    },
    HiddenResource {
        id: String,
        factionwide: bool,
    },
    BuildingPresent {
        id: String,
        level: Option<String>,
        queued: bool,
        factionwide: bool,
    },
    MajorEvent(String),
    Factions(Vec<String>),
    BuildingFactions(Vec<String>),
    Port,
    IsPlayer,
    IsToggled(String),
    Diplomacy { status: DipStatus, faction: String },
    // Capability { capability: Capability, amount: u32 },
    NoBuildingTagged {
        tag: String,
        queued: bool,
        factionwide: bool,
    },
    Religion {
        id: String,
        cmp: Cmp,
        amount: u32,
    },
    MajorityReligion(String),
    OfficialReligion,

    Alias(String),

    Not(Box<Requires>),
    And(Vec<Box<Requires>>),
    Or(Vec<Box<Requires>>),
}

#[derive(Debug)]
pub enum Cmp {
    Lt, Le, Gt, Ge,
}

#[derive(Debug)]
pub enum DipStatus {
	Allied,
	Protector,
	Protectorate,
	SameSuperfaction,
	AtWar,
}
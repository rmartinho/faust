mod lexer;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser, "/parse/export_descr_buildings/parser.rs");

pub use lexer::Token;
pub use parser::BuildingsFileParser as Parser;

#[derive(Debug)]
pub struct Building {
    pub req: Requires,
    pub caps: Vec<RecruitOption>,
    pub min: String,
}

#[derive(Debug)]
pub struct RecruitOption {
    pub unit: String,
    pub exp: u32,
    pub req: Requires,
}

#[derive(Debug, Default, Clone)]
pub enum Requires {
    #[default]
    None,
    False,
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
    And(Vec<Requires>),
    Or(Vec<Requires>),
}

#[derive(Debug, Clone)]
pub enum Cmp {
    Lt, Le, Gt, Ge,
}

#[derive(Debug, Clone)]
pub enum DipStatus {
	Allied,
	Protector,
	Protectorate,
	SameSuperfaction,
	AtWar,
}
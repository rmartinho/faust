use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::parse::{
    descr_regions::Region, descr_sm_factions::Faction, export_descr_buildings::Requires,
};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Evaluator {
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    faction: Option<EvaluatorChoices>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<EvaluatorChoices>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden_resource: Option<EvaluatorChoices>,
    #[serde(skip_serializing_if = "Option::is_none")]
    major_event: Option<EvaluatorChoices>,
}

impl Evaluator {
    pub fn faction(faction: &Faction) -> Self {
        Self {
            faction: Some(EvaluatorChoices {
                map: [
                    (faction.id.clone(), true),
                    (faction.culture.clone(), true),
                    ("all".into(), true),
                ]
                .into(),
                default: Some(false),
            }),
            ..Default::default()
        }
    }

    pub fn region(region: &Region) -> Self {
        Self {
            hidden_resource: Some(EvaluatorChoices {
                map: region
                    .hidden_resources
                    .iter()
                    .map(|r| (r.clone(), true))
                    .collect(),
                default: Some(false),
            }),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluatorChoices {
    #[serde(flatten)]
    map: HashMap<String, bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default: Option<bool>,
}

impl EvaluatorChoices {
    fn get(&self, choice: &str) -> Option<bool> {
        self.map.get(choice).copied().or(self.default)
    }
}

pub fn evaluate(req: &Requires, aliases: &HashMap<String, Requires>, eval: &Evaluator) -> bool {
    do_evaluate(req, aliases, eval).unwrap_or(true)
}

fn do_evaluate(
    req: &Requires,
    aliases: &HashMap<String, Requires>,
    eval: &Evaluator,
) -> Option<bool> {
    match req {
        Requires::False => Some(false),
        Requires::Resource {
            id,
            factionwide: false,
        } => eval.resource.as_ref().and_then(|r| r.get(id)),
        Requires::HiddenResource {
            id,
            factionwide: false,
        } => eval.hidden_resource.as_ref().and_then(|r| r.get(id)),
        Requires::MajorEvent(event) => eval.major_event.as_ref().and_then(|r| r.get(event)),
        Requires::Factions(factions) => {
            let res = factions
                .iter()
                .map(|id| eval.faction.as_ref().and_then(|r| r.get(id)))
                .filter_map(|x| x)
                .collect::<Vec<_>>();
            if res.len() == 0 {
                None
            } else {
                Some(res.iter().any(|x| *x))
            }
        }
        Requires::IsPlayer => Some(true),
        Requires::Alias(id) => do_evaluate(
            aliases.get(id).expect(&format!("invalid alias: {id}")),
            aliases,
            eval,
        ),
        Requires::Not(requires) => do_evaluate(requires, aliases, eval).map(|r| !r),
        Requires::And(items) => {
            let res = items
                .iter()
                .map(|item| do_evaluate(item, aliases, eval))
                .filter_map(|x| x)
                .collect::<Vec<_>>();
            if res.len() == 0 {
                None
            } else {
                Some(res.iter().all(|x| *x))
            }
        }
        Requires::Or(items) => {
            let res = items
                .iter()
                .map(|item| do_evaluate(item, aliases, eval))
                .filter_map(|x| x)
                .collect::<Vec<_>>();
            if res.len() == 0 {
                None
            } else {
                Some(res.iter().any(|x| *x))
            }
        }
        _ => eval.default,
    }
}

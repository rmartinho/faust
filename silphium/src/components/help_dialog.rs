use std::collections::HashMap;

use web_sys::{HtmlDetailsElement, HtmlElement};
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_hooks::prelude::*;

use crate::{
    components::{
        AbilitiesRow, CostRow, DefenseRow, Icon, MentalRow, SizeRow, TerrainRow, UnitCard,
        UpkeepRow, WeaponRow,
    },
    model::{Ability, GroundBonus, Unit, Weapon, WeaponType},
};

const HELP_UNIT_CBOR: &[u8] = include_bytes!("help-unit.cbor");

pub trait Dialog {
    fn show(&self);
    #[allow(dead_code)]
    fn hide(&self);
}

struct PopOver {
    el: HtmlElement,
}

impl Dialog for PopOver {
    fn show(&self) {
        let _ = self.el.show_popover();
    }

    fn hide(&self) {
        let _ = self.el.hide_popover();
    }
}

#[autoprops]
#[function_component(HelpDialog)]
pub fn help_dialog(#[prop_or_default] control: Callback<Option<Box<dyn Dialog>>>) -> Html {
    let popover_ref = use_node_ref();

    use_effect({
        let popover_ref = popover_ref.clone();
        move || {
            let popover = popover_ref.cast::<HtmlElement>().unwrap();
            control.emit(Some(Box::new(PopOver { el: popover })));
        }
    });

    let hide = {
        let popover_ref = popover_ref.clone();
        move |_| {
            let popover = popover_ref.cast::<HtmlElement>().unwrap();
            let _ = popover.hide_popover();
        }
    };
    use_click_away(popover_ref.clone(), hide.clone());
    // let hide = Callback::from(move |e| hide(Event::from(e)));

    let unit: &Unit = &ciborium::from_reader(HELP_UNIT_CBOR).unwrap();
    let display_weapon = Weapon {
        class: WeaponType::Spear,
        lethality: 0.75,
        armor_piercing: true,
        spear_bonus: 4,
        ..unit.secondary_weapon.clone().unwrap()
    };
    let display_hp = 2;
    let display_terrain = Unit {
        ground_bonus: GroundBonus {
            scrub: 2,
            sand: -4,
            forest: -6,
            snow: 1,
        },
        ..unit.clone()
    };
    let display_abilities = Unit {
        abilities: vec![Ability::Command, Ability::HideImprovedForest].into(),
        ..unit.clone()
    };

    let refs: HashMap<_, _> = [
        ("soldiers", use_node_ref()),
        ("cost", use_node_ref()),
        ("upkeep", use_node_ref()),
        ("mental", use_node_ref()),
        ("terrain", use_node_ref()),
        ("weapons", use_node_ref()),
        ("defenses", use_node_ref()),
        ("abilities", use_node_ref()),
    ]
    .into();
    let open_details = |s: &str| {
        Callback::from({
            let node = refs[s].clone();
            move |_: MouseEvent| node.cast::<HtmlDetailsElement>().unwrap().set_open(true)
        })
    };
    let close_details = Callback::from({
        let refs = refs.clone();
        move |_: MouseEvent| {
            for node in refs.values() {
                node.cast::<HtmlDetailsElement>().unwrap().set_open(false)
            }
        }
    });

    html! {
      <div ref={popover_ref} popover="">
        <div  class="help-dialog">
          <div class="descr left">
            <button onclick={close_details} class="name">{"Name"}</button>
            <button onclick={open_details("soldiers")} class="soldiers">{"Formation"}</button>
            <button onclick={open_details("cost")} class="cost">{"Recruitment"}</button>
            <button onclick={open_details("upkeep")} class="upkeep">{"Upkeep"}</button>
            <button onclick={open_details("mental")} class="mental">{"Soldiers"}</button>
            <button onclick={open_details("terrain")} class="terrain">{"Terrain"}</button>
            <button onclick={open_details("weapons")} class="weapons">{"Weapons"}</button>
            <button onclick={open_details("defenses")} class="defenses">{"Defense"}</button>
          </div>
          <UnitCard {unit} />
          <div class="descr right">
            <button onclick={open_details("abilities")} class="abilities">{"Abilities"}</button>
          </div>
          <div class="descr details">
            <details ref={&refs["soldiers"]} name="help-section">
              <summary>
                {"Formations"}
                <div class="unit-card">
                  <div class="frame">
                    <SizeRow class="size-row row" {unit} />
                  </div>
                </div>
              </summary>
              <div class="help-table">
                <Icon class="icon" src="/icons/stat.svg" symbol="soldiers" />
                <span>{"unit size: "}<span class="soldiers">{"soldiers"}</span>{" and "}<span class="officers">{"officers"}</span></span>

                <Icon class="icon" src="/icons/formation.svg" symbol="square" />
                <span>{"square"}</span>

                <Icon class="icon" src="/icons/formation.svg" symbol="horde" />
                <span>{"horde"}</span>

                <Icon class="icon" src="/icons/formation.svg" symbol="phalanx" />
                <span>{"phalanx"}</span>

                <Icon class="icon" src="/icons/formation.svg" symbol="testudo" />
                <span>{"testudo"}</span>

                <Icon class="icon" src="/icons/formation.svg" symbol="wedge" />
                <span>{"wedge"}</span>

                <Icon class="icon" src="/icons/formation.svg" symbol="schiltrom" />
                <span>{"schiltrom"}</span>

                <Icon class="icon" src="/icons/formation.svg" symbol="shield_wall" />
                <span>{"shield wall"}</span>
              </div>
            </details>
            <details ref={&refs["cost"]} name="help-section">
              <summary>
                {"Recruitment"}
                <div class="unit-card">
                  <div class="frame">
                    <CostRow class="cost-row row" {unit} />
                  </div>
                </div>
              </summary>
              <div class="help-table">
                <Icon class="icon" src="/icons/stat.svg" symbol="cost" />
                <span class="cost">{"cost to recruit"}</span>

                <Icon class="icon" src="/icons/attribute.svg" symbol="turns" />
                <span class="turns">{"turns to recruit"}</span>
              </div>
            </details>
            <details ref={&refs["upkeep"]} name="help-section">
              <summary>
                {"Upkeep"}
                <div class="unit-card">
                  <div class="frame">
                    <UpkeepRow class="upkeep-row row" {unit} />
                  </div>
                </div>
              </summary>
              <div class="help-table">
                <Icon class="icon" src="/icons/stat.svg" symbol="upkeep" />
                <span class="upkeep">{"upkeep cost"}</span>
              </div>
            </details>
            <details ref={&refs["mental"]} name="help-section">
              <summary>
                {"Soldiers"}
                <div class="unit-card">
                  <div class="frame">
                    <MentalRow class="mental-row row" {unit} />
                  </div>
                </div>
              </summary>
              <div class="help-table">
                <Icon class="icon" src="/icons/discipline.svg" symbol="normal" />
                <span>{"medium discipline: "}<span class="morale">{"morale"}</span></span>

                <Icon class="icon" src="/icons/discipline.svg" symbol="low" />
                <span>{"low discipline: morale"}</span>

                <Icon class="icon" src="/icons/discipline.svg" symbol="disciplined" />
                <span>{"high discipline: morale"}</span>

                <Icon class="icon" src="/icons/discipline.svg" symbol="impetuous" />
                <span>{"impetuous: morale"}</span>

                <Icon class="icon" src="/icons/discipline.svg" symbol="berserker" />
                <span>{"berserker: morale"}</span>

                <Icon class="icon" src="/icons/speed.svg" symbol="speed-1" />
                <span>{"slow"}</span>

                <Icon class="icon" src="/icons/speed.svg" symbol="speed-2" />
                <span>{"medium speed"}</span>

                <Icon class="icon" src="/icons/speed.svg" symbol="speed-3" />
                <span>{"fast"}</span>

                <Icon class="icon" src="/icons/speed.svg" symbol="speed-4" />
                <span>{"very fast"}</span>

                <Icon class="icon" src="/icons/attribute.svg" symbol="stamina" />
                <span>{"stamina"}</span>

                <Icon class="icon" src="/icons/attribute.svg" symbol="inexhaustible" />
                <span>{"inexhaustible"}</span>

                <Icon class="icon" src="/icons/attribute.svg" symbol="heat" />
                <span>{"heat penalty"}</span>
              </div>
            </details>
            <details ref={&refs["terrain"]} name="help-section">
              <summary>
                {"Terrain"}
                <div class="unit-card">
                  <div class="frame">
                    <TerrainRow class="terrain row" unit={display_terrain} />
                  </div>
                </div>
              </summary>
              <div class="help-table">
                <Icon class="icon" src="/icons/terrain.svg" symbol="scrub" />
                <span>{"scrub"}</span>

                <Icon class="icon" src="/icons/terrain.svg" symbol="forest" />
                <span>{"forest"}</span>

                <Icon class="icon" src="/icons/terrain.svg" symbol="sand" />
                <span>{"sand"}</span>

                <Icon class="icon" src="/icons/terrain.svg" symbol="snow" />
                <span>{"snow"}</span>
              </div>
            </details>
            <details ref={&refs["weapons"]} name="help-section">
              <summary>
                {"Weapons"}
                <div class="unit-card">
                  <div class="frame">
                    <WeaponRow class="weapon1-row row" {unit} weapon={display_weapon} />
                  </div>
                </div>
              </summary>
              <div class="help-table">
                <Icon class="icon" src="/icons/weapon.svg" symbol="melee" />
                <span>{"blade: "}<span class="strength"><strong>{"strength"}</strong></span>
                  {" and "}<span class="lethality">{"lethality"}</span></span>

                <Icon class="icon" src="/icons/weapon.svg" symbol="spear" />
                <span>{"spear: strength and lethality"}</span>

                <Icon class="icon" src="/icons/weapon.svg" symbol="missile" />
                <span>{"missile: strength and lethality"}</span>

                <Icon class="icon" src="/icons/weapon.svg" symbol="thrown" />
                <span>{"thrown: strength and lethality"}</span>

                <Icon class="icon" src="/icons/weapon.svg" symbol="gunpowder" />
                <span>{"firearm: strength and lethality"}</span>

                <Icon class="icon" src="/icons/attribute.svg" symbol="range" />
                <span>{"range"}</span>

                <Icon class="icon" src="/icons/attribute.svg" symbol="ammo" />
                <span>{"ammunition"}</span>

                <Icon class="icon" src="/icons/attribute.svg" symbol="charge" />
                <span>{"charge bonus"}</span>

                <Icon class="icon" src="/icons/attribute.svg" symbol="against-cavalry" />
                <span>{"bonus against cavalry"}</span>

                <Icon class="icon" src="/icons/attribute.svg" symbol="armor-piercing" />
                <span>{"armor piercing"}</span>

                <Icon class="icon" src="/icons/attribute.svg" symbol="precharge" />
                <span>{"thrown before charge"}</span>
              </div>
            </details>
            <details ref={&refs["defenses"]} name="help-section">
              <summary>
                {"Defenses"}
                <div class="unit-card">
                  <div class="frame">
                    <DefenseRow class="defense1-row row" def={&unit.defense} hp={display_hp} />
                  </div>
                </div>
              </summary>
              <div class="help-table">
                <Icon class="icon" src="/icons/stat.svg" symbol="defense" />
                <span>{"soldier defenses: "}<span class="defense"><strong>{"total"}</strong></span></span>

                <Icon class="icon" src="/icons/stat.svg" symbol="defense-mount" />
                <span>{"mount defenses: total"}</span>

                <Icon class="icon" src="/icons/attribute.svg" symbol="armor" />
                <span>{"armor"}</span>

                <Icon class="icon" src="/icons/attribute.svg" symbol="skill" />
                <span>{"skill"}</span>

                <Icon class="icon" src="/icons/attribute.svg" symbol="shield" />
                <span>{"shield"}</span>

                <Icon class="icon" src="/icons/ability.svg" symbol="heart" />
                <span>{"hit points"}</span>
              </div>
            </details>
            <details ref={&refs["abilities"]} name="help-section">
              <summary>
                {"Abilities"}
                <div class="unit-card">
                  <div class="frame">
                    <AbilitiesRow class="abilities row" unit={display_abilities} />
                  </div>
                </div>
              </summary>
              <div class="help-table">
                <Icon class="icon" src="/icons/ability.svg" symbol="cantabrian-circle" />
                <span>{"Cantabrian circle"}</span>

                <Icon class="icon" src="/icons/ability.svg" symbol="power-charge" />
                <span>{"powerful charge"}</span>

                <Icon class="icon" src="/icons/ability.svg" symbol="formed-charge" />
                <span>{"formed charge"}</span>

                <Icon class="icon" src="/icons/ability.svg" symbol="warcry" />
                <span>{"warcry"}</span>

                <Icon class="icon" src="/icons/ability.svg" symbol="chant" />
                <span>{"chanting/screeching"}</span>

                <Icon class="icon" src="/icons/ability.svg" symbol="command" />
                <span>{"inspires units"}</span>

                <Icon class="icon" src="/icons/ability.svg" symbol="frighten-all" />
                <span>{"frighten all units"}</span>

                <Icon class="icon" src="/icons/ability.svg" symbol="frighten-foot" />
                <span>{"frighten infantry"}</span>

                <Icon class="icon" src="/icons/ability.svg" symbol="frighten-mounted" />
                <span>{"frighten cavalry"}</span>

                <Icon class="icon" src="/icons/ability.svg" symbol="can-run-amok" />
                <span>{"may run amok"}</span>

                <Icon class="icon" src="/icons/ability.svg" symbol="cant-hide" />
                <span>{"cannot hide"}</span>

                <Icon class="icon" src="/icons/ability.svg" symbol="hide-anywhere" />
                <span>{"hide anywhere"}</span>

                <Icon class="icon" src="/icons/ability.svg" symbol="hide-forest" />
                <span>{"hide well in forests"}</span>

                <Icon class="icon" src="/icons/ability.svg" symbol="hide-grass" />
                <span>{"hide in long grass"}</span>

                <Icon class="icon" src="/icons/ability.svg" symbol="stakes" />
                <span>{"defensive stakes"}</span>

                <Icon class="icon" src="/icons/ability.svg" symbol="knight" />
                <span>{"knight"}</span>
              </div>
            </details>
          </div>
        </div>
      </div>
    }
}

use std::{io, path::Path};

use askama::Template;

use crate::utils::write_file;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexHtml<'a> {
    pub head: &'a str,
    pub body: &'a str,
}

#[derive(Template)]
#[template(path = "redirect.html")]
pub struct RedirectHtml<'a> {
    pub target: &'a str,
}

pub struct StaticFile<'a> {
    pub path: &'a str,
    pub contents: &'a [u8],
}

impl<'a> StaticFile<'a> {
    pub async fn create(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref().join(self.path);
        write_file(path, self.contents).await
    }
}

pub const FILESYSTEM_STATIC: &[StaticFile] = &[
    StaticFile {
        path: "favicon.png",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/favicon.png")),
    },
    StaticFile {
        path: "styles/main.css",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/styles/main.css")),
    },
    StaticFile {
        path: "scripts/silphium.js",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/scripts/silphium.js")),
    },
    StaticFile {
        path: "scripts/silphium_bg.wasm",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/scripts/silphium_bg.wasm")),
    },
    StaticFile {
        path: "fonts/blinker-regular-1.woff2",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/fonts/blinker-regular-1.woff2")),
    },
    StaticFile {
        path: "fonts/blinker-regular-2.woff2",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/fonts/blinker-regular-2.woff2")),
    },
    StaticFile {
        path: "fonts/blinker-bold-1.woff2",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/fonts/blinker-bold-1.woff2")),
    },
    StaticFile {
        path: "fonts/blinker-bold-2.woff2",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/fonts/blinker-bold-2.woff2")),
    },
    StaticFile {
        path: "fonts/blinker-regular.woff",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/fonts/blinker-regular.woff")),
    },
    StaticFile {
        path: "fonts/blinker-bold.woff",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/fonts/blinker-bold.woff")),
    },
    StaticFile {
        path: "icons/ability/can-run-amok.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ability/can-run-amok.svg")),
    },
    StaticFile {
        path: "icons/ability/chant.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ability/chant.svg")),
    },
    StaticFile {
        path: "icons/ability/cantabrian-circle.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ability/cantabrian-circle.svg")),
    },
    StaticFile {
        path: "icons/ability/command.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ability/command.svg")),
    },
    StaticFile {
        path: "icons/ability/frighten-all.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ability/frighten-all.svg")),
    },
    StaticFile {
        path: "icons/ability/frighten-foot.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ability/frighten-foot.svg")),
    },
    StaticFile {
        path: "icons/ability/frighten-mounted.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ability/frighten-mounted.svg")),
    },
    StaticFile {
        path: "icons/ability/heart.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ability/heart.svg")),
    },
    StaticFile {
        path: "icons/ability/hide-anywhere.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ability/hide-anywhere.svg")),
    },
    StaticFile {
        path: "icons/ability/hide-forest.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ability/hide-forest.svg")),
    },
    StaticFile {
        path: "icons/ability/hide-grass.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ability/hide-grass.svg")),
    },
    StaticFile {
        path: "icons/ability/cant-hide.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ability/cant-hide.svg")),
    },
    StaticFile {
        path: "icons/ability/power-charge.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ability/power-charge.svg")),
    },
    StaticFile {
        path: "icons/ability/warcry.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ability/warcry.svg")),
    },
    StaticFile {
        path: "icons/attribute/against-cavalry.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/attribute/against-cavalry.svg")),
    },
    StaticFile {
        path: "icons/attribute/ammo.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/attribute/ammo.svg")),
    },
    StaticFile {
        path: "icons/attribute/armor-piercing.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/attribute/armor-piercing.svg")),
    },
    StaticFile {
        path: "icons/attribute/armor.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/attribute/armor.svg")),
    },
    StaticFile {
        path: "icons/attribute/charge.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/attribute/charge.svg")),
    },
    StaticFile {
        path: "icons/attribute/heat.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/attribute/heat.svg")),
    },
    StaticFile {
        path: "icons/attribute/inexhaustible.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/attribute/inexhaustible.svg")),
    },
    StaticFile {
        path: "icons/attribute/precharge.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/attribute/precharge.svg")),
    },
    StaticFile {
        path: "icons/attribute/range.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/attribute/range.svg")),
    },
    StaticFile {
        path: "icons/attribute/shield.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/attribute/shield.svg")),
    },
    StaticFile {
        path: "icons/attribute/skill.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/attribute/skill.svg")),
    },
    StaticFile {
        path: "icons/attribute/stamina.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/attribute/stamina.svg")),
    },
    StaticFile {
        path: "icons/attribute/turns.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/attribute/turns.svg")),
    },
    StaticFile {
        path: "icons/class/artillery.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/class/artillery.svg")),
    },
    StaticFile {
        path: "icons/class/beasts.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/class/beasts.svg")),
    },
    StaticFile {
        path: "icons/class/general.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/class/general.svg")),
    },
    StaticFile {
        path: "icons/class/horses.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/class/horses.svg")),
    },
    StaticFile {
        path: "icons/class/missiles.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/class/missiles.svg")),
    },
    StaticFile {
        path: "icons/class/ship.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/class/ship.svg")),
    },
    StaticFile {
        path: "icons/class/spears.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/class/spears.svg")),
    },
    StaticFile {
        path: "icons/class/swords.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/class/swords.svg")),
    },
    StaticFile {
        path: "icons/discipline/low.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/discipline/low.svg")),
    },
    StaticFile {
        path: "icons/discipline/normal.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/discipline/normal.svg")),
    },
    StaticFile {
        path: "icons/discipline/disciplined.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/discipline/disciplined.svg")),
    },
    StaticFile {
        path: "icons/discipline/impetuous.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/discipline/impetuous.svg")),
    },
    StaticFile {
        path: "icons/discipline/berserker.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/discipline/berserker.svg")),
    },
    StaticFile {
        path: "icons/exp/exp1.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/exp/exp1.svg")),
    },
    StaticFile {
        path: "icons/exp/exp2.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/exp/exp2.svg")),
    },
    StaticFile {
        path: "icons/exp/exp3.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/exp/exp3.svg")),
    },
    StaticFile {
        path: "icons/exp/exp4.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/exp/exp4.svg")),
    },
    StaticFile {
        path: "icons/exp/exp5.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/exp/exp5.svg")),
    },
    StaticFile {
        path: "icons/exp/exp6.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/exp/exp6.svg")),
    },
    StaticFile {
        path: "icons/exp/exp7.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/exp/exp7.svg")),
    },
    StaticFile {
        path: "icons/exp/exp8.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/exp/exp8.svg")),
    },
    StaticFile {
        path: "icons/exp/exp9.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/exp/exp9.svg")),
    },
    StaticFile {
        path: "icons/formation/horde.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/formation/horde.svg")),
    },
    StaticFile {
        path: "icons/formation/square.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/formation/square.svg")),
    },
    StaticFile {
        path: "icons/formation/phalanx.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/formation/phalanx.svg")),
    },
    StaticFile {
        path: "icons/formation/wedge.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/formation/wedge.svg")),
    },
    StaticFile {
        path: "icons/formation/testudo.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/formation/testudo.svg")),
    },
    StaticFile {
        path: "icons/formation/schiltrom.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/formation/schiltrom.svg")),
    },
    StaticFile {
        path: "icons/formation/shield-wall.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/formation/shield-wall.svg")),
    },
    StaticFile {
        path: "icons/stat/soldiers.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/stat/soldiers.svg")),
    },
    StaticFile {
        path: "icons/stat/cost.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/stat/cost.svg")),
    },
    StaticFile {
        path: "icons/stat/upkeep.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/stat/upkeep.svg")),
    },
    StaticFile {
        path: "icons/stat/defense.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/stat/defense.svg")),
    },
    StaticFile {
        path: "icons/stat/defense2.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/stat/defense2.svg")),
    },
    StaticFile {
        path: "icons/stat/replenish.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/stat/replenish.svg")),
    },
    StaticFile {
        path: "icons/terrain/scrub-up.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/terrain/scrub-up.svg")),
    },
    StaticFile {
        path: "icons/terrain/scrub-down.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/terrain/scrub-down.svg")),
    },
    StaticFile {
        path: "icons/terrain/forest-up.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/terrain/forest-up.svg")),
    },
    StaticFile {
        path: "icons/terrain/forest-down.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/terrain/forest-down.svg")),
    },
    StaticFile {
        path: "icons/terrain/sand-up.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/terrain/sand-up.svg")),
    },
    StaticFile {
        path: "icons/terrain/sand-down.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/terrain/sand-down.svg")),
    },
    StaticFile {
        path: "icons/terrain/snow-up.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/terrain/snow-up.svg")),
    },
    StaticFile {
        path: "icons/terrain/snow-down.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/terrain/snow-down.svg")),
    },
    StaticFile {
        path: "icons/ui/back.png",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ui/back.png")),
    },
    StaticFile {
        path: "icons/ui/help.png",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ui/help.png")),
    },
    StaticFile {
        path: "icons/ui/settings.png",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/ui/settings.png")),
    },
    StaticFile {
        path: "icons/weapon/blade.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/weapon/blade.svg")),
    },
    StaticFile {
        path: "icons/weapon/missile.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/weapon/missile.svg")),
    },
    StaticFile {
        path: "icons/weapon/spear.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/weapon/spear.svg")),
    },
    StaticFile {
        path: "icons/weapon/thrown.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/weapon/thrown.svg")),
    },
];

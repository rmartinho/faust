use std::path::Path;

use anyhow::Result;
use askama::Template;

use crate::{
    render::renderer::{Preload, PreloadType},
    utils::write_file,
};

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

#[derive(Template)]
#[template(path = "preload.html")]
pub struct PrefetchHtml<'a> {
    pub preload: &'a [(String, Preload)],
}

pub struct StaticFile<'a> {
    pub path: &'a str,
    pub contents: &'a [u8],
    pub preload_as: Option<PreloadType>,
}

impl<'a> StaticFile<'a> {
    pub async fn create(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref().join(self.path);
        write_file(path, self.contents).await
    }
}

pub const FILESYSTEM_STATIC: &[StaticFile] = &[
    StaticFile {
        path: "favicon.webp",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/favicon.webp")),
        preload_as: None,
    },
    StaticFile {
        path: "styles/main.css",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/styles/main.css"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "scripts/silphium.js",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/scripts/silphium.js"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "scripts/silphium_bg.wasm",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/scripts/silphium_bg.wasm"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "fonts/blinker-regular-1.woff2",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/fonts/blinker-regular-1.woff2"
        )),
        preload_as: Some(PreloadType::Woff2),
    },
    StaticFile {
        path: "fonts/blinker-regular-2.woff2",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/fonts/blinker-regular-2.woff2"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "fonts/blinker-bold-1.woff2",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/fonts/blinker-bold-1.woff2"
        )),
        preload_as: Some(PreloadType::Woff2),
    },
    StaticFile {
        path: "fonts/blinker-bold-2.woff2",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/fonts/blinker-bold-2.woff2"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "icons/ability.svg",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/icons/ability.svg"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "icons/attribute.svg",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/icons/attribute.svg"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "icons/class.svg",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/icons/class.svg"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "icons/discipline.svg",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/icons/discipline.svg"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "icons/exp.svg",
        contents: include_bytes!(concat!(env!("OUT_DIR"), "/silphium_template/icons/exp.svg")),
        preload_as: None,
    },
    StaticFile {
        path: "icons/formation.svg",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/icons/formation.svg"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "icons/speed.svg",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/icons/speed.svg"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "icons/stat.svg",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/icons/stat.svg"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "icons/terrain.svg",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/icons/terrain.svg"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "icons/weapon.svg",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/icons/weapon.svg"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "icons/ui/back.webp",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/icons/ui/back.webp"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "icons/ui/mercs.webp",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/icons/ui/mercs.webp"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "icons/ui/help.webp",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/icons/ui/help.webp"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "icons/ui/settings.webp",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/icons/ui/settings.webp"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "icons/ui/horde.svg",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/icons/ui/horde.svg"
        )),
        preload_as: None,
    },
    StaticFile {
        path: "images/ui/example-unit.webp",
        contents: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/silphium_template/images/ui/example-unit.webp"
        )),
        preload_as: None,
    },
];

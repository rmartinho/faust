use std::net::SocketAddr;

use anyhow::{Context as _, Result};
use axum::Router;
use clipboard_rs::{Clipboard as _, ClipboardContext};
use console::style;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

use crate::{args::Config, utils::EARTH};

pub async fn serve(cfg: &Config) -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("binding socket to {addr}"))?;

    let url = format!("http://localhost:{PORT}/");
    {
        if let Ok(clipboard) = ClipboardContext::new() {
            let _ = clipboard.set_text(url.clone());
        }
    }

    println!(
        "      {EARTH}{} {}",
        style(format!("Browse your site at {}", style(url).cyan().bold())).green(),
        style("(this has been copied to your clipboard)").dim()
    );
    axum::serve(
        listener,
        Router::new().fallback_service(
            ServeDir::new(&cfg.out_dir)
                .not_found_service(ServeFile::new(&cfg.out_dir.join("404.html"))),
        ),
    )
    .await
    .context("serving HTTP")?;
    Ok(())
}

const PORT: u16 = 7777;

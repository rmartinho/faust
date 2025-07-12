use askama::Template as _;
use silphium::{Route, StaticApp, StaticAppProps};
use tokio::{fs, io};
use yew_router::Routable as _;

mod templates;

use crate::{
    env::Env,
    render::templates::{FILESYSTEM_STATIC, IndexHtml, RedirectHtml},
    utils::write_file,
};

pub async fn render_routes(env: &Env) -> io::Result<()> {
    for r in &env.routes {
        println!("{}", r.route.to_path());
        if let Some(ref target) = r.redirect {
            write_file(
                &env.out_dir.join(&r.path),
                RedirectHtml { target }.render()?,
            )
            .await?;
        } else {
            let body = &render_route(r.route.clone(), env).await;
            write_file(
                &env.out_dir.join(&r.path),
                IndexHtml { head: "", body }.render()?,
            )
            .await?;
        }
    }
    Ok(())
}

async fn render_route(route: Route, env: &Env) -> String {
    let props = StaticAppProps {
        route,
        data: env.data.clone().into(),
    };
    let renderer = yew::LocalServerRenderer::<StaticApp>::with_props(props);
    renderer.render().await
}

pub async fn create_directory(env: &Env) -> io::Result<()> {
    if env.out_dir.exists() {
        fs::remove_dir_all(&env.out_dir).await?;
    }
    fs::create_dir_all(&env.out_dir).await
}

pub async fn create_static_files(env: &Env) -> io::Result<()> {
    for file in FILESYSTEM_STATIC {
        file.create(&env.out_dir).await?;
    }
    Ok(())
}

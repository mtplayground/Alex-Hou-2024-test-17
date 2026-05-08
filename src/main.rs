#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use alex_hou_2024_test_17::{config::AppEnv, db};
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use alex_hou_2024_test_17::app::*;

    let app_env = AppEnv::load()?;
    app_env.init_tracing()?;
    let pool = db::connect(app_env.database_url()).await?;
    db::run_migrations(&pool).await?;

    let conf = get_configuration(None)?;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    let pool_for_context = pool.clone();

    let app = Router::new()
        .leptos_routes_with_context(&leptos_options, routes, move || {
            provide_context(pool_for_context.clone());
        }, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    tracing::info!(site_addr = %addr, "starting Leptos Axum server");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}

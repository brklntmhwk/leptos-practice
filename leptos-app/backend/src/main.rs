mod file_server;

use app::*;
use axum::{
    body::Body as AxumBody,
    extract::{FromRef, Path, RawQuery, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use entity::db::{DBConfig, DB};
use file_server::file_and_error_handler;
use http::{header, HeaderMap, Request};
use leptos::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use tower_http::{
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub db: DB,
}

async fn server_fn_handler(
    State(app_state): State<AppState>,
    path: Path<String>,
    headers: HeaderMap,
    raw_query: RawQuery,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    tracing::info!("serverfn: {:?}", path);
    handle_server_fns_with_context(
        path,
        headers,
        raw_query,
        move || {
            provide_context(app_state.db.clone());
        },
        request,
    )
    .await
}

async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        app_state.leptos_options.clone(),
        move || {
            provide_context(app_state.db.clone());
        },
        || view! { <App/> },
    );
    handler(req).await.into_response()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Setting get_configuration(None) means using cargo-leptos's env values
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|| view! { <App/> });

    let db_conf = DBConfig::figment().extract::<DBConfig>().unwrap();
    let db = DB::connect(&db_conf).await.unwrap();
    db.run_migrations().await.unwrap();

    let app_state = AppState {
        leptos_options,
        db: db.clone(),
    };

    // build an application with a route, and states of DB and Leptos config
    // * This mustn't return Router<LeptosOptions> but Router<()>,
    // * otherwise the app can't have the method into_make_service()
    let app = Router::new()
        .route("/api/*fn_name", post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .layer(SetSensitiveRequestHeadersLayer::new(vec![
            header::AUTHORIZATION,
            header::COOKIE,
        ]))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(app_state);

    // `axum::Server` is a re-export of `hyper::Server`
    tracing::info!("listening on **http://{}**", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

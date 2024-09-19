use std::sync::Arc;

use axum::{
    middleware::from_fn,
    routing::{get, post},
    Extension, Router,
};
use tower_http::services::ServeDir;

use crate::{
    controller::{
        middleware::{jwt::auth, limiter::limiter, logger::logger},
        pem, ping, todo, user,
    },
    core::dep::Dep,
};

pub async fn create() -> (Router, Arc<Dep>) {
    let dep = Arc::new(Dep::new().await);

    let fs = Router::new().nest_service(
        &dep.env.dir_asset,
        ServeDir::new(dep.env.dir_private.clone() + &dep.env.dir_asset),
    );

    (
        Router::new()
            .nest("/api", router_api())
            .nest("/open", router_open())
            .fallback_service(fs)
            .layer(from_fn(limiter))
            .layer(from_fn(logger))
            .layer(Extension(dep.clone())),
        dep,
    )
}

fn router_open() -> Router {
    Router::new()
        .route("/pem", get(pem::get_public_pem))
        .route("/sign", post(user::sign))
        .route("/login", post(user::login))
}

fn router_api() -> Router {
    Router::new()
        .route("/ping", get(ping::ping))
        .nest("/v1", router_api_v1())
        .nest("/v2", router_api_v2())
        .layer(from_fn(auth))
}

fn router_api_v1() -> Router {
    let router_todo = Router::new()
        .route("/add", post(todo::add))
        .route("/put", post(todo::put))
        .route("/del", post(todo::del))
        .route("/list", post(todo::list));

    let router_user = Router::new();
    let router_blog = Router::new();

    Router::new()
        .nest("/todo", router_todo)
        .nest("/user", router_user)
        .nest("/blog", router_blog)
}

fn router_api_v2() -> Router {
    let router_todo = Router::new().route("/list", get(|| async { "v2 todo list" }));

    Router::new().nest("/todo", router_todo)
}

mod ws;
mod handler;
mod model;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock};
use warp::{Filter};
use crate::model::Clients;

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    // GET /health
    let health_route = warp::path!("health").and_then(handler::health_handler);

    // POST /register
    let register = warp::path("register");
    let register_routes = register
        .and(warp::post())
        .and(warp::body::json())
        .and(ws::with_clients(clients.clone()))
        .and_then(handler::register_handler)
        .or(register
            .and(warp::delete())
            .and(warp::path::param())
            .and(ws::with_clients(clients.clone()))
            .and_then(handler::unregister_handler)
        );

    // POST /publish
    let publish = warp::path!("publish")
        .and(warp::body::json())
        .and(ws::with_clients(clients.clone()))
        .and_then(handler::publish_handler);

    // GET /ws/:id
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(ws::with_clients(clients.clone()))
        .and_then(handler::ws_handler);

    // Combine routers
    let routes = health_route
        .or(register_routes)
        .or(ws_route)
        .or(publish)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use log::{LevelFilter, Level};
use red_siren_lib::app;

fn main() {
    
    #[cfg(feature = "web")]
    {
        let log_config = wasm_logger::Config::new(Level::Info);
        wasm_logger::init(log_config);
        dioxus_web::launch_with_props(
            app,
            get_root_props_from_document().unwrap_or_default(),
            dioxus_web::Config::new(),
        );

    }
    #[cfg(feature = "ssr")]
    {
        use std::net::SocketAddr;
        use axum::routing::*;
        dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
        
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
                let app = Router::new()
                    // Server side render the application, serve static assets, and register server functions
                    .serve_dioxus_application("", ServeConfigBuilder::new(app, ()));

                axum::Server::bind(&addr)
                    .serve(app.into_make_service())
                    .await
                    .unwrap();
            });
    }
}

mod api;
mod configs;
mod errors;
mod models;

use std::sync::Arc;

use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::{get, post},
    Router,
};
use shuttle_axum::ShuttleAxum;
use shuttle_runtime::{
    main as shuttle_main, Error as ShuttleError, SecretStore as ShuttleSecretStore,
    Secrets as ShuttleSecrets,
};
use tower_http::cors::{Any, CorsLayer};

use crate::api::handlers::{alive_handler, send_email_handler};
use crate::configs::AppConfigs;
use crate::errors::SecretError;

#[derive(Clone)]
pub struct AppState {
    configs: AppConfigs,
    secrets: AppSecrets,
}

impl AppState {
    pub fn configs(&self) -> &AppConfigs {
        &self.configs
    }

    pub fn secrets(&self) -> &AppSecrets {
        &self.secrets
    }
}

#[derive(Clone, Debug, Default)]
pub struct AppSecrets {
    pub smtp_host: String,
    pub smtp_port: String,
    pub smtp_user: String,
    pub smtp_pass: String,
}

impl AppSecrets {
    fn new(store: &ShuttleSecretStore) -> Result<Self, SecretError> {
        let smtp_host = store
            .get("SMTP_HOST")
            .ok_or_else(|| SecretError::MissingSecret("SMTP_HOST"))?;

        let smtp_port = store
            .get("SMTP_PORT")
            .ok_or_else(|| SecretError::MissingSecret("SMTP_PORT"))?;

        let smtp_user = store
            .get("SMTP_USER")
            .ok_or_else(|| SecretError::MissingSecret("SMTP_USER"))?;

        let smtp_pass = store
            .get("SMTP_PASS")
            .ok_or_else(|| SecretError::MissingSecret("SMTP_PASS"))?;

        Ok(Self {
            smtp_host,
            smtp_port,
            smtp_user,
            smtp_pass,
        })
    }
}

fn configure_cors(configs: &AppConfigs) -> CorsLayer {
    let origins: Vec<HeaderValue> = (configs.allow_cors_origins)
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.parse::<HeaderValue>()
                .unwrap_or_else(|_| HeaderValue::from_static("localhost"))
        })
        .collect();

    let cors_layer = if origins.is_empty() {
        CorsLayer::new().allow_origin(Any)
    } else {
        CorsLayer::new().allow_origin(origins)
    };

    cors_layer.allow_headers([CONTENT_TYPE]).allow_methods([
        Method::HEAD,
        Method::GET,
        Method::POST,
    ])
}

#[shuttle_main]
async fn axum(#[ShuttleSecrets] store: ShuttleSecretStore) -> ShuttleAxum {
    let configs = match AppConfigs::new(&"configs/Default") {
        Ok(config) => config,
        Err(cause) => return Err(ShuttleError::Custom(cause.into())),
    };
    let secrets = match AppSecrets::new(&store) {
        Ok(secret) => secret,
        Err(cause) => return Err(ShuttleError::Custom(cause.into())),
    };

    let cors_layer = configure_cors(&configs);

    let state = Arc::new(AppState { configs, secrets });

    let router: Router = Router::new()
        .route("/api/v1/alive", get(alive_handler))
        .route("/api/v1/send-email", post(send_email_handler))
        .layer(cors_layer)
        .with_state(state);

    Ok(router.into())
}

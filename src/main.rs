use std::env::var;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::Redirect,
    routing::{get, post},
    Router,
};
use eyre::Context;
use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;
use sqlx::PgPool;
use tower_http::services::ServeDir;
use url::{Host, Url};

mod encoder;
mod ui;

#[derive(Debug, Clone)]
struct Config {
    /// Host the service is running at
    host: Host,
    /// Port the service is running on
    port: u16,
    /// An url from which the service can be reached
    visible_host: Url,
    /// Connection string to the database
    database_url: SecretBox<str>,
}

impl Config {
    fn try_create() -> eyre::Result<Self> {
        let host = Host::parse(
            &var("SHORTY_HOST").wrap_err("Could not get `SHORTY_HOST` environment variable")?,
        )
        .wrap_err("The `SHORTY_HOST` should be a valid host")?;

        let port = var("SHORTY_PORT")
            .wrap_err("Could not get `SHORTY_PORT` environment variable")?
            .parse()
            .wrap_err("The `SHORTY_PORT` environment variable should be a valid port number")?;

        let visible_host = var("SHORTY_VISIBLE_HOST")
            .wrap_err("Could not get `SHORTY_VISIBLE_HOST` environment variable")?
            .parse()
            .wrap_err("The `SHORTY_VISIBLE_HOST` must be a valid url")?;

        let database_url = var("SHORTY_DATABASE_URL")
            .wrap_err("Could not get `SHORTY_DATABASE_URL` environment variable")?;

        Ok(Self {
            host,
            port,
            visible_host,
            database_url: SecretBox::new(database_url.into()),
        })
    }
}

#[derive(Debug, Clone)]
struct AppState {
    url_service: UrlService,
    config: Config,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::try_create()?;

    let pool = PgPool::connect(config.database_url.expose_secret())
        .await
        .wrap_err("Failed to setup database pool")?;

    sqlx::migrate!("./migrations/")
        .run(&pool)
        .await
        .wrap_err("Failed to perform migrations")?;

    let app = Router::new()
        .route("/s", post(store))
        .route("/s/:name", get(visit))
        .route("/ui", get(ui::ui))
        .nest_service("/ui/static", ServeDir::new("static"))
        .with_state(AppState {
            url_service: UrlService { pool },
            config: config.clone(),
        });

    tracing::info!("Booting up");

    let listener = tokio::net::TcpListener::bind(&format!("{}:{}", config.host, config.port))
        .await
        .wrap_err("Could not bind to host")?;

    tracing::info!("Shorty is now live");

    axum::serve(listener, app)
        .await
        .wrap_err("An error occurred while running the server")?;

    Ok(())
}

async fn visit(
    Path(name): Path<String>,
    State(AppState { url_service, .. }): State<AppState>,
) -> Result<Redirect, StatusCode> {
    match url_service.get(name).await {
        Ok(Some(url)) => Ok(Redirect::to(url.as_ref())),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(err) => {
            tracing::error!(?err, "Could not fetch url");
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

#[derive(Debug, Deserialize)]
struct StoreRequest {
    url: Url,
}

async fn store(
    State(AppState {
        url_service,
        config,
        ..
    }): State<AppState>,
    Json(body): Json<StoreRequest>,
) -> Result<String, StatusCode> {
    let short = url_service.store(body.url).await.map_err(|error| {
        tracing::error!(?error, "Failed to store url");
        StatusCode::SERVICE_UNAVAILABLE
    })?;

    let url = config
        .visible_host
        .join(&format!("/s/{short}"))
        .map_err(|error| {
            tracing::error!(?error, "Could not parse url");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(url.to_string())
}

#[derive(Debug, Clone)]
struct UrlService {
    pool: PgPool,
}

impl UrlService {
    async fn get(&self, short: String) -> eyre::Result<Option<String>> {
        struct Response {
            url: String,
        }

        let id = encoder::decode(&short).wrap_err("Could not decode provided short")?;

        let url = sqlx::query_as!(Response, "SELECT url FROM url WHERE url_id = $1", id)
            .fetch_optional(&self.pool)
            .await?
            .map(|res| res.url);

        Ok(url)
    }

    async fn store(&self, url: Url) -> eyre::Result<String> {
        struct Response {
            url_id: i64,
        }

        let response = sqlx::query_as!(
            Response,
            "INSERT INTO url (url) VALUES ($1) RETURNING url_id",
            url.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(encoder::encode(response.url_id))
    }
}

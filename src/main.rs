#![feature(duration_constructors)]

use axum::{
    Router,
    body::Body,
    http::StatusCode,
    extract::{
        ConnectInfo, FromRequest, Multipart, Query, Request, State,
        multipart::{
            MultipartError, MultipartRejection
        }
    },
    response::{IntoResponse, Response},
    routing::{get, post}
};
use chrono::Utc;
use glc::server::{setup_logging, serve, SpanMaker};
use serde::Deserialize;
use std::{
    fs,
    io,
    net::{IpAddr, SocketAddr},
    path::Path,
    sync::Arc,
    time::Duration
};
use sqlx::{
    Executor, Pool,
    sqlite::{Sqlite, SqlitePoolOptions}
};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    trace::{DefaultOnFailure, DefaultOnResponse, TraceLayer}
};
use tracing::{error, info, Level};

#[derive(Clone)]
pub struct AppState {
    game_server_ip: String,
    motd_path: String,
    db: Pool<Sqlite>
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub base_path: String,
    pub listen_ip: String,
    pub listen_port: u16,
    pub log_headers: bool,
    pub game_server_ip: String,
    pub motd_path: String,
    pub db_path: String
}

#[derive(Debug, thiserror::Error)]
enum StartupError {
    #[error("{0}")]
    AddrParse(#[from] std::net::AddrParseError),
    #[error("{0}")]
    TomlParse(#[from] toml::de::Error),
    #[error("{0}")]
    Database(#[from] sqlx::Error),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("base_path must not be /")]
    BadBasePath
}

// TODO: make the one in gls public, use that
fn real_addr(request: &Request) -> String {
    // If we're behind a proxy, get IP from X-Forwarded-For header
    match request.headers().get("x-forwarded-for") {
        Some(addr) => addr.to_str()
            .map(String::from)
            .ok(),
        None => request.extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|info| info.ip().to_string())
    }
    .unwrap_or_else(|| "<unknown>".into())
}

#[derive(Deserialize)]
struct ConnectionHistoryArgs {
    start: Option<u32>,
    end: Option<u32>
}

#[derive(Debug, Deserialize)]
struct Connection {
    module_name: String,
    game_room: String,
    player_name: String,
    time: i64
}

#[derive(Debug, thiserror::Error)]
pub enum ConnectionHistoryError {
    #[error("Missing start")]
    EndWithoutStart,
    #[error("Missing end")]
    StartWithoutEnd,
    #[error("Start not less than end")]
    StartNotLessThanEnd,
    #[error("{0}")]
    SqlxError(#[from] sqlx::Error),
}

impl IntoResponse for ConnectionHistoryError {
    fn into_response(self) -> Response {
        error!("{}", self);

        let status = match self {
            Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST
        };

        (status, self.to_string()).into_response()
    }
}

async fn get_connections<'e, E>(
    ex: E
) -> Result<Vec<Connection>, sqlx::Error>
where
    E: Executor<'e, Database = Sqlite>
{
    sqlx::query_as!(
        Connection,
        "
SELECT
    module_name,
    game_room,
    player_name,
    time
FROM connections
        " 
    )
    .fetch_all(ex)
    .await
}

async fn get_connections_between<'e, E>(
    ex: E,
    start: i64,
    end: i64
) -> Result<Vec<Connection>, sqlx::Error>
where
    E: Executor<'e, Database = Sqlite>
{
    sqlx::query_as!(
        Connection,
        "
SELECT
    module_name,
    game_room,
    player_name,
    time
FROM connections
WHERE time BETWEEN ? AND ?
        ",
        start,
        end + 1
    )
    .fetch_all(ex)
    .await
}

async fn get_connection_history(
    Query(query): Query<ConnectionHistoryArgs>,
    State(state): State<Arc<AppState>>
) -> Result<String, ConnectionHistoryError> {

    let conns = if let Some(end) = query.end {
        if let Some(start) = query.start {
            if end <= start {
                return Err(ConnectionHistoryError::StartNotLessThanEnd);
            }

            get_connections_between(&state.db, start as i64, end as i64).await?
        }
        else {
            return Err(ConnectionHistoryError::EndWithoutStart);
        }
    }
    else if query.start.is_some() {
        return Err(ConnectionHistoryError::StartWithoutEnd);
    }
    else { 
        get_connections(&state.db).await?
    };

    // FIXME: this doesn't properly escape tabs
    // FIXME: should stream instead of constructing huge response?
    Ok(
        conns
            .into_iter()
            .map(|c| format!(
                "{}\t{}\t{}\t{}",
                c.module_name,
                c.game_room,
                c.player_name,
                c.time
            ))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

#[derive(Debug, Deserialize)]
struct CurrentConnection {
    module_name: String,
    game_room: String,
    player_name: String
}

async fn get_connections_current<'e, E>(
    ex: E,
    when: Duration
) -> Result<Vec<CurrentConnection>, sqlx::Error>
where
    E: Executor<'e, Database = Sqlite>
{
    match when {
        Duration::ZERO => sqlx::query_as!(
            CurrentConnection,
                "
SELECT
    module_name,
    game_room,
    player_name
FROM connections
WHERE time = (SELECT MAX(time) FROM connections)
ORDER BY module_name, game_room, player_name
                "
            )
            .fetch_all(ex)
            .await,

        _ => sqlx::query_as!(
            CurrentConnection,
                "
SELECT
    module_name,
    game_room,
    player_name
FROM connections
WHERE unixepoch() * 1000 - time <= ? 
ORDER BY module_name, game_room, player_name
                ",
                when.as_millis() as i64
            )
            .fetch_all(ex)
            .await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CurrentConnectionsError {
    #[error("{0}")]
    SqlxError(#[from] sqlx::Error),
}

impl IntoResponse for CurrentConnectionsError {
    fn into_response(self) -> Response {
        error!("{}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

#[derive(Deserialize)]
struct CurrentConnectionsArgs {
    when: Option<String>
}

async fn get_current_connections(
    Query(query): Query<CurrentConnectionsArgs>,
    State(state): State<Arc<AppState>>
) -> Result<String, CurrentConnectionsError> {

    let d = match query.when.as_deref() {
        Some("day") => Duration::from_days(1),
        Some("week") => Duration::from_days(7),
        Some("month") => Duration::from_days(30),
        _ => Duration::ZERO
    };

    let conns = get_connections_current(&state.db, d).await?;

    Ok(
        conns
            .into_iter()
            .map(|c| format!(
                "{}\t{}\t{}",
                c.module_name,
                c.game_room,
                c.player_name
            ))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

#[derive(Deserialize)]
struct GetMessagesArgs {
    module: String 
}

#[derive(Debug, Deserialize)]
struct Message {
    sender: String,
    time: i64,
    content: String
}

async fn get_messages_for<'e, E>(
    ex: E,
    module: &str
) -> Result<Vec<Message>, sqlx::Error>
where
    E: Executor<'e, Database = Sqlite>
{
    sqlx::query_as!(
        Message, 
        "
SELECT
    sender,
    time,
    content
FROM messages 
WHERE module_name = ?
        ",
        module
    )
    .fetch_all(ex)
    .await
}

#[derive(Debug, thiserror::Error)]
pub enum GetMessagesError {
    #[error("{0}")]
    SqlxError(#[from] sqlx::Error),
}

impl IntoResponse for GetMessagesError {
    fn into_response(self) -> Response {
        error!("{}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

async fn get_messages(
    Query(query): Query<GetMessagesArgs>,
    State(state): State<Arc<AppState>>
) -> Result<String, GetMessagesError> {
    
    Ok( 
        get_messages_for(&state.db, &query.module)
            .await?
            .into_iter()
            .map(|m| format!(
                "SENDER={}&DATE={}&CONTENT={}",
                m.sender,
                m.time,
                m.content
            ))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

async fn get_my_address(request: Request<Body>) -> String {
    real_addr(&request)
}

#[derive(Deserialize)]
struct GetMotdArgs {
    module: Option<String>
}

#[derive(Debug, thiserror::Error)]
pub enum GetMotdError {
    #[error("{0}")]
    IoError(#[from] std::io::Error)
}

impl IntoResponse for GetMotdError {
    fn into_response(self) -> Response {
        error!("{}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

async fn get_motd(
    Query(query): Query<GetMotdArgs>,
    State(state): State<Arc<AppState>>
) -> Result<String, GetMotdError> {

    let filename = match query.module.as_deref() {
        Some("VASL") => "VASL",
        _ => "VASSAL"
    };

    let path = Path::new(&state.motd_path).join(filename);
    Ok(fs::read_to_string(path)?)
}

#[derive(Debug, thiserror::Error)]
pub enum PostMessageError {
    #[error("{0}")]
    MultipartError(#[from] MultipartError),
    #[error("{0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("Missing sender")]
    MissingSender,
    #[error("Missing module")]
    MissingModule,
    #[error("Missing content")]
    MissingContent,

}

impl IntoResponse for PostMessageError {
    fn into_response(self) -> Response {
        error!("{}", self);
        let status = match self {
            Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST
        };

        (status, self.to_string()).into_response()
    }
}

async fn insert_message<'e, E>(
    ex: E,
    module: &str,
    sender: &str,
    content: &str
) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Sqlite>
{
    sqlx::query!(
        "
INSERT INTO messages (
    module_name,
    sender,
    content
)
VALUES (?, ?, ?)
        ",
        module,
        sender,
        content
    )
    .execute(ex)
    .await?;

    Ok(())
}

async fn post_message(
    State(state): State<Arc<AppState>>,
    mut form: Multipart
) -> Result<(), PostMessageError> {

    let mut sender = None;
    let mut module = None;
    let mut content = None;

    while let Some(field) = form.next_field().await? {
        match field.name() {
            Some("sender") => sender = Some(field.text().await?),
            Some("module") => module = Some(field.text().await?),
            Some("content") => content = Some(field.text().await?),
            _ => {}
        }
    }

    let sender = sender.ok_or(PostMessageError::MissingSender)?;
    let module = module.ok_or(PostMessageError::MissingModule)?;
    let content = content.ok_or(PostMessageError::MissingContent)?;

    insert_message(&state.db, &module, &sender, &content).await?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateConnectionsError {
    #[error("{0}")]
    MultipartError(#[from] MultipartError),
    #[error("{0}")]
    MultipartRejection(#[from] MultipartRejection),
    #[error("{0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("Missing STATUS")]
    MissingStatus,
    #[error("You are not our game server")]
    NotOurGameServer,
    #[error("Bad status line")]
    BadStatusLine
}

impl IntoResponse for UpdateConnectionsError {
    fn into_response(self) -> Response {
        error!("{}", self);
        let status = match self {
            Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotOurGameServer => StatusCode::FORBIDDEN,
            _ => StatusCode::BAD_REQUEST
        };

        (status, self.to_string()).into_response()
    }
}

async fn insert_connection<'e, E>(
    ex: E,
    module_name: &str,
    game_room: &str,
    player_name: &str,
    time: i64
) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Sqlite>
{
    sqlx::query!(
        "
INSERT INTO connections (
    time, 
    game_room,
    module_name,
    player_name
)
VALUES (?, ?, ?, ?)
ON CONFLICT DO UPDATE SET time = ?
        ",
        time,
        game_room,
        module_name,
        player_name,
        time
    )
    .execute(ex)
    .await?;

    Ok(())
}

#[axum::debug_handler]
async fn post_update_connections(
    State(state): State<Arc<AppState>>,
    request: Request<Body>
) -> Result<(), UpdateConnectionsError>
{
    let addr = real_addr(&request);

    if addr != state.game_server_ip {
        return Err(UpdateConnectionsError::NotOurGameServer);
    }

    let mut form = Multipart::from_request(request, &state).await?;

    let mut status = None;

    while let Some(field) = form.next_field().await? {
        if let Some("STATUS") = field.name() {
            status = Some(field.text().await?);
        }
    }

    let status = status.ok_or(UpdateConnectionsError::MissingStatus)?;

    let now = Utc::now().timestamp_millis();

    for l in status.lines() {
        let mut s = l.split('\t');

        let Some(module_name) = s.next() else {
            return Err(UpdateConnectionsError::BadStatusLine);
        };

        let Some(game_room) = s.next() else {
            return Err(UpdateConnectionsError::BadStatusLine);
        };

        let Some(player_name) = s.next() else {
            return Err(UpdateConnectionsError::BadStatusLine);
        };

        if !s.next().is_none() {
            return Err(UpdateConnectionsError::BadStatusLine);
        }

        insert_connection(
            &state.db,
            module_name,
            game_room,
            player_name,
            now
        ).await?;
    }

    Ok(())
}

fn routes(base_path: &str, log_headers: bool) -> Router<Arc<AppState>> {
    Router::new()
        .route(
            &format!("{base_path}/util/getConnectionHistory"),
            get(get_connection_history)
        )
        .route(
            &format!("{base_path}/util/getCurrentConnections"),
            get(get_current_connections)
        )
        .route(
            &format!("{base_path}/util/getMessages"),
            get(get_messages)
        )
        .route(
            &format!("{base_path}/util/getMyAddress"),
            get(get_my_address)
        )
        .route(
            &format!("{base_path}/util/motd"),
            get(get_motd)
        )
        .route(
            &format!("{base_path}/util/postMessage"),
            post(post_message)
        )
        .route(
            &format!("{base_path}/util/updateConnections"),
            post(post_update_connections)
        )
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new())
                 // ensure requests don't block shutdown
                .layer(TimeoutLayer::with_status_code(
                    StatusCode::REQUEST_TIMEOUT,
                    Duration::from_secs(10)
                ))
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(SpanMaker::new().include_headers(log_headers))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::WARN))
        )
}

async fn run() -> Result<(), StartupError> {
    info!("Reading config.toml");
    let config: Config = toml::from_str(&fs::read_to_string("config.toml")?)?;

    if config.base_path == "/" {
        return Err(StartupError::BadBasePath);
    }

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite://{}", config.db_path))
        .await?;

    let state = Arc::new(AppState {
        game_server_ip: config.game_server_ip,
        motd_path: config.motd_path,
        db
    });

    // set up router
    let app = routes(
        &config.base_path,
        config.log_headers
    )
    .with_state(state);

    // serve pages
    let ip: IpAddr = config.listen_ip.parse()?;
    serve(app, ip, config.listen_port).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    // set up logging
    let _guard = setup_logging(env!("CARGO_CRATE_NAME"), "", "util.log");

    info!("Starting");

    if let Err(e) = run().await {
        error!("{}", e);
    }

    info!("Exiting");
}

#[cfg(test)]
mod test {
}

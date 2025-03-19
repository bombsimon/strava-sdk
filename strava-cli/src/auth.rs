//! This is just a wrapper around [strava_client_rs] to be able to grab a token. The
//! strava_client_rs crate also contains some methods to work with the API but they're mostly
//! incomplete and hand written without the schema.
//!
//! Since the tool had a decent way of getting OAuth token and store the refresh token to avoid
//! multiple consents it's currently being used here.
use std::path::Path;

use strava_client_rs::api::auth;
use strava_client_rs::util::auth_config::config_file;

const AUTH_URL: &str = "http://www.strava.com/oauth/authorize";
const TOKEN_URL: &str = "https://www.strava.com/oauth/token";

type TokenFn = fn(auth::Config) -> Result<String, String>;

pub async fn get_token(
    config_path: Option<&str>,
    client_id: &str,
    client_secret: &str,
) -> anyhow::Result<String> {
    let config_file = config_path.unwrap_or("config.json");

    let (refresh_token, token_fn): (String, TokenFn) = if Path::new(&config_file).exists() {
        (
            config_file::load_config().refresh_token,
            auth::get_refresh_token,
        )
    } else {
        (Default::default(), auth::get_authorization)
    };

    let config = auth::Config::new(
        client_id.to_string(),
        client_secret.to_string(),
        refresh_token,
        AUTH_URL.to_string(),
        TOKEN_URL.to_string(),
    );

    tokio::task::spawn_blocking(move || token_fn(config).map_err(|err| anyhow::anyhow!(err)))
        .await?
}


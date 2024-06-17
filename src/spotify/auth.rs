use std::env;
use std::fs::create_dir_all;
use std::path::PathBuf;
use rspotify::{AuthCodeSpotify, Config, Credentials, DEFAULT_CACHE_PATH, OAuth, scopes};
use rspotify::clients::OAuthClient;

pub const DEFAULT_CONFIG_FOLDER: &str = "~/.config/knob_rocker";
pub const DEFAULT_CONFIG_PATH: &str = "~/.config/knob_rocker/config.toml";

fn save_credentials_to_config(creds: &Credentials) {
    let config_path = resolve_home_path(PathBuf::from(DEFAULT_CONFIG_PATH));
    let config_folder = config_path.parent().expect("Could not get parent folder of config path");

    create_dir_all(config_folder).expect("Could not create config folder");

    let client_id = creds.id.clone();
    let client_secret = creds.secret.clone().expect("Could not get client secret");

    let config = format!("client_id = \"{}\"\nclient_secret = \"{}\"", client_id, client_secret);

    std::fs::write(config_path, config).expect("Could not write to config file");
}

fn prompt_for_app_credentials() -> Credentials {
    let client_id = dialoguer::Input::<String>::new()
        .with_prompt("Enter your Spotify client ID")
        .interact()
        .expect("Could not read client ID");

    let client_secret = dialoguer::Password::new()
        .with_prompt("Enter your Spotify client secret")
        .interact()
        .expect("Could not read client secret");

    let creds = Credentials {
        id: client_id,
        secret: Option::from(client_secret),
    };

    save_credentials_to_config(&creds);

    creds
}

fn get_credentials_from_config() -> Option<Credentials> {
    let config_path = resolve_home_path(PathBuf::from(DEFAULT_CONFIG_PATH));
    let config = std::fs::read_to_string(config_path);

    if config.is_err() {
        return None;
    }

    let config = config.unwrap();
    let config: toml::Value = toml::from_str(&config).expect("Could not parse config file");

    let client_id = config.get("client_id").and_then(|v| v.as_str());
    let client_secret = config.get("client_secret").and_then(|v| v.as_str());

    match (client_id, client_secret) {
        (Some(id), Some(secret)) => {
            Some(Credentials {
                id: id.to_string(),
                secret: Option::from(secret.to_string()),
            })
        }
        _ => None,
    }
}

fn get_credentials() -> Credentials {
    // First check if the credentials are stored in config file
    if let Some(creds) = get_credentials_from_config() {
        return creds;
    }

    // If not, prompt the user for the credentials
    prompt_for_app_credentials()
}

pub async fn spotify_auth() -> Option<AuthCodeSpotify> {
    let scopes = scopes!("user-read-playback-state", "user-modify-playback-state");
    let oauth = OAuth {
        redirect_uri: "http://localhost".to_string(),
        scopes,
        ..Default::default()
    };
    let creds = get_credentials();

    let cache_folder = resolve_home_path(PathBuf::from(DEFAULT_CONFIG_FOLDER));

    create_dir_all(cache_folder.clone()).expect("Could not create cache folder");
    let cache_path = cache_folder.join(DEFAULT_CACHE_PATH);

    let config = Config {
        token_cached: true,
        token_refreshing: true,
        cache_path: cache_path.clone(),
        ..Default::default()
    };
    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    let url = spotify.get_authorize_url(true).unwrap();

    if spotify.prompt_for_token(&url).await.is_err() {
        log::error!("Failed to authenticate with Spotify");

        // Delete cache file if it exists
        if cache_path.exists() {
            std::fs::remove_file(cache_path).expect("Could not delete cache file");
        }

        return None;
    }

    Some(spotify)
}

fn resolve_home_path(path: PathBuf) -> PathBuf {
    if path.starts_with("~/") {
        let possible_vars = [
            "HOME", // most shells
            "HOMEPATH" // used by nushell
        ];

        let home_path = possible_vars.iter()
            .find_map(env::var_os)
            .expect("could not find the home variable to use for the default config path");

        let relative_path = path
            .to_str()
            .expect("invalid path specified")
            .to_string()
            .split_off(2);
        PathBuf::from(home_path).join(relative_path)
    } else {
        path
    }
}
use log::{error, info};
use rspotify::AuthCodeSpotify;
use rspotify::clients::OAuthClient;

pub async fn get_current_device_id(spotify: &AuthCodeSpotify) -> Option<String> {
    let devices = spotify.device().await;

    if devices.is_err() {
        error!("Could not fetch devices from Spotify.");
        return None;
    }

    let devices = devices.unwrap();
    let device = devices.iter().find(|d| d.is_active);

    let device = match device {
        Some(device) => device,
        None => {
            error!("Could not find active device.");
            return None;
        }
    };

    let device_id = match &device.id {
        Some(id) => id.clone(),
        None => {
            error!("Could not find device ID for active device.");
            return None;
        }
    };

    Some(device_id)
}

pub async fn get_volume(spotify: &AuthCodeSpotify, device_id: String) -> Option<u8> {
    let devices = spotify.device().await;

    if devices.is_err() {
        error!("Could not fetch devices from Spotify.");
        return None;
    }

    let devices = devices.unwrap();
    let device = devices.iter().find(|d| d.id.is_some() && d.id.as_ref().unwrap() == &device_id);

    let device = device.ok_or_else(|| {
        error!("Could not find device with ID: {:?}", device_id);
    }).unwrap();

    let volume = device.volume_percent.unwrap_or(0);

    Some(volume as u8)
}

pub async fn set_volume(spotify: &AuthCodeSpotify, volume: u8, device_id: String) {
    let response = spotify.volume(
        volume, Option::from(device_id.as_str()),
    ).await;

    match response {
        Ok(_) => {
            info!("Volume set to {}", volume);
        }
        Err(e) => {
            error!("Could not set volume: {}", e);
        }
    }
}

pub async fn raise_volume(spotify: &AuthCodeSpotify, raise_by: u8, device_id: String) {
    let current_volume = get_volume(spotify, device_id.clone()).await.unwrap();
    let mut new_volume = current_volume + raise_by;

    if new_volume > 100 {
        new_volume = 100;
    }

    set_volume(spotify, new_volume, device_id).await;
}

pub async fn lower_volume(spotify: &AuthCodeSpotify, lower_by: u8, device_id: String) {
    let current_volume = get_volume(spotify, device_id.clone()).await.unwrap();
    
    let new_volume = if (current_volume as i16) - (lower_by as i16) < 0 {
        0
    } else {
        current_volume - lower_by
    };


    set_volume(spotify, new_volume, device_id).await;
}
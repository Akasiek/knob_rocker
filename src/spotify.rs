use log::error;
use crate::volume;
use crate::spotify::api::{get_current_device_id, lower_volume, raise_volume};

mod auth;
mod api;

pub async fn set_spotify_volume() {
    let spotify = auth::spotify_auth().await.unwrap();

    let device_id = match get_current_device_id(&spotify).await {
        Some(device_id) => device_id,
        None => {
            error!("Could not find active device.");
            return;
        }
    };


    let volume = volume::get_mutex_value();

    if volume < 0 {
        lower_volume(&spotify, volume.unsigned_abs() as u8, device_id).await;
    } else {
        raise_volume(&spotify, volume as u8, device_id).await;
    }

    volume::set_mutex_value(0);
}
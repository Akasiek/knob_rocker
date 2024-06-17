use std::sync::Arc;
use std::time::Duration;
use log::info;
use tokio::sync::{Mutex, Notify};
use tokio::time::sleep;
use crate::spotify::set_spotify_volume;

pub fn spawn(timer_notifier: Arc<Notify>) {
    let timer_state = Arc::new(Mutex::new(false));

    tokio::spawn(async move {
        info!("Starting volume adjustment timer loop.");
        
        loop {
            let timer = sleep(Duration::from_millis(300));

            tokio::select! {
                _ = timer => {
                    let mut state = timer_state.lock().await;
                    
                    if *state {
                        info!("Timer expired, setting volume.");
                        *state = false;
                        set_spotify_volume().await;
                    }
                }
                _ = timer_notifier.notified() => {
                    let mut state = timer_state.lock().await;
                    
                    if !*state {
                        info!("Timer notified, setting state. Starting timer.");
                        *state = true;
                    } else {
                        info!("Timer notified, state already set. Resetting timer");
                    }
                }
            }
        }
    });
}
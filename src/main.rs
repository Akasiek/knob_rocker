use std::sync::Arc;
use std::thread;
use dotenvy::dotenv;
use tokio::sync::{Notify};
use tokio::sync::broadcast;

mod tray;
mod spotify;
mod volume;
mod hotkeys;
mod timer;

#[derive(Debug, Clone, PartialEq)]
enum Message {
    VolumeUp,
    VolumeDown,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    
    #[cfg(not(debug_assertions))]
    spotify::hide_console_window_after_auth().await;

    let (up_tx, up_rx) = broadcast::channel(32);
    let (down_tx, down_rx) = broadcast::channel(32);

    let timer_notifier = Arc::new(Notify::new());
    let timer_notifier_up = timer_notifier.clone();
    let timer_notifier_down = timer_notifier.clone();

    volume::spawn_up_adjustment(up_rx);
    volume::spawn_down_adjustment(down_rx);
    timer::spawn(timer_notifier);
    
    hotkeys::register_up_hotkey(up_tx.clone(), timer_notifier_up.clone());
    hotkeys::register_down_hotkey(down_tx.clone(), timer_notifier_down.clone());

    thread::spawn(move || {
        inputbot::handle_input_events();
    });

    tray::init_tray();
}


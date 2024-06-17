#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use std::thread;
use dotenvy::dotenv;
use log::info;
use tokio::sync::{Notify};
use windows_hotkeys::HotkeyManagerImpl;
use windows_hotkeys::singlethreaded::HotkeyManager;
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
    info!("Logger initialized.");

    let (up_tx, up_rx) = broadcast::channel(32);
    let (down_tx, down_rx) = broadcast::channel(32);

    let timer_notifier = Arc::new(Notify::new());
    let timer_notifier_up = timer_notifier.clone();
    let timer_notifier_down = timer_notifier.clone();

    volume::spawn_up_adjustment(up_rx);
    volume::spawn_down_adjustment(down_rx);
    timer::spawn(timer_notifier);

    thread::spawn(move || {
        let mut hkm = HotkeyManager::new();

        hotkeys::register_up_hotkey(&mut hkm, up_tx.clone(), timer_notifier_up.clone());
        hotkeys::register_down_hotkey(&mut hkm, down_tx.clone(), timer_notifier_down.clone());

        info!("Hotkeys registered. Starting event loop.");

        hkm.event_loop();
    });

    tray::init_tray();
}


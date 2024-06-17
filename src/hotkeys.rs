use std::sync::Arc;
use log::{error, info};
use tokio::sync::broadcast::Sender;
use tokio::sync::Notify;
use windows_hotkeys::HotkeyManagerImpl;
use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::singlethreaded::HotkeyManager;
use crate::Message;

pub fn register_up_hotkey(hkm: &mut HotkeyManager<()>, up_sender: Sender<Message>, timer_notifier_up: Arc<Notify>) {
    hkm.register(VKey::F22, &[ModKey::NoRepeat], move || {
        match up_sender.send(Message::VolumeUp) {
            Ok(_) => {
                info!("Volume up hotkey pressed. Notifying timer.");
                timer_notifier_up.notify_one();
            }
            Err(e) => {
                error!("Failed to send notification: {}", e);
            }
        }
    }).unwrap();
}

pub fn register_down_hotkey(hkm: &mut HotkeyManager<()>, down_sender: Sender<Message>, timer_notifier_down: Arc<Notify>) {
    hkm.register(VKey::F21, &[ModKey::NoRepeat], move || {
        match down_sender.send(Message::VolumeDown) {
            Ok(_) => {
                info!("Volume down hotkey pressed. Notifying timer.");
                timer_notifier_down.notify_one();
            }
            Err(e) => {
                error!("Failed to send notification: {}", e);
            }
        }
    }).unwrap();
}
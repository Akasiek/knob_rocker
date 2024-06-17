use std::sync::Arc;
use inputbot::KeybdKey::{F21Key, F22Key};
use log::{error, info};
use tokio::sync::broadcast::Sender;
use tokio::sync::Notify;
use crate::Message;

pub fn register_up_hotkey(up_sender: Sender<Message>, timer_notifier_up: Arc<Notify>) {    
    F21Key.bind(move || {
        match up_sender.send(Message::VolumeUp) {
            Ok(_) => {
                info!("Volume up hotkey pressed. Notifying timer.");
                timer_notifier_up.notify_one();
            }
            Err(e) => {
                error!("Failed to send notification: {}", e);
            }
        }
    });
}

pub fn register_down_hotkey(down_sender: Sender<Message>, timer_notifier_down: Arc<Notify>) {    
    F22Key.bind(move || {
        match down_sender.send(Message::VolumeDown) {
            Ok(_) => {
                info!("Volume down hotkey pressed. Notifying timer.");
                timer_notifier_down.notify_one();
            }
            Err(e) => {
                error!("Failed to send notification: {}", e);
            }
        }
    });
}
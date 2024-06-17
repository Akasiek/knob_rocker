use std::sync::Mutex;
use log::info;
use tokio::sync::broadcast::Receiver;
use crate::{Message, volume};

static VOLUME: Mutex<i32> = Mutex::new(0);

pub fn set_mutex_value(new_value: i32) {
    let mut volume = VOLUME.lock().unwrap();
    
    *volume = match new_value {
        x if x <= -100 => -100,
        x if x >= 100 => 100,
        x => x,
    };
}

pub fn get_mutex_value() -> i32 {
    let volume = VOLUME.lock().unwrap();
    *volume
}

pub fn spawn_up_adjustment(receiver: Receiver<Message>) {
    info!("Starting volume up adjustment loop.");
    spawn(receiver, 5);
}

pub fn spawn_down_adjustment(receiver: Receiver<Message>) {
    info!("Starting volume down adjustment loop.");
    spawn(receiver, -5);
}

fn spawn(mut receiver: Receiver<Message>, adjustment: i32) {
    tokio::spawn(async move {
        loop {
            adjust_volume(&mut receiver, adjustment).await;
        }
    });
} 

async fn adjust_volume(receiver: &mut Receiver<Message>, adjustment: i32) {
    receiver.recv().await.unwrap();

    let new_value = volume::get_mutex_value() + adjustment;
    volume::set_mutex_value(new_value);
}
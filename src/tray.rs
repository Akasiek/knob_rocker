use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use log::info;
use tray_item::{IconSource, TrayItem};

enum Message {
    Quit,
}

pub fn init_tray() {
    let mut tray = TrayItem::new(
        "Knob Rocker",
        IconSource::Resource("tray-default"),
    ).unwrap();

    tray.inner_mut().add_label_with_id("Knob Rocker").unwrap();

    let (tx, rx) = mpsc::sync_channel(1);

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
        .unwrap();
    
    info!("Tray initialized.");

    wait_for_input(rx);
}

fn wait_for_input(receiver: Receiver<Message>) {
    loop {
        if let Ok(Message::Quit) = receiver.recv() {
            info!("Quit");
            break;
        }
    }
}
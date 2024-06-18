use std::io::Cursor;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use log::info;
use tray_item::{IconSource, TrayItem};

enum Message {
    Quit,
}

pub fn init_tray() {
    let icon = if cfg!(windows) {
        get_icon_windows()
    } else {
        get_icon_unix()
    };

    let mut tray = TrayItem::new(
        "Knob Rocker",
        icon,
    ).unwrap();

    #[cfg(windows)]
    tray.inner_mut().add_label_with_id("Knob Rocker").unwrap();
    #[cfg(unix)]
    tray.inner_mut().add_label("Knob Rocker").unwrap();

    let (tx, rx) = mpsc::sync_channel(1);

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
        .unwrap();

    info!("Tray initialized.");

    wait_for_input(rx);
}

fn get_icon_windows() -> IconSource {
    IconSource::Resource("tray-default")
}

fn get_icon_unix() -> IconSource {
    let cursor_red = Cursor::new(include_bytes!("../icons/icon.png"));
    let decoder_red = png::Decoder::new(cursor_red);
    let mut reader = decoder_red.read_info().unwrap();
    let buffer_size = reader.output_buffer_size();
    let mut buf_red = vec![0; buffer_size];
    reader.next_frame(&mut buf_red).unwrap();

    IconSource::Data {
        data: buf_red,
        height: 32,
        width: 32,
    }
}

fn wait_for_input(receiver: Receiver<Message>) {
    loop {
        if let Ok(Message::Quit) = receiver.recv() {
            info!("Quit");
            break;
        }
    }
}
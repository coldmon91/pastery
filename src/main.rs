mod database;
mod key_combination;

use std::sync::mpsc;

use arboard::Clipboard;
use rdev::{listen, Event};

// const COPY_EVENT_CODE: &str = "\u{3}";
// const PASTE_EVENT_CODE: &str = "\u{16}";

fn new_windows_copy_key_combination() -> key_combination::KeyCombination {
    key_combination::KeyCombination::new(rdev::Key::ControlLeft, rdev::Key::KeyC)
}
fn new_windows_paste_key_combination() -> key_combination::KeyCombination {
    key_combination::KeyCombination::new(rdev::Key::ControlLeft, rdev::Key::KeyV)
}

fn key_event_handle(channel: mpsc::Receiver<Event>) {
    let mut copy_key_combination = new_windows_copy_key_combination();
    let mut paste_key_combination = new_windows_paste_key_combination();
    let clipboard_data = database::ClipboardData::new("clip.data".to_string());
    loop {
        match channel.recv() {
            Ok(event) => {
                match event.event_type {
                    rdev::EventType::KeyRelease(key) => {
                        if paste_key_combination.contains(key) {
                            paste_key_combination.release_key(key);
                        }
                        if copy_key_combination.contains(key) {
                            if copy_key_combination.is_active() {
                                let mut clipboard = Clipboard::new().unwrap();
                                println!("Clipboard content: {}", clipboard.get_text().unwrap());
                                clipboard_data.write(clipboard.get_text().unwrap().as_str());
                            }
                            copy_key_combination.release_key(key);
                        }
                    },
                    rdev::EventType::KeyPress(key) => {
                        if copy_key_combination.contains(key) {
                            copy_key_combination.press_key(key);
                        }

                        if paste_key_combination.contains(key) {
                            paste_key_combination.press_key(key);
                        }
                        if paste_key_combination.is_active() {
                            // paste from user's choice
                        }
                    },
                    _ => {}
                }
            },
            Err(_) => break,
        }
    }
}

fn callback(event: Event, channel: mpsc::Sender<Event>) {
    channel.send(event.clone()).unwrap();
}

fn main() {
    println!("Pastery is running");
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        key_event_handle(rx);
    });
    if let Err(error) = listen(move |event| {
        callback(event, tx.clone())
    }) {
        println!("Error: {:?}", error)
    }
    println!("Hello, world!");
}

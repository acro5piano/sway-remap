use evdev::InputEventKind;
use serde::Deserialize;
use serde_yaml;
use std::env;
use std::error::Error;
use std::fs;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use swayipc::{Connection, EventType};
use uinput;
use uinput_sys::EV_KEY;

mod utils;

use utils::config_parser::ConfigKeyCombination;
use utils::input;
use utils::keycodes;
use utils::wayland;

#[derive(Debug, PartialEq, Deserialize, Clone)]
struct Setting {
    applications: Vec<String>,
    remap: Vec<RemapSetting>,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
struct RemapSetting {
    from: ConfigKeyCombination,
    to: Vec<ConfigKeyCombination>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config_str =
        fs::read_to_string(&args[1]).expect("Something went wrong reading the config file");
    let settings: Vec<Setting> =
        serde_yaml::from_str(&config_str).expect("Unable to read config file");

    println!("{:?}", settings);

    let remap_enabled = Arc::new(Mutex::new(false));
    let mut handles = vec![];

    // TODO: get sway socket programatically!
    let conn = Connection::new(Some("/run/user/1000/sway-ipc.1000.25887.sock".to_string()))?;

    /////////////////////////
    // Sway subscribe part
    /////////////////////////
    let remap_enabled_cloned = Arc::clone(&remap_enabled);
    let settings_1 = settings.clone();
    handles.push(thread::spawn(move || {
        let mut stream = conn
            .subscribe(&[EventType::Window])
            .expect("Unable to subscribe sway");
        loop {
            let window_class = wayland::get_window_class(stream.next());
            let mut remap_enabled_lock = remap_enabled_cloned.lock().unwrap();

            if settings_1
                .iter()
                .any(|setting| setting.applications.iter().any(|app| app == &window_class))
            {
                println!("Remap enabled for {}", window_class);
                *remap_enabled_lock = true;
            } else {
                println!("Remap disabled for {}", window_class);
                *remap_enabled_lock = false;
            }
        }
    }));

    /////////////////////////
    // Keyboard part
    /////////////////////////
    let mut device = input::get_keyboard_device().expect("Failed to get keyboard device");

    let mut virtual_input = uinput::default()
        .expect("Please load uinput module; Possibly you should run `modprobe uinput`")
        .name("/dev/uinput")?
        .event(uinput::event::Keyboard::All)?
        .event(uinput::event::Controller::All)?
        .create()?;

    // To wait virtual_input is ready (important)
    thread::sleep(time::Duration::from_secs(1));

    // Intercept real input
    device.grab()?;

    let mut is_caps_pressing = false;

    let remap_enabled_cloned_2 = Arc::clone(&remap_enabled);
    let settings_2 = settings.clone();
    handles.push(thread::spawn(move || loop {
        let remap_enabled_ = *remap_enabled_cloned_2.lock().unwrap();
        let events = device.fetch_events().unwrap();
        events.for_each(|event| {
            match event.kind() {
                InputEventKind::Key(orig_key) => {
                    println!("{:?} -> {:?}", orig_key, event.value());

                    if !remap_enabled_ {
                        virtual_input
                            .write(EV_KEY, orig_key.code() as i32, event.value())
                            .unwrap();
                        return;
                    }

                    // capture ctrl and meta key
                    match (keycodes::code_to_name(orig_key.code()), event.value()) {
                        ("capslock", 1) => is_caps_pressing = true,
                        ("capslock", 0) => is_caps_pressing = false,
                        (_, _) => {}
                    }

                    let mut handled = false;
                    settings_2.iter().for_each(|setting| {
                        setting.remap.iter().for_each(|remap| {
                            if (is_caps_pressing && remap.from.is_ctrl)
                                && remap.from.keyname == keycodes::code_to_name(orig_key.code())
                            {
                                handled = true;
                                remap.to.iter().for_each(|to| {
                                    if is_caps_pressing && !to.is_ctrl {
                                        virtual_input
                                            .write(EV_KEY, keycodes::name_to_code("capslock"), 0)
                                            .unwrap();
                                    }
                                    virtual_input
                                        .write(
                                            EV_KEY,
                                            keycodes::name_to_code(&to.keyname),
                                            event.value(),
                                        )
                                        .unwrap();
                                });
                            }
                        });
                    });

                    if !handled {
                        if is_caps_pressing {
                            virtual_input
                                .write(EV_KEY, keycodes::name_to_code("capslock"), 1)
                                .unwrap();
                        }
                        virtual_input
                            .write(EV_KEY, orig_key.code() as i32, event.value())
                            .unwrap();
                    }
                }
                InputEventKind::Synchronization(_) => {
                    virtual_input.synchronize().unwrap();
                }
                _ => {}
            }
        });
    }));

    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: add cleanup function
    // device.ungrab()?;
    //
    Ok(())
}

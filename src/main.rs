use evdev::{Device, InputEventKind, Key};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use swayipc::{reply, Connection, EventType};
use uinput;
use uinput_sys::EV_KEY;

mod keycodes;

const CAPS: u16 = 58;

const TEST_CONFIG: &str = r#"---
- applications:
    - Brave-browser
    - firefoxdeveloperedition
  remap:
    - from: f
      with: capslock
      to:
        - right
    - from: b
      with: capslock
      to:
        - left
    - from: p
      with: capslock
      to:
        - up
    - from: n
      with: capslock
      to:
        - up
"#;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct Setting {
    applications: Vec<String>,
    remap: Vec<RemapSetting>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct RemapSetting {
    from: String,
    to: Vec<String>,
    with: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let settings: Vec<Setting> = serde_yaml::from_str(&TEST_CONFIG)?;

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
            let window_class = get_window_class(stream.next());
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
    let mut device = get_keyboard_device().expect("Failed to get keyboard device");

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
            println!("{:?}", event);
            match event.kind() {
                InputEventKind::Key(key) => {
                    println!("{:?}", key);
                    // caps
                    match (key.code(), event.value()) {
                        (CAPS, 1) => is_caps_pressing = true,
                        (CAPS, 0) => is_caps_pressing = false,
                        (_, _) => {}
                    }
                    if remap_enabled_
                        && is_caps_pressing
                        && settings_2.iter().any(|setting| {
                            setting
                                .remap
                                .iter()
                                .any(|r| r.from == keycodes::code_to_name(key.code()))
                        })
                    {
                        let setting = match settings_2.iter().find(|setting| {
                            setting
                                .remap
                                .iter()
                                .any(|r| r.from == keycodes::code_to_name(key.code()))
                        }) {
                            Some(s) => s,
                            _ => unreachable!("Logic error"),
                        };
                        let key = match setting
                            .remap
                            .iter()
                            .find(|r| r.from == keycodes::code_to_name(key.code()))
                        {
                            Some(s) => s,
                            _ => unreachable!("Logic error"),
                        };
                        // Clear CapsLock before executing the combination!
                        virtual_input.write(EV_KEY, CAPS as i32, 0).unwrap();
                        key.to.iter().for_each(|to| {
                            virtual_input
                                .write(EV_KEY, keycodes::name_to_code(to), event.value())
                                .unwrap()
                        });
                        virtual_input.write(EV_KEY, CAPS as i32, 1).unwrap();
                    } else {
                        virtual_input
                            .write(EV_KEY, key.code() as i32, event.value())
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

fn get_keyboard_device() -> Result<Device, Box<dyn Error>> {
    for i in 0..25 {
        let device = Device::open(format!("/dev/input/event{}", i))?;
        if device
            .supported_keys()
            .map_or(false, |keys| keys.contains(Key::KEY_ENTER))
        {
            return Ok(device);
        }
    }
    panic!("Cannot infer default device");
}

fn get_window_class(evt: Option<Result<reply::Event, swayipc::Error>>) -> String {
    match evt {
        Some(Ok(reply::Event::Window(w))) => {
            // app_id => native wayland
            // xwayland => window_properties.class
            match (w.container.app_id, w.container.window_properties) {
                (Some(id), _) => id,
                (_, Some(props)) => match props.class {
                    Some(class) => class,
                    _ => panic!("Cannot get window id"),
                },
                (_, _) => panic!("Cannot get window id"),
            }
        }
        _ => panic!("Cannot get window id"),
    }
}

use evdev::{Device, InputEventKind, Key};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use swayipc::{reply, Connection, EventType};
use uinput;
use uinput_sys::EV_KEY;

const CAPS: u16 = 58;

fn main() -> Result<(), Box<dyn Error>> {
    let remap_enabled = Arc::new(Mutex::new(false));
    let mut handles = vec![];

    // TODO: get sway process from root permission (or vice versa)
    let conn = Connection::new(Some("/run/user/1000/sway-ipc.1000.1663.sock".to_string()))?;

    /////////////////////////
    // Sway subscribe part
    /////////////////////////
    let remap_enabled_cloned = Arc::clone(&remap_enabled);
    handles.push(thread::spawn(move || {
        let mut stream = conn
            .subscribe(&[EventType::Window])
            .expect("Unable to subscribe sway");
        loop {
            let window_class = get_window_class(stream.next());
            let mut remap_enabled_lock = remap_enabled_cloned.lock().unwrap();

            if window_class == "Brave-browser" {
                println!("brave found!");
                *remap_enabled_lock = true;
            } else {
                *remap_enabled_lock = false;
            }
        }
    }));

    /////////////////////////
    // Keyboard part
    /////////////////////////
    let mut device = get_keyboard_device()?;

    let mut virtual_input = uinput::default()?
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
    handles.push(thread::spawn(move || loop {
        let remap_enabled_lock = *remap_enabled_cloned_2.lock().unwrap();
        if remap_enabled_lock == false {
            continue;
        }
        let events = device.fetch_events().unwrap();
        events.for_each(|event| {
            println!("{:?}", event);
            match event.kind() {
                InputEventKind::Key(key) => {
                    // caps
                    match (key.code(), event.value()) {
                        (CAPS, 1) => is_caps_pressing = true,
                        (CAPS, 0) => is_caps_pressing = false,
                        (_, _) => {}
                    }
                    virtual_input
                        .write(EV_KEY, key.code() as i32, event.value())
                        .unwrap();
                    if is_caps_pressing && key.code() == 38 {
                        println!("++++++++++++++++++++++++++++LLLLLLLLLLLLLLLLLl+++++++++++++++++");
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

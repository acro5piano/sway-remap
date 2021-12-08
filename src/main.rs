use evdev::{Device, InputEventKind, Key};
use std::error::Error;
use std::{thread, time};
use uinput;
use uinput_sys::EV_KEY;

fn get_fallback_device() -> Result<Device, Box<dyn Error>> {
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

const CAPS: u16 = 58;

fn main() -> Result<(), Box<dyn Error>> {
    let mut device = get_fallback_device()?;

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

    loop {
        let events = device.fetch_events()?;
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
    }

    // TODO: add cleanup function
    //
    // device.ungrab()?;
    //
    // Ok(())
}

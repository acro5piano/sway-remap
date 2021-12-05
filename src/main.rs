use evdev::uinput::VirtualDeviceBuilder;
use evdev::{AttributeSet, Device, EventType, InputEvent, Key};
use std::io::Error;
use std::{thread, time};

fn get_fallback_device() -> Result<Device, Error> {
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

fn main() -> Result<(), Error> {
    let mut device = get_fallback_device()?;
    let mut keys = AttributeSet::<Key>::new();

    let mut virtual_device = VirtualDeviceBuilder::new()?
        .name("Fake Keyboard")
        .with_keys(&keys)?
        .build()
        .unwrap();

    // println!("{:?}", device.supported_keys());

    device.grab()?;

    // loop {
    for _ in 0..20 {
        let events = device.fetch_events()?;
        events.for_each(|event| {
            println!("{:?}", event);
            virtual_device.emit(&[event]).unwrap();
        });
    }

    device.ungrab()?;

    // println!("{:?}", events.);

    Ok(())
}

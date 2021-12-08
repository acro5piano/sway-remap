use evdev::uinput::VirtualDeviceBuilder;
use evdev::{AttributeSet, Device, EventType, InputEvent, Key};
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut device = get_fallback_device()?;
    let mut keys = AttributeSet::<Key>::new();

    let mut virtual_input = uinput::default()?
        .name("/dev/input/event26")?
        .event(uinput::event::Keyboard::All)?
        .event(uinput::event::Controller::All)?
        .create()?;

    // for item in uinput::event::relative::Position::iter_variants() {
    //     builder = builder.event(item)?;
    // }
    // for item in uinput::event::relative::Wheel::iter_variants() {
    //     builder = builder.event(item)?;
    // }

    virtual_input.write(EV_KEY, 19, 1)?;
    virtual_input.synchronize()?;

    for _ in 0..20 {
        virtual_input.write(EV_KEY, 19, 1)?;
        virtual_input.synchronize()?;
    }

    // let mut virtual_device = VirtualDeviceBuilder::new()?
    //     .name("Fake Keyboard")
    //     .with_keys(&keys)?
    //     .build()
    //     .unwrap();

    // println!("{:?}", device.supported_keys());

    // device.grab()?;
    //
    // // loop {
    // for _ in 0..20 {
    //     let events = device.fetch_events()?;
    //     events.for_each(|event| {
    //         println!("{:?}", event);
    //         virtual_device.emit(&[event]).unwrap();
    //     });
    // }
    //
    // device.ungrab()?;

    // println!("{:?}", events.);

    Ok(())
}

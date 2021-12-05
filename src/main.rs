use evdev::{Device, Key};
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

    // println!("{:?}", device.supported_keys());

    device.grab()?;

    println!("grab");

    thread::sleep(time::Duration::from_millis(5000));

    // let res = device.read();

    // loop {
    // println!("{}", "aaaaa");
    // }

    device.ungrab()?;

    println!("ungrab");

    Ok(())
}

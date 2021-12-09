import time

import evdev
import uinput

from all_keys import ALL_KEYS
from config import example_config, Binding

CTRL_KEYS = [evdev.ecodes.KEY_CAPSLOCK]  # type: ignore

real_input = evdev.InputDevice('/dev/input/event4')
print(real_input)

EV_KEY = evdev.ecodes.EV_KEY  # type: ignore


def is_pressed(value: int) -> bool:
    return value == 1 or value == 2


def run(config: list[Binding]):
    is_ctrl = False

    with uinput.Device(ALL_KEYS) as virtual_uinput:
        time.sleep(1)  # Important delay
        with real_input.grab_context():
            for event in real_input.read_loop():
                if event.type == EV_KEY:
                    if event.code in CTRL_KEYS:
                        is_ctrl = is_pressed(event.value)
                    handled = False
                    for binding in config:
                        if (is_ctrl
                                and event.code == binding.get_remap_keycode()
                                and is_pressed(event.value)):
                            handled = True
                            virtual_uinput.emit(uinput.KEY_CAPSLOCK, 0)
                            virtual_uinput.emit_combo(binding.to)
                            virtual_uinput.emit(uinput.KEY_CAPSLOCK, 1)
                    if not handled:
                        virtual_uinput.emit((0x01, event.code), event.value)


# TODO: list-device helper
# devices = [evdev.InputDevice(path) for path in evdev.list_devices()]
# for device in devices:
#    print(device.path, device.name, device.phys)

if __name__ == "__main__":
    run(example_config)

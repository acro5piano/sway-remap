import uinput
import time
import evdev
from all_keys import ALL_KEYS

CTRL_KEYS = [evdev.ecodes.KEY_CAPSLOCK]  # type: ignore

real_input = evdev.InputDevice('/dev/input/event4')
print(real_input)

EV_KEY = evdev.ecodes.EV_KEY  # type: ignore

is_ctrl = False


def is_pressed(value: int) -> bool:
    return value == 1 or value == 2


with uinput.Device(ALL_KEYS) as virtual_uinput:
    time.sleep(1)  # Important delay
    with real_input.grab_context():
        for event in real_input.read_loop():
            if event.type == EV_KEY:
                if event.code in CTRL_KEYS:
                    is_ctrl = is_pressed(event.value)
                handled = False
                if is_ctrl and event.code == evdev.ecodes.KEY_F and is_pressed(
                        event.value):  # type: ignore
                    handled = True
                    virtual_uinput.emit(uinput.KEY_CAPSLOCK, 0)
                    virtual_uinput.emit_combo([uinput.KEY_RIGHT])
                    virtual_uinput.emit(uinput.KEY_CAPSLOCK, 1)
                if is_ctrl and event.code == evdev.ecodes.KEY_B and is_pressed(
                        event.value):  # type: ignore
                    virtual_uinput.emit(uinput.KEY_CAPSLOCK, 0)
                    virtual_uinput.emit_combo([uinput.KEY_LEFT])
                    virtual_uinput.emit(uinput.KEY_CAPSLOCK, 1)
                    handled = True
                if not handled:
                    virtual_uinput.emit((0x01, event.code), event.value)

# TODO: list-device helper
# devices = [evdev.InputDevice(path) for path in evdev.list_devices()]
# for device in devices:
#    print(device.path, device.name, device.phys)

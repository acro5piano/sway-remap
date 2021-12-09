import uinput
import time
import evdev

# TODO: list-device helper
# devices = [evdev.InputDevice(path) for path in evdev.list_devices()]
# for device in devices:
#    print(device.path, device.name, device.phys)

KEYS = [
    uinput.KEY_LEFTALT, uinput.KEY_TAB, uinput.KEY_E, uinput.KEY_O,
    uinput.KEY_LEFT, uinput.KEY_RIGHT, uinput.KEY_CAPSLOCK
]

CTRL_KEYS = [evdev.ecodes.KEY_CAPSLOCK]  # type: ignore

real_input = evdev.InputDevice('/dev/input/event4')
print(real_input)

is_ctrl = False

with uinput.Device(KEYS) as virtual_uinput:
    time.sleep(1)  # Important delay
    with real_input.grab_context():
        for event in real_input.read_loop():
            if event.type == evdev.ecodes.EV_KEY:  # type: ignore
                if event.code in CTRL_KEYS:
                    is_ctrl = event.value == 1 or event.value == 2
                if is_ctrl and event.code == evdev.ecodes.KEY_F and event.value == 1:  # type: ignore
                    virtual_uinput.emit_combo([uinput.KEY_RIGHT])
                if is_ctrl and event.code == evdev.ecodes.KEY_B and event.value == 1:  # type: ignore
                    virtual_uinput.emit_combo([uinput.KEY_LEFT])

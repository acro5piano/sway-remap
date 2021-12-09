import uinput
import time

with uinput.Device([uinput.KEY_LEFTALT, uinput.KEY_TAB, uinput.KEY_E, uinput.KEY_O]) as device:
    time.sleep(1) # Important delay
    device.emit_combo([uinput.KEY_LEFTALT, uinput.KEY_E])
    device.emit_click(uinput.KEY_O)


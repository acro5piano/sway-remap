from dataclasses import dataclass
import evdev
import uinput


@dataclass
class Binding:
    remap: str
    to: list[tuple[int, int]]

    def get_remap_keycode(self) -> str:
        """
        @type evdev.ecodes.KEY_F
        """
        _k = self.remap.split('.')
        key_name = _k[len(_k) - 1]
        if key_name == 'f':
            return evdev.ecodes.KEY_F  # type:ignore
        if key_name == 'b':
            return evdev.ecodes.KEY_B  # type:ignore
        raise Exception("Not Found")

    def only_ctrl(self) -> bool:
        return 'ctrl' in self.remap

    def only_alt(self) -> bool:
        return 'alt' in self.remap


example_config = [
    Binding('ctrl.f', [uinput.KEY_RIGHT]),
    Binding('ctrl.b', [uinput.KEY_LEFT]),
    Binding('alt.f', [uinput.KEY_LEFTCTRL, uinput.KEY_RIGHT]),
    Binding('alt.b', [uinput.KEY_LEFTCTRL, uinput.KEY_LEFT]),
]

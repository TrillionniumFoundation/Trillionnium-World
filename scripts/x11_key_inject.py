#!/usr/bin/env python3
"""Inject explicit X11 key chords into one test window using libXtst."""

import ctypes
import os
import sys
import time


def main() -> int:
    if len(sys.argv) < 3:
        raise SystemExit("usage: x11_key_inject.py WINDOW_ID KEY [KEY ...]")
    window = int(sys.argv[1], 0)
    x11 = ctypes.CDLL("libX11.so.6")
    xtst = ctypes.CDLL("libXtst.so.6")
    x11.XOpenDisplay.restype = ctypes.c_void_p
    x11.XSetInputFocus.argtypes = [ctypes.c_void_p, ctypes.c_ulong, ctypes.c_int, ctypes.c_ulong]
    x11.XFlush.argtypes = [ctypes.c_void_p]
    x11.XStringToKeysym.restype = ctypes.c_ulong
    x11.XStringToKeysym.argtypes = [ctypes.c_char_p]
    x11.XKeysymToKeycode.argtypes = [ctypes.c_void_p, ctypes.c_ulong]
    x11.XKeysymToKeycode.restype = ctypes.c_uint
    x11.XCloseDisplay.argtypes = [ctypes.c_void_p]
    xtst.XTestFakeKeyEvent.argtypes = [
        ctypes.c_void_p,
        ctypes.c_uint,
        ctypes.c_bool,
        ctypes.c_ulong,
    ]
    display = x11.XOpenDisplay(None)
    if not display:
        raise SystemExit("cannot open X11 display")
    x11.XSetInputFocus(display, ctypes.c_ulong(window), 1, 0)
    x11.XFlush(display)

    fast_delay = os.environ.get("TRNM_X11_KEY_DELAY")

    def pause(default: float) -> None:
        time.sleep(float(fast_delay) if fast_delay is not None else default)

    def keycode(name: str) -> int:
        symbol = x11.XStringToKeysym(name.encode())
        code = x11.XKeysymToKeycode(display, symbol)
        if not code:
            raise SystemExit(f"unknown X11 key: {name}")
        return code

    modifiers = {"ctrl": "Control_L", "shift": "Shift_L", "alt": "Alt_L"}
    special = {
        "enter": "Return",
        "escape": "Escape",
        "tab": "Tab",
        "backspace": "BackSpace",
        **{f"f{i}": f"F{i}" for i in range(1, 13)},
    }
    for chord in sys.argv[2:]:
        parts = chord.lower().split("+")
        modifier_codes = [keycode(modifiers[part]) for part in parts[:-1]]
        main_code = keycode(special.get(parts[-1], parts[-1]))
        for code in modifier_codes:
            xtst.XTestFakeKeyEvent(display, code, True, 0)
        x11.XFlush(display)
        pause(0.25)
        xtst.XTestFakeKeyEvent(display, main_code, True, 0)
        x11.XFlush(display)
        pause(0.25)
        xtst.XTestFakeKeyEvent(display, main_code, False, 0)
        x11.XFlush(display)
        pause(0.75 if modifier_codes else 0.15)
        for code in reversed(modifier_codes):
            xtst.XTestFakeKeyEvent(display, code, False, 0)
        x11.XFlush(display)
        pause(0.5)
    x11.XCloseDisplay(display)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

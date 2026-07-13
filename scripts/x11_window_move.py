#!/usr/bin/env python3
"""Move one X11 window during native multi-window acceptance runs."""

import ctypes
import sys


def main() -> int:
    if len(sys.argv) != 4:
        raise SystemExit("usage: x11_window_move.py WINDOW_ID X Y")
    window = int(sys.argv[1], 0)
    x = int(sys.argv[2])
    y = int(sys.argv[3])
    x11 = ctypes.CDLL("libX11.so.6")
    x11.XOpenDisplay.restype = ctypes.c_void_p
    x11.XMoveWindow.argtypes = [ctypes.c_void_p, ctypes.c_ulong, ctypes.c_int, ctypes.c_int]
    x11.XFlush.argtypes = [ctypes.c_void_p]
    x11.XSync.argtypes = [ctypes.c_void_p, ctypes.c_bool]
    x11.XCloseDisplay.argtypes = [ctypes.c_void_p]
    display = x11.XOpenDisplay(None)
    if not display:
        raise SystemExit("cannot open X11 display")
    x11.XMoveWindow(display, ctypes.c_ulong(window), x, y)
    x11.XFlush(display)
    x11.XSync(display, False)
    x11.XCloseDisplay(display)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

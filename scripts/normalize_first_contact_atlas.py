#!/usr/bin/env python3
"""Normalize an authored First Contact sheet into a fixed Bevy atlas grid."""

from __future__ import annotations

import argparse
from pathlib import Path

from PIL import Image


def parse_rows(value: str) -> set[int]:
    if not value.strip():
        return set()
    return {int(item) for item in value.split(",")}


def alpha_bbox(image: Image.Image) -> tuple[int, int, int, int] | None:
    alpha = image.getchannel("A").point(lambda value: 255 if value > 16 else 0)
    return alpha.getbbox()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("input", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--columns", type=int, default=8)
    parser.add_argument("--rows", type=int, default=6)
    parser.add_argument("--cell-size", type=int, default=128)
    parser.add_argument("--top-center", type=float, default=150.0)
    parser.add_argument("--row-pitch", type=float, default=200.0)
    parser.add_argument("--source-cell-width", type=float, default=156.0)
    parser.add_argument("--source-cell-height", type=float, default=188.0)
    parser.add_argument("--full-bleed-rows", default="")
    args = parser.parse_args()

    source = Image.open(args.input).convert("RGBA")
    target = Image.new(
        "RGBA",
        (args.columns * args.cell_size, args.rows * args.cell_size),
        (0, 0, 0, 0),
    )
    full_bleed_rows = parse_rows(args.full_bleed_rows)

    for row in range(args.rows):
        center_y = args.top_center + row * args.row_pitch
        for column in range(args.columns):
            center_x = (column + 0.5) * source.width / args.columns
            left = round(center_x - args.source_cell_width / 2)
            top = round(center_y - args.source_cell_height / 2)
            right = round(center_x + args.source_cell_width / 2)
            bottom = round(center_y + args.source_cell_height / 2)
            crop = source.crop((left, top, right, bottom))
            bbox = alpha_bbox(crop)
            if bbox is None:
                continue
            sprite = crop.crop(bbox)
            if row in full_bleed_rows:
                sprite = sprite.resize(
                    (args.cell_size, args.cell_size), Image.Resampling.NEAREST
                )
                target.alpha_composite(
                    sprite, (column * args.cell_size, row * args.cell_size)
                )
                continue

            maximum = args.cell_size - 12
            scale = min(maximum / sprite.width, maximum / sprite.height, 1.0)
            size = (
                max(1, round(sprite.width * scale)),
                max(1, round(sprite.height * scale)),
            )
            sprite = sprite.resize(size, Image.Resampling.NEAREST)
            x = column * args.cell_size + (args.cell_size - sprite.width) // 2
            y = row * args.cell_size + (args.cell_size - sprite.height) // 2
            target.alpha_composite(sprite, (x, y))

    args.output.parent.mkdir(parents=True, exist_ok=True)
    target.save(args.output, optimize=True)


if __name__ == "__main__":
    main()

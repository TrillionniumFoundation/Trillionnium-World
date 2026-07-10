#!/usr/bin/env python3
"""Human-oriented visual smoke check for the First Contact live screen."""

from __future__ import annotations

import argparse
from pathlib import Path

from PIL import Image, ImageStat


def luminance(rgb: tuple[int, int, int]) -> float:
    channels = []
    for value in rgb:
        channel = value / 255.0
        channels.append(
            channel / 12.92
            if channel <= 0.04045
            else ((channel + 0.055) / 1.055) ** 2.4
        )
    return 0.2126 * channels[0] + 0.7152 * channels[1] + 0.0722 * channels[2]


def percentile(values: list[float], fraction: float) -> float:
    ordered = sorted(values)
    return ordered[min(len(ordered) - 1, round((len(ordered) - 1) * fraction))]


def region_metrics(image: Image.Image, box: tuple[int, int, int, int]) -> tuple[float, float, float]:
    region = image.crop(box).convert("RGB")
    sample = region.resize(
        (max(1, region.width // 4), max(1, region.height // 4)),
        Image.Resampling.BILINEAR,
    )
    hard_sample = region.resize(sample.size, Image.Resampling.NEAREST)
    values = [luminance(pixel) for pixel in sample.getdata()]
    low = percentile(values, 0.05)
    # UI text/markers intentionally occupy a small fraction of each region;
    # use the bright tail rather than letting panel background dominate p95.
    high = percentile(values, 0.995)
    contrast = (high + 0.05) / (low + 0.05)
    variation = sum(ImageStat.Stat(sample).stddev) / 3.0
    hard_values = [luminance(pixel) for pixel in hard_sample.getdata()]
    dark_fraction = sum(value < 0.018 for value in hard_values) / len(hard_values)
    return contrast, variation, dark_fraction


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("screenshot", type=Path)
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()
    source = Image.open(args.screenshot).convert("RGBA")
    alpha_sample = source.getchannel("A").resize(
        (max(1, source.width // 4), max(1, source.height // 4)),
        Image.Resampling.NEAREST,
    )
    transparent_fraction = sum(value < 250 for value in alpha_sample.getdata()) / (
        alpha_sample.width * alpha_sample.height
    )
    image = Image.alpha_composite(
        Image.new("RGBA", source.size, (0, 0, 0, 255)), source
    ).convert("RGB")
    width, height = image.size
    if width < 1280 or height < 690:
        raise SystemExit(f"FAIL viewport {width}x{height}; expected at least 1280x690")

    regions = {
        "top_hud": (0, 0, width, 48),
        "battlefield": (100, 48, width - 100, height - 154),
        "radar": (width - 208, 60, width - 18, 192),
        "bottom_hud": (0, height - 154, width, height),
        "selection": (0, height - 154, 286, height),
        "commands": (286, height - 154, 820, height),
        "objective": (820, height - 154, width, height),
    }
    metrics = {name: region_metrics(image, box) for name, box in regions.items()}
    if args.verbose:
        print(f"CAPTURE transparent_fraction={transparent_fraction:.3f}")
        for name, (contrast, variation, dark_fraction) in metrics.items():
            print(
                f"REGION {name}: contrast={contrast:.2f} "
                f"variation={variation:.2f} dark_fraction={dark_fraction:.2f}"
            )
    failures: list[str] = []
    if transparent_fraction > 0.01:
        failures.append("capture contains transparent/partially presented frame regions")
    battlefield_contrast, battlefield_variation, battlefield_dark = metrics["battlefield"]
    if battlefield_variation < 18.0 or battlefield_dark > 0.32:
        failures.append("battlefield is blank/dead or materially occluded")
    if battlefield_contrast < 2.2:
        failures.append("battlefield terrain/actors do not separate in luminance")
    for name in ["top_hud", "radar", "selection", "commands", "objective"]:
        contrast, variation, _dark = metrics[name]
        if contrast < 2.6 or variation < 8.0:
            failures.append(f"{name} lacks readable foreground/background separation")

    if failures:
        for name, (contrast, variation, dark_fraction) in metrics.items():
            print(
                f"REGION {name}: contrast={contrast:.2f} "
                f"variation={variation:.2f} dark_fraction={dark_fraction:.2f}"
            )
        for failure in failures:
            print(f"FAIL {failure}")
        raise SystemExit(1)

    print(
        "PASS First Contact live screen: authored battlefield visible; "
        "top HUD, radar, selection, commands, and objective regions have usable contrast; "
        "no large dead/occluded playfield"
    )


if __name__ == "__main__":
    main()

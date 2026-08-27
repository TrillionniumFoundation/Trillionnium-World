use bevy::prelude::*;

pub(super) const HEADER_HEIGHT: f32 = 64.0;
pub(super) const EDGE_GAP: f32 = 16.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum UiTone {
    Neutral,
    Positive,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct UiPalette {
    pub canvas: Color,
    pub surface: Color,
    pub surface_raised: Color,
    pub border: Color,
    pub text: Color,
    pub muted: Color,
    pub accent: Color,
    pub accent_text: Color,
    pub positive: Color,
    pub warning: Color,
    pub critical: Color,
}

impl UiPalette {
    pub(super) fn tone(self, tone: UiTone) -> Color {
        match tone {
            UiTone::Neutral => self.accent,
            UiTone::Positive => self.positive,
            UiTone::Warning => self.warning,
            UiTone::Critical => self.critical,
        }
    }

    pub(super) fn chip(self, tone: UiTone) -> (Color, Color, Color) {
        let border = self.tone(tone);
        let background = match tone {
            UiTone::Neutral => Color::srgba(0.075, 0.13, 0.11, 0.98),
            UiTone::Positive => Color::srgba(0.08, 0.20, 0.12, 0.98),
            UiTone::Warning => Color::srgba(0.24, 0.17, 0.055, 0.98),
            UiTone::Critical => Color::srgba(0.24, 0.07, 0.065, 0.98),
        };
        (background, border, self.text)
    }

    pub(super) fn button(
        self,
        interaction: Interaction,
        selected: bool,
    ) -> (Color, Color, Color) {
        match interaction {
            Interaction::Pressed => (
                Color::srgba(0.34, 0.55, 0.22, 1.0),
                self.warning,
                self.accent_text,
            ),
            Interaction::Hovered => (
                Color::srgba(0.18, 0.38, 0.30, 1.0),
                self.positive,
                Color::WHITE,
            ),
            Interaction::None if selected => (
                Color::srgba(0.14, 0.30, 0.24, 0.98),
                self.positive,
                self.text,
            ),
            Interaction::None => (self.surface_raised, self.border, self.text),
        }
    }
}

pub(super) fn world_ui_palette(high_contrast: bool) -> UiPalette {
    if high_contrast {
        UiPalette {
            canvas: Color::BLACK,
            surface: Color::srgba(0.0, 0.0, 0.0, 0.98),
            surface_raised: Color::srgba(0.045, 0.045, 0.045, 0.99),
            border: Color::WHITE,
            text: Color::WHITE,
            muted: Color::srgb(0.86, 0.86, 0.86),
            accent: Color::srgb(0.20, 0.92, 0.82),
            accent_text: Color::BLACK,
            positive: Color::srgb(0.48, 1.0, 0.52),
            warning: Color::srgb(1.0, 0.86, 0.22),
            critical: Color::srgb(1.0, 0.36, 0.30),
        }
    } else {
        UiPalette {
            canvas: Color::srgb(0.012, 0.025, 0.024),
            surface: Color::srgba(0.018, 0.034, 0.031, 0.98),
            surface_raised: Color::srgba(0.055, 0.085, 0.082, 0.98),
            border: Color::srgb(0.22, 0.43, 0.38),
            text: Color::srgb(0.91, 0.94, 0.82),
            muted: Color::srgb(0.62, 0.76, 0.72),
            accent: Color::srgb(0.62, 0.88, 0.70),
            accent_text: Color::srgb(0.012, 0.025, 0.024),
            positive: Color::srgb(0.58, 0.96, 0.48),
            warning: Color::srgb(1.0, 0.80, 0.30),
            critical: Color::srgb(0.98, 0.40, 0.34),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_contrast_palette_preserves_maximum_text_contrast() {
        let palette = world_ui_palette(true);
        assert_eq!(palette.canvas, Color::BLACK);
        assert_eq!(palette.text, Color::WHITE);
        assert_eq!(palette.border, Color::WHITE);
    }

    #[test]
    fn semantic_tones_remain_visually_distinct() {
        let palette = world_ui_palette(false);
        assert_ne!(palette.tone(UiTone::Neutral), palette.tone(UiTone::Positive));
        assert_ne!(palette.tone(UiTone::Positive), palette.tone(UiTone::Warning));
        assert_ne!(palette.tone(UiTone::Warning), palette.tone(UiTone::Critical));
    }
}

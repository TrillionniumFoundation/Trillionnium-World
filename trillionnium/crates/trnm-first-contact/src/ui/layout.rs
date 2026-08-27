use super::theme::{EDGE_GAP, HEADER_HEIGHT};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum UiViewportClass {
    Compact,
    Standard,
    Wide,
}

impl UiViewportClass {
    pub(super) fn from_width(width: f32) -> Self {
        if width < 960.0 {
            Self::Compact
        } else if width < 1440.0 {
            Self::Standard
        } else {
            Self::Wide
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct UiLayoutMetrics {
    pub drawer_width: Option<f32>,
    pub drawer_height: Option<f32>,
    pub campaign_top_inset: f32,
    pub campaign_right_inset: f32,
    pub campaign_bottom_inset: f32,
    pub body_font_size: f32,
}

impl UiLayoutMetrics {
    pub(super) fn for_viewport(viewport: UiViewportClass, drawer_open: bool) -> Self {
        let campaign_top_inset = HEADER_HEIGHT + EDGE_GAP;
        match viewport {
            UiViewportClass::Compact => Self {
                drawer_width: None,
                drawer_height: Some(286.0),
                campaign_top_inset,
                campaign_right_inset: 24.0,
                campaign_bottom_inset: if drawer_open { 318.0 } else { 24.0 },
                body_font_size: 13.0,
            },
            UiViewportClass::Standard => Self {
                drawer_width: Some(340.0),
                drawer_height: None,
                campaign_top_inset,
                campaign_right_inset: if drawer_open { 372.0 } else { 24.0 },
                campaign_bottom_inset: 24.0,
                body_font_size: 14.0,
            },
            UiViewportClass::Wide => Self {
                drawer_width: Some(400.0),
                drawer_height: None,
                campaign_top_inset,
                campaign_right_inset: if drawer_open { 432.0 } else { 24.0 },
                campaign_bottom_inset: 24.0,
                body_font_size: 15.0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_breakpoints_cover_compact_standard_and_wide() {
        assert_eq!(UiViewportClass::from_width(800.0), UiViewportClass::Compact);
        assert_eq!(UiViewportClass::from_width(1280.0), UiViewportClass::Standard);
        assert_eq!(UiViewportClass::from_width(1600.0), UiViewportClass::Wide);
    }

    #[test]
    fn open_drawer_reserves_space_without_changing_battle_canvas_size() {
        let closed = UiLayoutMetrics::for_viewport(UiViewportClass::Standard, false);
        let open = UiLayoutMetrics::for_viewport(UiViewportClass::Standard, true);
        assert!(open.campaign_right_inset > closed.campaign_right_inset);
        assert_eq!(open.campaign_bottom_inset, closed.campaign_bottom_inset);
    }

    #[test]
    fn compact_drawer_becomes_a_bottom_sheet() {
        let compact = UiLayoutMetrics::for_viewport(UiViewportClass::Compact, true);
        assert_eq!(compact.drawer_width, None);
        assert_eq!(compact.drawer_height, Some(286.0));
        assert!(compact.campaign_bottom_inset > 300.0);
    }
}

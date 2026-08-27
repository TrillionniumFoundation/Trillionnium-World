mod layout;
mod model;
mod theme;

use super::{
    campaign_flow::{CampaignFlow, CampaignMode, ShellMode},
    campaign_ui::CampaignOverlayRoot,
    map_loader::FirstContactMap,
    online_authority::OnlineAuthorityClient,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use layout::{UiLayoutMetrics, UiViewportClass};
use model::{WorldUiPage, WorldUiSnapshot};
use theme::{world_ui_palette, EDGE_GAP, HEADER_HEIGHT};

#[derive(Resource, Debug, Clone, Copy)]
pub(super) struct WorldUiState {
    drawer_open: bool,
    page: WorldUiPage,
    initialized: bool,
}

impl Default for WorldUiState {
    fn default() -> Self {
        Self {
            drawer_open: true,
            page: WorldUiPage::Now,
            initialized: false,
        }
    }
}

#[derive(Component)]
pub(super) struct WorldUiRoot;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum WorldUiSurface {
    Header,
    Drawer,
    BattleBadge,
    AuthorityChip,
    EconomyChip,
    Interactive,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum WorldUiTextSlot {
    Product,
    Context,
    Observer,
    Authority,
    Economy,
    DrawerToggle,
    DrawerTitle,
    DrawerBody,
    DrawerFooter,
    BattleBadge,
    PageTab(WorldUiPage),
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
struct WorldUiButton {
    action: WorldUiButtonAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorldUiButtonAction {
    ToggleDrawer,
    SelectPage(WorldUiPage),
}

fn ui_text(text: impl Into<String>, slot: WorldUiTextSlot, font_size: f32) -> impl Bundle {
    (
        Text::new(text.into()),
        slot,
        TextFont::from_font_size(font_size),
        TextColor(Color::WHITE),
    )
}

fn ui_button(
    action: WorldUiButtonAction,
    label: impl Into<String>,
    slot: WorldUiTextSlot,
    min_width: f32,
) -> impl Bundle {
    (
        Button,
        Node {
            min_width: px(min_width),
            height: px(36),
            padding: UiRect::axes(px(12), px(6)),
            border: UiRect::all(px(1)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.055, 0.085, 0.082, 0.98)),
        BorderColor::all(Color::srgb(0.22, 0.43, 0.38)),
        WorldUiSurface::Interactive,
        WorldUiButton { action },
        children![(ui_text(label, slot, 12.0))],
    )
}

fn status_chip(
    surface: WorldUiSurface,
    slot: WorldUiTextSlot,
    initial: &str,
    min_width: f32,
) -> impl Bundle {
    (
        Node {
            min_width: px(min_width),
            max_width: px(240),
            height: px(44),
            padding: UiRect::axes(px(10), px(5)),
            border: UiRect::all(px(1)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.075, 0.13, 0.11, 0.98)),
        BorderColor::all(Color::srgb(0.31, 0.62, 0.47)),
        surface,
        children![(ui_text(initial, slot, 10.0))],
    )
}

pub(super) fn spawn_world_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: px(0),
            top: px(0),
            width: percent(100),
            height: percent(100),
            ..default()
        },
        GlobalZIndex(160),
        FocusPolicy::Pass,
        WorldUiRoot,
        children![
            (
                Node {
                    position_type: PositionType::Absolute,
                    left: px(0),
                    top: px(0),
                    width: percent(100),
                    height: px(HEADER_HEIGHT),
                    padding: UiRect::axes(px(18), px(8)),
                    border: UiRect::bottom(px(2)),
                    align_items: AlignItems::Center,
                    column_gap: px(14),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.012, 0.025, 0.024, 0.98)),
                BorderColor::all(Color::srgb(0.22, 0.43, 0.38)),
                WorldUiSurface::Header,
                children![
                    (
                        Node {
                            width: px(208),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(1),
                            ..default()
                        },
                        children![
                            (ui_text(
                                "TRILLIONNIUM / WORLD",
                                WorldUiTextSlot::Product,
                                18.0,
                            )),
                            (ui_text(
                                "PLAYER CONTROL SURFACE v1",
                                WorldUiTextSlot::Context,
                                9.0,
                            )),
                        ],
                    ),
                    (
                        Node {
                            flex_grow: 1.0,
                            flex_direction: FlexDirection::Column,
                            row_gap: px(2),
                            ..default()
                        },
                        children![(ui_text(
                            "NEXT · loading campaign state",
                            WorldUiTextSlot::Observer,
                            12.0,
                        ))],
                    ),
                    status_chip(
                        WorldUiSurface::AuthorityChip,
                        WorldUiTextSlot::Authority,
                        "OFFLINE WORLD",
                        146.0,
                    ),
                    status_chip(
                        WorldUiSurface::EconomyChip,
                        WorldUiTextSlot::Economy,
                        "LOCAL ECONOMY",
                        142.0,
                    ),
                    ui_button(
                        WorldUiButtonAction::ToggleDrawer,
                        "F6 GUIDE",
                        WorldUiTextSlot::DrawerToggle,
                        92.0,
                    ),
                ],
            ),
            (
                Node {
                    position_type: PositionType::Absolute,
                    right: px(EDGE_GAP),
                    top: px(HEADER_HEIGHT + EDGE_GAP / 2.0),
                    bottom: px(EDGE_GAP),
                    width: px(340),
                    padding: UiRect::all(px(16)),
                    border: UiRect::all(px(2)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(12),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.018, 0.034, 0.031, 0.98)),
                BorderColor::all(Color::srgb(0.22, 0.43, 0.38)),
                WorldUiSurface::Drawer,
                children![
                    (ui_text(
                        "NOW / PLAYER CONTROL CENTER",
                        WorldUiTextSlot::DrawerTitle,
                        15.0,
                    )),
                    (
                        Node {
                            width: percent(100),
                            flex_direction: FlexDirection::Row,
                            column_gap: px(6),
                            ..default()
                        },
                        children![
                            ui_button(
                                WorldUiButtonAction::SelectPage(WorldUiPage::Now),
                                "NOW",
                                WorldUiTextSlot::PageTab(WorldUiPage::Now),
                                76.0,
                            ),
                            ui_button(
                                WorldUiButtonAction::SelectPage(WorldUiPage::System),
                                "SYSTEM",
                                WorldUiTextSlot::PageTab(WorldUiPage::System),
                                84.0,
                            ),
                            ui_button(
                                WorldUiButtonAction::SelectPage(WorldUiPage::Help),
                                "HELP",
                                WorldUiTextSlot::PageTab(WorldUiPage::Help),
                                76.0,
                            ),
                        ],
                    ),
                    (
                        Node {
                            width: percent(100),
                            flex_grow: 1.0,
                            ..default()
                        },
                        children![(ui_text(
                            "Loading the five-second read...",
                            WorldUiTextSlot::DrawerBody,
                            14.0,
                        ))],
                    ),
                    (
                        Node {
                            width: percent(100),
                            padding: UiRect::top(px(8)),
                            border: UiRect::top(px(1)),
                            ..default()
                        },
                        children![(ui_text(
                            "Campaign ready",
                            WorldUiTextSlot::DrawerFooter,
                            11.0,
                        ))],
                    ),
                ],
            ),
            (
                Node {
                    position_type: PositionType::Absolute,
                    left: px(18),
                    top: px(54),
                    height: px(32),
                    padding: UiRect::axes(px(10), px(5)),
                    border: UiRect::all(px(1)),
                    align_items: AlignItems::Center,
                    display: Display::None,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.075, 0.13, 0.11, 0.98)),
                BorderColor::all(Color::srgb(0.31, 0.62, 0.47)),
                WorldUiSurface::BattleBadge,
                children![(ui_text(
                    "OFFLINE WORLD · RTS BATTLE",
                    WorldUiTextSlot::BattleBadge,
                    10.0,
                ))],
            ),
        ],
    ));
}

pub(super) fn handle_world_ui_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<WorldUiState>,
) {
    if keys.just_pressed(KeyCode::F6) {
        state.drawer_open = !state.drawer_open;
    }
    if keys.just_pressed(KeyCode::F7) {
        state.page = state.page.next();
        state.drawer_open = true;
    }
}

pub(super) fn handle_world_ui_interactions(
    mut state: ResMut<WorldUiState>,
    buttons: Query<(&Interaction, &WorldUiButton), Changed<Interaction>>,
) {
    for (interaction, button) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match button.action {
            WorldUiButtonAction::ToggleDrawer => {
                state.drawer_open = !state.drawer_open;
            }
            WorldUiButtonAction::SelectPage(page) => {
                state.page = page;
                state.drawer_open = true;
            }
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(super) fn sync_world_ui(
    flow: Res<CampaignFlow>,
    map: Res<FirstContactMap>,
    online: Option<Res<OnlineAuthorityClient>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<WorldUiState>,
    mut surfaces: Query<(
        &WorldUiSurface,
        &mut Node,
        &mut BackgroundColor,
        &mut BorderColor,
        Option<&Interaction>,
        Option<&WorldUiButton>,
    )>,
    mut texts: Query<(
        &WorldUiTextSlot,
        &mut Text,
        &mut TextColor,
        &mut TextFont,
    )>,
    mut campaign_overlay: Query<
        &mut Node,
        (With<CampaignOverlayRoot>, Without<WorldUiSurface>),
    >,
) {
    let window_width = windows
        .iter()
        .next()
        .map(|window| window.width())
        .unwrap_or(1280.0);
    let viewport = UiViewportClass::from_width(window_width);
    if !state.initialized {
        state.drawer_open = viewport != UiViewportClass::Compact;
        state.initialized = true;
    }

    let battle = flow.mode == CampaignMode::Battle && flow.shell_mode == ShellMode::Playing;
    let drawer_visible = state.drawer_open && !battle;
    let metrics = UiLayoutMetrics::for_viewport(viewport, drawer_visible);
    let snapshot = WorldUiSnapshot::from_flow(&flow, online.is_some(), &map.objective.label);
    let palette = world_ui_palette(flow.settings.high_contrast);

    for (surface, mut node, mut background, mut border, interaction, button) in &mut surfaces {
        match *surface {
            WorldUiSurface::Header => {
                node.display = if battle { Display::None } else { Display::Flex };
                node.height = px(HEADER_HEIGHT);
                background.0 = palette.canvas;
                *border = BorderColor::all(palette.border);
            }
            WorldUiSurface::Drawer => {
                node.display = if drawer_visible {
                    Display::Flex
                } else {
                    Display::None
                };
                node.left = if viewport == UiViewportClass::Compact {
                    px(EDGE_GAP)
                } else {
                    Val::Auto
                };
                node.right = px(EDGE_GAP);
                node.top = if viewport == UiViewportClass::Compact {
                    Val::Auto
                } else {
                    px(HEADER_HEIGHT + EDGE_GAP / 2.0)
                };
                node.bottom = px(EDGE_GAP);
                node.width = metrics.drawer_width.map(px).unwrap_or(Val::Auto);
                node.height = metrics.drawer_height.map(px).unwrap_or(Val::Auto);
                background.0 = palette.surface;
                *border = BorderColor::all(palette.border);
            }
            WorldUiSurface::BattleBadge => {
                node.display = if battle { Display::Flex } else { Display::None };
                let (chip_background, chip_border, _) = palette.chip(snapshot.authority_tone);
                background.0 = chip_background;
                *border = BorderColor::all(chip_border);
            }
            WorldUiSurface::AuthorityChip => {
                node.display = if viewport == UiViewportClass::Compact {
                    Display::None
                } else {
                    Display::Flex
                };
                let (chip_background, chip_border, _) = palette.chip(snapshot.authority_tone);
                background.0 = chip_background;
                *border = BorderColor::all(chip_border);
            }
            WorldUiSurface::EconomyChip => {
                node.display = if viewport == UiViewportClass::Wide {
                    Display::Flex
                } else {
                    Display::None
                };
                let (chip_background, chip_border, _) = palette.chip(snapshot.economy_tone);
                background.0 = chip_background;
                *border = BorderColor::all(chip_border);
            }
            WorldUiSurface::Interactive => {
                let selected = button.is_some_and(|button| {
                    matches!(
                        button.action,
                        WorldUiButtonAction::SelectPage(page) if page == state.page
                    )
                });
                let (button_background, button_border, _) = palette.button(
                    interaction.copied().unwrap_or(Interaction::None),
                    selected,
                );
                background.0 = button_background;
                *border = BorderColor::all(button_border);
            }
        }
    }

    for mut overlay in &mut campaign_overlay {
        overlay.padding = if battle {
            UiRect::all(px(24))
        } else {
            UiRect {
                left: px(24),
                right: px(metrics.campaign_right_inset),
                top: px(metrics.campaign_top_inset),
                bottom: px(metrics.campaign_bottom_inset),
            }
        };
    }

    for (slot, mut text, mut color, mut font) in &mut texts {
        let (value, text_color, font_size) = match *slot {
            WorldUiTextSlot::Product => (
                "TRILLIONNIUM / WORLD".to_string(),
                palette.warning,
                18.0,
            ),
            WorldUiTextSlot::Context => {
                let value = if viewport == UiViewportClass::Compact {
                    snapshot.phase_label.clone()
                } else {
                    snapshot.context_line()
                };
                (value, palette.muted, 9.0)
            }
            WorldUiTextSlot::Observer => {
                let value = if viewport == UiViewportClass::Wide {
                    format!(
                        "NEXT · {}  ·  {}",
                        snapshot.next_action, snapshot.progress_label
                    )
                } else {
                    format!("NEXT · {}", snapshot.next_action)
                };
                (value, palette.accent, 12.0)
            }
            WorldUiTextSlot::Authority => {
                let value = if viewport == UiViewportClass::Wide {
                    format!(
                        "{}\n{}",
                        snapshot.authority_label, snapshot.authority_detail
                    )
                } else {
                    snapshot.authority_label.clone()
                };
                (value, palette.tone(snapshot.authority_tone), 10.0)
            }
            WorldUiTextSlot::Economy => (
                format!("{}\n{}", snapshot.economy_label, snapshot.economy_detail),
                palette.tone(snapshot.economy_tone),
                10.0,
            ),
            WorldUiTextSlot::DrawerToggle => (
                if state.drawer_open {
                    "F6 HIDE".to_string()
                } else {
                    "F6 GUIDE".to_string()
                },
                palette.text,
                12.0,
            ),
            WorldUiTextSlot::DrawerTitle => (
                format!("{} / PLAYER CONTROL CENTER", state.page.label()),
                palette.warning,
                15.0,
            ),
            WorldUiTextSlot::DrawerBody => (
                snapshot.body_for(state.page).to_string(),
                palette.text,
                metrics.body_font_size,
            ),
            WorldUiTextSlot::DrawerFooter => {
                let value = if state.page == WorldUiPage::Help {
                    format!("{}\n{}", snapshot.status, snapshot.input_label)
                } else {
                    snapshot.status.clone()
                };
                (value, palette.tone(snapshot.status_tone), 11.0)
            }
            WorldUiTextSlot::BattleBadge => (
                snapshot.battle_badge(),
                palette.tone(snapshot.authority_tone),
                10.0,
            ),
            WorldUiTextSlot::PageTab(page) => (
                if page == state.page {
                    format!("● {}", page.label())
                } else {
                    page.label().to_string()
                },
                if page == state.page {
                    palette.positive
                } else {
                    palette.text
                },
                12.0,
            ),
        };
        text.0 = value;
        color.0 = text_color;
        font.font_size = FontSize::Px(font_size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_ui_spawns_header_drawer_tabs_and_battle_badge() {
        let mut app = App::new();
        app.add_systems(Startup, spawn_world_ui);
        app.update();
        let world = app.world_mut();

        let mut roots = world.query_filtered::<(Entity, &FocusPolicy), With<WorldUiRoot>>();
        let roots = roots.iter(world).collect::<Vec<_>>();
        assert_eq!(roots.len(), 1);
        assert_eq!(*roots[0].1, FocusPolicy::Pass);

        let mut surfaces = world.query::<&WorldUiSurface>();
        let surface_values = surfaces.iter(world).copied().collect::<Vec<_>>();
        assert!(surface_values.contains(&WorldUiSurface::Header));
        assert!(surface_values.contains(&WorldUiSurface::Drawer));
        assert!(surface_values.contains(&WorldUiSurface::BattleBadge));

        let mut buttons = world.query::<&WorldUiButton>();
        let pages = buttons
            .iter(world)
            .filter_map(|button| match button.action {
                WorldUiButtonAction::SelectPage(page) => Some(page),
                WorldUiButtonAction::ToggleDrawer => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(pages.len(), 3);
        assert!(pages.contains(&WorldUiPage::Now));
        assert!(pages.contains(&WorldUiPage::System));
        assert!(pages.contains(&WorldUiPage::Help));
    }

    #[test]
    fn button_interactions_toggle_drawer_and_select_page() {
        let mut app = App::new();
        app.init_resource::<WorldUiState>()
            .add_systems(Update, handle_world_ui_interactions);
        let toggle = app
            .world_mut()
            .spawn(ui_button(
                WorldUiButtonAction::ToggleDrawer,
                "toggle",
                WorldUiTextSlot::DrawerToggle,
                80.0,
            ))
            .id();
        *app.world_mut().get_mut::<Interaction>(toggle).unwrap() = Interaction::Pressed;
        app.update();
        assert!(!app.world().resource::<WorldUiState>().drawer_open);

        *app.world_mut().get_mut::<Interaction>(toggle).unwrap() = Interaction::None;
        app.update();
        let system = app
            .world_mut()
            .spawn(ui_button(
                WorldUiButtonAction::SelectPage(WorldUiPage::System),
                "system",
                WorldUiTextSlot::PageTab(WorldUiPage::System),
                80.0,
            ))
            .id();
        *app.world_mut().get_mut::<Interaction>(system).unwrap() = Interaction::Pressed;
        app.update();
        let state = app.world().resource::<WorldUiState>();
        assert!(state.drawer_open);
        assert_eq!(state.page, WorldUiPage::System);
    }

    #[test]
    fn page_cycle_is_complete_and_stable() {
        assert_eq!(WorldUiPage::Now.next(), WorldUiPage::System);
        assert_eq!(WorldUiPage::System.next(), WorldUiPage::Help);
        assert_eq!(WorldUiPage::Help.next(), WorldUiPage::Now);
    }
}

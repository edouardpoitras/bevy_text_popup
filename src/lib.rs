#![doc = include_str!("../README.md")]

use bevy::{
    prelude::{
        default, App, ButtonBundle, Color, Commands, Component, DespawnRecursiveExt, Entity, Event,
        Handle, NodeBundle, Plugin, TextBundle, Update,
    },
    text::{Font, TextAlignment},
    ui::{UiRect, Val, ZIndex},
};

mod systems;
mod text_popup;

#[derive(Debug, Component)]
pub struct TextPopup;

#[derive(Debug, Component)]
pub struct TextPopupNeverExpires;

#[derive(Debug, Component)]
pub struct TextPopupExpires {
    pub expiration_time: f64,
}

#[derive(Debug)]
pub struct TextPopupRootNode(pub NodeBundle);

#[derive(Debug)]
pub struct TextPopupBorderNode(pub NodeBundle);

#[derive(Debug)]
pub struct TextPopupTextNode(pub TextBundle);

#[derive(Debug)]
pub struct TextPopupActionNode(pub NodeBundle);

#[derive(Debug)]
pub struct TextPopupButtonNode(pub ButtonBundle);

#[derive(Debug)]
pub struct TextPopupButtonTextNode(pub TextBundle);

/// Users send these events to create text popups.
#[derive(Debug, Event)]
pub struct TextPopupEvent {
    pub content: String,
    pub font: Option<Handle<Font>>,
    pub font_size: f32,
    pub font_color: Color,
    pub border: UiRect,
    pub border_color: Color,
    pub padding: UiRect,
    pub margin: UiRect,
    pub modal: Option<Color>,
    pub text_alignment: TextAlignment,
    pub background_color: Color,
    pub confirm_button: Option<TextPopupButton>,
    pub dismiss_button: Option<TextPopupButton>,
    pub location: TextPopupLocation,
    pub z_index: ZIndex,
    pub timeout: TextPopupTimeout,
}

impl Default for TextPopupEvent {
    fn default() -> Self {
        Self {
            content: default(),
            font: default(),
            font_size: 32.0,
            font_color: Color::WHITE,
            border: UiRect::all(Val::Px(5.)),
            border_color: Color::WHITE.with_a(0.5),
            padding: UiRect::all(Val::Px(20.)),
            margin: UiRect::all(Val::Px(10.)),
            modal: None,
            text_alignment: TextAlignment::Center,
            background_color: Color::BLACK,
            confirm_button: default(),
            dismiss_button: default(),
            location: default(),
            z_index: ZIndex::Global(i32::MAX),
            timeout: TextPopupTimeout::Never,
        }
    }
}

#[derive(Debug, Default)]
pub enum TextPopupLocation {
    TopLeft,
    Top,
    TopRight,
    Left,
    #[default]
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

#[derive(Debug)]
pub enum TextPopupTimeout {
    Never,
    Seconds(u32),
}

#[derive(Debug, Clone)]
pub struct TextPopupButton {
    pub text: String,
    pub background_color: Color,
    pub font: Option<Handle<Font>>,
    pub font_size: f32,
    pub font_color: Color,
    pub action: fn(&mut Commands, Entity),
}

impl Default for TextPopupButton {
    fn default() -> Self {
        Self {
            text: "OK".to_string(),
            background_color: Color::WHITE,
            font: default(),
            font_size: 24.0,
            font_color: Color::BLACK,
            action: |commands, entity| commands.entity(entity).despawn_recursive(),
        }
    }
}

#[derive(Debug, Component)]
pub struct TextPopupButtonActionData {
    pub root_id: Entity,
    pub action: fn(&mut Commands, Entity),
}

pub struct TextPopupPlugin;

impl Plugin for TextPopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TextPopupEvent>().add_systems(
            Update,
            (
                systems::handle_text_popup_events,
                systems::cleanup_expired_text_popups,
                systems::text_popup_button_system,
            ),
        );
    }
}

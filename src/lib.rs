#![doc = include_str!("../README.md")]

use bevy::{
    prelude::{
        default, Alpha, App, Color, Commands, Component, Entity, Event, Name, Node, Plugin, Text,
        Update,
    },
    text::{JustifyText, TextColor, TextFont, TextLayout},
    ui::{BackgroundColor, BorderColor, GlobalZIndex, UiRect, Val},
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

#[derive(Debug, Default)]
pub struct TextPopupRootNode {
    pub node: Node,
    pub background_color: BackgroundColor,
}

#[derive(Debug, Default)]
pub struct TextPopupTextNode {
    pub node: Node,
    pub border_color: BorderColor,
    pub background_color: BackgroundColor,
    pub text: Text,
    pub text_layout: TextLayout,
    pub text_font: TextFont,
    pub text_color: TextColor,
}

#[derive(Debug)]
pub struct TextPopupActionNode(pub Node);

/// Users send these events to create text popups.
#[derive(Debug, Event)]
pub struct TextPopupEvent {
    pub content: String,
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub border: UiRect,
    pub border_color: BorderColor,
    pub padding: UiRect,
    pub margin: UiRect,
    pub modal: Option<BackgroundColor>,
    pub text_alignment: JustifyText,
    pub background_color: BackgroundColor,
    pub confirm_button: Option<TextPopupButton>,
    pub dismiss_button: Option<TextPopupButton>,
    pub location: TextPopupLocation,
    pub z_index: GlobalZIndex,
    pub timeout: TextPopupTimeout,
    pub name: Option<Name>,
}

impl Default for TextPopupEvent {
    fn default() -> Self {
        Self {
            content: default(),
            text_font: TextFont {
                font_size: 32.0,
                ..Default::default()
            },
            text_color: TextColor::WHITE,
            border: UiRect::all(Val::Px(5.)),
            border_color: BorderColor::from(Color::WHITE.with_alpha(0.5)),
            padding: UiRect::all(Val::Px(5.)),
            margin: UiRect::all(Val::Px(5.)),
            modal: None,
            text_alignment: JustifyText::Center,
            background_color: BackgroundColor::from(Color::BLACK),
            confirm_button: default(),
            dismiss_button: default(),
            location: default(),
            z_index: GlobalZIndex(i32::MAX),
            timeout: TextPopupTimeout::Never,
            name: None,
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
    /// Custom location specified by x and y coordinates in pixels
    Custom(f32, f32),
}

#[derive(Debug)]
pub enum TextPopupTimeout {
    Never,
    Seconds(u32),
}

#[derive(Debug, Clone)]
pub struct TextPopupButton {
    pub text: String,
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub border: UiRect,
    pub border_color: BorderColor,
    pub padding: UiRect,
    pub margin: UiRect,
    pub background_color: BackgroundColor,
    pub action: fn(&mut Commands, Entity),
}

impl Default for TextPopupButton {
    fn default() -> Self {
        Self {
            text: "OK".to_string(),
            text_font: TextFont {
                font_size: 24.0,
                ..Default::default()
            },
            text_color: TextColor::from(Color::WHITE),
            border: UiRect::all(Val::Px(0.0)),
            border_color: BorderColor::from(Color::WHITE),
            padding: UiRect::all(Val::Px(0.0)),
            margin: UiRect::all(Val::Px(5.0)),
            background_color: BackgroundColor::from(Color::BLACK),
            action: |commands, entity| commands.entity(entity).despawn(),
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

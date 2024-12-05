use bevy::prelude::*;
use bevy_text_popup::{TextPopupEvent, TextPopupLocation, TextPopupPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextPopupPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut text_popup_events: EventWriter<TextPopupEvent>) {
    commands.spawn(Camera2d::default());

    text_popup_events.send(TextPopupEvent {
        content: "Transparent Background".to_string(),
        text_font: TextFont {
            font_size: 64.0,
            ..Default::default()
        },
        background_color: Color::linear_rgba(0., 0., 0., 0.5).into(),
        location: TextPopupLocation::Top,
        ..default()
    });

    text_popup_events.send(TextPopupEvent {
        content: "Transparent Text".to_string(),
        text_font: TextFont {
            font_size: 64.0,
            ..Default::default()
        },
        text_color: Color::linear_rgba(1., 1., 1., 0.5).into(),
        location: TextPopupLocation::Bottom,
        ..default()
    });
}

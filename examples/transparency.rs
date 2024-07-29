use bevy::prelude::*;
use bevy_text_popup::{TextPopupEvent, TextPopupLocation, TextPopupPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextPopupPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut text_popup_events: EventWriter<TextPopupEvent>) {
    commands.spawn(Camera2dBundle::default());

    text_popup_events.send(TextPopupEvent {
        content: "Transparent Background".to_string(),
        font_size: 64.0,
        background_color: Color::BLACK.with_alpha(0.5),
        location: TextPopupLocation::Top,
        ..default()
    });

    text_popup_events.send(TextPopupEvent {
        content: "Transparent Text".to_string(),
        font_size: 64.0,
        font_color: Color::WHITE.with_alpha(0.5),
        location: TextPopupLocation::Bottom,
        ..default()
    });
}

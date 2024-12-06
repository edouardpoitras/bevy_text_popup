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

    let custom_positions = vec![(100.0, 100.0), (300.0, 200.0), (500.0, 300.0)];

    for (x, y) in custom_positions {
        text_popup_events.send(TextPopupEvent {
            content: format!("Custom ({}, {})", x, y),
            location: TextPopupLocation::Custom(x, y),
            ..default()
        });
    }
}

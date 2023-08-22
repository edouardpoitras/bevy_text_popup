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

    let popup_locations = vec![
        TextPopupLocation::TopLeft,
        TextPopupLocation::Top,
        TextPopupLocation::TopRight,
        TextPopupLocation::Left,
        TextPopupLocation::Center,
        TextPopupLocation::Right,
        TextPopupLocation::BottomLeft,
        TextPopupLocation::Bottom,
        TextPopupLocation::BottomRight,
    ];

    for location in popup_locations {
        text_popup_events.send(TextPopupEvent {
            content: format!("{:?}", location),
            location,
            ..default()
        });
    }
}

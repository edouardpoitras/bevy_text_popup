use bevy::prelude::*;
use bevy_text_popup::{TextPopupEvent, TextPopupLocation, TextPopupPlugin, TextPopupTimeout};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextPopupPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_new_popups)
        .run();
}

fn setup(mut commands: Commands, mut text_popup_events: MessageWriter<TextPopupEvent>) {
    commands.spawn(Camera2d::default());

    text_popup_events.write(TextPopupEvent {
        content: "I'll disappear after 60 frames".to_string(),
        text_font: TextFont {
            font_size: 32.0,
            ..Default::default()
        },
        location: TextPopupLocation::Top,
        timeout: TextPopupTimeout::Frames(60),
        ..default()
    });

    text_popup_events.write(TextPopupEvent {
        content: "I'll disappear after 120 frames".to_string(),
        text_font: TextFont {
            font_size: 32.0,
            ..Default::default()
        },
        location: TextPopupLocation::Center,
        timeout: TextPopupTimeout::Frames(120),
        ..default()
    });

    text_popup_events.write(TextPopupEvent {
        content: "I'll disappear after 240 frames".to_string(),
        text_font: TextFont {
            font_size: 32.0,
            ..Default::default()
        },
        location: TextPopupLocation::Bottom,
        timeout: TextPopupTimeout::Frames(240),
        background_color: LinearRgba::new(0.0, 1.0, 0.0, 0.8).into(),
        ..default()
    });
}

// This system demonstrates that you can also spawn popups during the game
fn spawn_new_popups(
    mut text_popup_events: MessageWriter<TextPopupEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        // Create a text popup that expires after 2 seconds when space is pressed
        text_popup_events.write(TextPopupEvent {
            content: "Space pressed! I'll disappear after 2 seconds".to_string(),
            text_font: TextFont {
                font_size: 24.0,
                ..Default::default()
            },
            location: TextPopupLocation::Right,
            timeout: TextPopupTimeout::Seconds(2),
            background_color: LinearRgba::new(0.0, 0.0, 1.0, 0.7).into(),
            ..default()
        });
    }
}

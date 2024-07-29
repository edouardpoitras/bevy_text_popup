use bevy::prelude::*;
use bevy_text_popup::{TextPopupEvent, TextPopupPlugin, TextPopupTimeout};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextPopupPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut text_popup_events: EventWriter<TextPopupEvent>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(TextBundle::from_section(
        "Some Other Element",
        TextStyle {
            font_size: 40.,
            color: Color::WHITE,
            ..Default::default()
        },
    ));

    text_popup_events.send(TextPopupEvent {
        content: "Modal Example".to_string(),
        modal: Some(Color::srgb(0., 0., 100.).with_alpha(0.75)),
        timeout: TextPopupTimeout::Seconds(5),
        ..default()
    });
}

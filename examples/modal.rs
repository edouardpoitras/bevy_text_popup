use bevy::prelude::*;
use bevy_text_popup::{TextPopupEvent, TextPopupPlugin, TextPopupTimeout};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextPopupPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut text_popup_events: EventWriter<TextPopupEvent>) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Text::new("Some Other Element"),
        TextFont {
            font_size: 40.,
            ..Default::default()
        },
        TextColor::from(Color::WHITE),
    ));

    text_popup_events.send(TextPopupEvent {
        content: "Modal Example".to_string(),
        modal: Some(Color::linear_rgba(0., 0., 100., 0.75).into()),
        timeout: TextPopupTimeout::Seconds(5),
        ..default()
    });
}

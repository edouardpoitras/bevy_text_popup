use bevy::prelude::*;
use bevy_text_popup::{TextPopupEvent, TextPopupPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextPopupPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut text_popup_events: EventWriter<TextPopupEvent>) {
    commands.spawn(Camera2dBundle::default());

    let event = TextPopupEvent {
        content: "Border Example".to_string(),
        border: UiRect::all(Val::Px(25.)),
        border_color: Color::srgb(100., 0., 0.),
        ..default()
    };
    text_popup_events.send(event);
}

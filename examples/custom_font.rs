use bevy::prelude::*;
use bevy_text_popup::{TextPopupEvent, TextPopupPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextPopupPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut text_popup_events: EventWriter<TextPopupEvent>,
) {
    commands.spawn(Camera2dBundle::default());

    text_popup_events.send(TextPopupEvent {
        content: "Custom Font Example".to_string(),
        font: Some(asset_server.load("fonts/FiraSans-Bold.ttf")),
        font_size: 64.0,
        font_color: Color::srgb(100., 0., 0.),
        ..default()
    });
}

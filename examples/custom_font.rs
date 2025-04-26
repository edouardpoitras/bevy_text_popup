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
    commands.spawn(Camera2d::default());

    text_popup_events.write(TextPopupEvent {
        content: "Custom Font Example".to_string(),
        text_font: TextFont {
            font_size: 64.0,
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            ..Default::default()
        },
        text_color: Color::srgb(100., 0., 0.).into(),
        padding: UiRect::all(Val::Px(25.)),
        ..default()
    });
}

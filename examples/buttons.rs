use bevy::prelude::*;
use bevy_text_popup::{TextPopupButton, TextPopupEvent, TextPopupLocation, TextPopupPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextPopupPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut text_popup_events: EventWriter<TextPopupEvent>) {
    commands.spawn(Camera2dBundle::default());

    let event = TextPopupEvent {
        content: "Close this popup and generate a new one at the bottom?".to_string(),
        confirm_button: Some(TextPopupButton {
            text: "OK".to_string(),
            action: |commands, root_entity| {
                // Fire event to spawn a new popup when user clicks 'OK'.
                commands.add(|world: &mut World| {
                    world.send_event(TextPopupEvent {
                        content: "New Popup Generated".to_string(),
                        location: TextPopupLocation::Bottom,
                        ..Default::default()
                    });
                });
                // Despawn the original popup.
                commands.entity(root_entity).despawn_recursive();
            },
            ..Default::default()
        }),
        dismiss_button: Some(TextPopupButton {
            text: "Cancel".to_string(),
            background_color: Color::srgb(100., 0. , 0.),
            ..Default::default()
        }),
        ..default()
    };
    text_popup_events.send(event);
}

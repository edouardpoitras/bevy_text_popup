use bevy::prelude::*;
use bevy_text_popup::{TextPopup, TextPopupEvent, TextPopupPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextPopupPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands, mut text_popup_events: MessageWriter<TextPopupEvent>) {
    commands.spawn(Camera2d::default());

    let event = TextPopupEvent {
        content: "Named Example".to_string(),
        name: Some(Name::new("named_example")),
        ..default()
    };
    text_popup_events.write(event);
}

fn update(
    mut commands: Commands,
    query: Query<(Entity, &Name, &TextPopup)>,
    mut iterations: Local<usize>,
) {
    *iterations += 1;
    if *iterations > 120 {
        for (entity, name, _) in query.iter() {
            if name.as_str() == "named_example" {
                println!(
                    "Deleting our named popup root entity: {:?} with name: {:?}",
                    entity, name
                );
                commands.entity(entity).despawn();
            }
        }
    }
}

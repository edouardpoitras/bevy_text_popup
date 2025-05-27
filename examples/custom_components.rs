use bevy::prelude::*;
use bevy_text_popup::{
    TextPopup, TextPopupEvent, TextPopupLocation, TextPopupPlugin, TextPopupTimeout,
};

#[derive(Component, Debug)]
struct GamepadTipsRight;

#[derive(Component, Debug)]
struct PlayerOneUI;

#[derive(Component, Debug)]
struct NpcDialogue {
    npc_id: u32,
}

#[derive(Component, Debug)]
struct NotificationMessage;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextPopupPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (query_popups, cleanup_system))
        .run();
}

fn setup(mut commands: Commands, mut text_popup_events: EventWriter<TextPopupEvent>) {
    commands.spawn(Camera2d::default());

    // Example 1: Gamepad tip with custom marker component
    text_popup_events.write(TextPopupEvent {
        content: "Press A to interact".to_string(),
        timeout: TextPopupTimeout::Seconds(3),
        location: TextPopupLocation::Top,
        custom_component: Some(|entity_commands| {
            entity_commands.insert(GamepadTipsRight);
        }),
        ..default()
    });

    // Example 2: Player-specific UI popup with multiple components
    text_popup_events.write(TextPopupEvent {
        content: "Player 1 Health: 100%".to_string(),
        timeout: TextPopupTimeout::Seconds(5),
        custom_component: Some(|entity_commands| {
            entity_commands.insert((PlayerOneUI, NotificationMessage));
        }),
        ..default()
    });

    // Example 3: NPC dialogue with custom data
    text_popup_events.write(TextPopupEvent {
        content: "Hello, traveler! Welcome to our village.".to_string(),
        timeout: TextPopupTimeout::Frames(300),
        location: TextPopupLocation::Bottom,
        custom_component: Some(|entity_commands| {
            entity_commands.insert(NpcDialogue { npc_id: 42 });
        }),
        ..default()
    });
}

fn query_popups(
    gamepad_tips: Query<Entity, (With<TextPopup>, With<GamepadTipsRight>)>,
    player_ui: Query<Entity, (With<TextPopup>, With<PlayerOneUI>)>,
    npc_dialogues: Query<&NpcDialogue, With<TextPopup>>,
) {
    // Query for gamepad tips
    for entity in gamepad_tips.iter() {
        info!("Found gamepad tip popup: {:?}", entity);
    }

    // Query for player-specific UI
    for entity in player_ui.iter() {
        info!("Found Player 1 UI popup: {:?}", entity);
    }

    // Query for NPC dialogues with data
    for npc_dialogue in npc_dialogues.iter() {
        info!("Found NPC dialogue from NPC ID: {}", npc_dialogue.npc_id);
    }
}

fn cleanup_system(
    mut commands: Commands,
    query: Query<Entity, (With<TextPopup>, With<NpcDialogue>)>,
    mut iterations: Local<usize>,
) {
    *iterations += 1;
    // Demonstrate custom cleanup logic for NPC dialogues after 4 seconds
    if *iterations > 240 {
        for entity in query.iter() {
            info!("Custom cleanup: Removing NPC dialogue popup: {:?}", entity);
            commands.entity(entity).despawn();
        }
    }
}

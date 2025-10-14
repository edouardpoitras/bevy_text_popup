use bevy::{
    prelude::{Changed, Children, Commands, Entity, MessageReader, Query, Res},
    time::Time,
    ui::Interaction,
};

use crate::{
    text_popup::generate_text_popup_from_event, TextPopupButtonActionData, TextPopupEvent,
    TextPopupExpires, TextPopupExpiresInFrames,
};

pub fn handle_text_popup_events(
    mut commands: Commands,
    mut text_popup_events: MessageReader<TextPopupEvent>,
    time: Res<Time>,
) {
    for text_popup_event in text_popup_events.read() {
        generate_text_popup_from_event(&mut commands, &time, text_popup_event);
    }
}

pub fn cleanup_expired_text_popups(
    mut commands: Commands,
    text_popups: Query<(Entity, &TextPopupExpires)>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs_f64();
    for (entity, text_popup) in text_popups.iter() {
        if text_popup.expiration_time < current_time {
            commands.entity(entity).despawn();
        }
    }
}

pub fn text_popup_button_system(
    mut interaction_query: Query<
        (&Interaction, &TextPopupButtonActionData, &Children),
        Changed<Interaction>,
    >,
    mut commands: Commands,
) {
    for (interaction, TextPopupButtonActionData { root_id, action }, _children) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                action(&mut commands, *root_id);
            },
            Interaction::Hovered => {
                // Hover color change here.
            },
            Interaction::None => {
                // Back to normal after a hover.
            },
        }
    }
}

pub fn cleanup_frame_expired_text_popups(
    mut commands: Commands,
    mut text_popups: Query<(Entity, &mut TextPopupExpiresInFrames)>,
) {
    for (entity, mut text_popup) in text_popups.iter_mut() {
        text_popup.frames_remaining = text_popup.frames_remaining.saturating_sub(1);
        if text_popup.frames_remaining == 0 {
            commands.entity(entity).despawn();
        }
    }
}

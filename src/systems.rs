use bevy::{
    prelude::{Changed, Children, Commands, DespawnRecursiveExt, Entity, EventReader, Query, Res},
    time::Time,
    ui::Interaction,
};

use crate::{
    text_popup::generate_text_popup_from_event, TextPopupButtonActionData, TextPopupEvent,
    TextPopupExpires,
};

pub fn handle_text_popup_events(
    mut commands: Commands,
    mut text_popup_events: EventReader<TextPopupEvent>,
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
    let current_time = time.elapsed_seconds_f64();
    for (entity, text_popup) in text_popups.iter() {
        if text_popup.expiration_time < current_time {
            commands.entity(entity).despawn_recursive();
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

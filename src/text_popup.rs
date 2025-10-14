use bevy::{
    prelude::{Button, Commands, Node, Text},
    text::{Justify, TextLayout},
    time::Time,
    ui::{AlignItems, Display, FlexDirection, GlobalZIndex, JustifyContent, PositionType, Val},
};

use crate::{
    TextPopup, TextPopupActionNode, TextPopupButtonActionData, TextPopupEvent, TextPopupExpires,
    TextPopupExpiresInFrames, TextPopupLocation, TextPopupNeverExpires, TextPopupRootNode,
    TextPopupTextNode, TextPopupTimeout,
};

pub fn generate_text_popup_from_event(
    commands: &mut Commands,
    time: &Time,
    text_popup_event: &TextPopupEvent,
) {
    let root_node = get_root_node(text_popup_event);
    let text_node = get_text_node(text_popup_event);
    let action_node = get_action_node();
    spawn_text_popup(
        commands,
        time,
        text_popup_event,
        root_node,
        text_node,
        action_node,
        text_popup_event.z_index,
    );
}

fn get_root_node(text_popup_event: &TextPopupEvent) -> TextPopupRootNode {
    let justify_content = match text_popup_event.location {
        TextPopupLocation::TopLeft | TextPopupLocation::Left | TextPopupLocation::BottomLeft => {
            JustifyContent::Start
        },
        TextPopupLocation::Top | TextPopupLocation::Center | TextPopupLocation::Bottom => {
            JustifyContent::Center
        },
        TextPopupLocation::TopRight | TextPopupLocation::Right | TextPopupLocation::BottomRight => {
            JustifyContent::End
        },
        TextPopupLocation::Custom(_, _) => JustifyContent::Start,
    };
    let node = Node {
        position_type: PositionType::Absolute,
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        justify_content,
        align_items: AlignItems::Center,
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        ..Default::default()
    };
    TextPopupRootNode {
        node,
        background_color: text_popup_event.modal.unwrap_or_default(),
    }
}

fn get_text_node(text_popup_event: &TextPopupEvent) -> TextPopupTextNode {
    let mut node = Node {
        position_type: PositionType::Absolute,
        border: text_popup_event.border,
        padding: text_popup_event.padding,
        margin: text_popup_event.margin,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..Default::default()
    };
    match text_popup_event.location {
        TextPopupLocation::TopLeft | TextPopupLocation::Top | TextPopupLocation::TopRight => {
            node.top = Val::Percent(0.);
        },
        TextPopupLocation::Left => node.left = Val::Percent(0.),
        TextPopupLocation::Center => {},
        TextPopupLocation::Right => node.right = Val::Percent(0.),
        TextPopupLocation::BottomLeft
        | TextPopupLocation::Bottom
        | TextPopupLocation::BottomRight => {
            node.bottom = Val::Percent(0.);
        },
        TextPopupLocation::Custom(x, y) => {
            node.left = Val::Px(x);
            node.top = Val::Px(y);
        },
    };
    TextPopupTextNode {
        node,
        border_color: text_popup_event.border_color,
        background_color: text_popup_event.background_color,
        text: Text::new(text_popup_event.content.clone()),
        text_layout: TextLayout {
            justify: text_popup_event.text_alignment,
            ..Default::default()
        },
        text_font: text_popup_event.text_font.clone(),
        text_color: text_popup_event.text_color,
    }
}

fn get_action_node() -> TextPopupActionNode {
    TextPopupActionNode(Node {
        padding: bevy::ui::UiRect::top(Val::Px(5.)),
        ..Default::default()
    })
}

fn spawn_text_popup(
    commands: &mut Commands,
    time: &Time,
    text_popup_event: &TextPopupEvent,
    root_node: TextPopupRootNode,
    text_node: TextPopupTextNode,
    action_node: TextPopupActionNode,
    z_index: GlobalZIndex,
) {
    let mut spawned_root = commands.spawn((
        TextPopup,
        root_node.node,
        root_node.background_color,
        z_index,
    ));
    let spawned_root = match text_popup_event.timeout {
        TextPopupTimeout::Seconds(seconds) => spawned_root.insert(TextPopupExpires {
            expiration_time: time.elapsed_secs_f64() + seconds as f64,
        }),
        TextPopupTimeout::Frames(frames) => spawned_root.insert(TextPopupExpiresInFrames {
            frames_remaining: frames,
        }),
        TextPopupTimeout::Never => spawned_root.insert(TextPopupNeverExpires),
    };
    if let Some(name) = &text_popup_event.name {
        spawned_root.insert(name.clone());
    }
    if let Some(custom_component_fn) = text_popup_event.custom_component {
        custom_component_fn(spawned_root);
    }
    let root_id = spawned_root.id();
    spawned_root.with_children(|commands| {
        commands
            .spawn((
                text_node.node,
                text_node.border_color,
                text_node.background_color,
            ))
            .with_children(|commands| {
                commands.spawn((
                    text_node.text,
                    text_node.text_layout,
                    text_node.text_font,
                    text_node.text_color,
                ));
                commands.spawn(action_node.0).with_children(|commands| {
                    if let Some(confirm_button) = text_popup_event.confirm_button.clone() {
                        commands
                            .spawn((
                                Button,
                                Node {
                                    border: confirm_button.border,
                                    padding: confirm_button.padding,
                                    margin: confirm_button.margin,
                                    ..Default::default()
                                },
                                confirm_button.border_color,
                                TextLayout::new_with_justify(Justify::Center),
                                TextPopupButtonActionData {
                                    root_id,
                                    action: text_popup_event
                                        .confirm_button
                                        .as_ref()
                                        .unwrap()
                                        .action,
                                },
                            ))
                            .with_children(|commands| {
                                commands.spawn((
                                    Text::new(confirm_button.text),
                                    confirm_button.text_font,
                                    confirm_button.text_color,
                                    confirm_button.background_color,
                                ));
                            });
                    }
                    if let Some(dismiss_button) = text_popup_event.dismiss_button.clone() {
                        commands
                            .spawn((
                                Button,
                                Node {
                                    border: dismiss_button.border,
                                    padding: dismiss_button.padding,
                                    margin: dismiss_button.margin,
                                    ..Default::default()
                                },
                                dismiss_button.border_color,
                                TextLayout::new_with_justify(Justify::Center),
                                TextPopupButtonActionData {
                                    root_id,
                                    action: text_popup_event
                                        .dismiss_button
                                        .as_ref()
                                        .unwrap()
                                        .action,
                                },
                            ))
                            .with_children(|commands| {
                                commands.spawn((
                                    Text::new(dismiss_button.text),
                                    dismiss_button.text_font,
                                    dismiss_button.text_color,
                                    dismiss_button.background_color,
                                ));
                            });
                    }
                });
            });
    });
}

use bevy::{
    prelude::{BuildChildren, ButtonBundle, Commands, NodeBundle, TextBundle},
    text::TextStyle,
    time::Time,
    ui::{AlignItems, Display, FlexDirection, JustifyContent, PositionType, Style, Val},
};

use crate::{
    TextPopup, TextPopupActionNode, TextPopupBorderNode, TextPopupButton,
    TextPopupButtonActionData, TextPopupButtonNode, TextPopupButtonTextNode, TextPopupEvent,
    TextPopupExpires, TextPopupLocation, TextPopupNeverExpires, TextPopupRootNode,
    TextPopupTextNode, TextPopupTimeout,
};

pub fn generate_text_popup_from_event(
    commands: &mut Commands,
    time: &Time,
    text_popup_event: &TextPopupEvent,
) {
    let text_style = get_text_style(text_popup_event);
    let border_style = get_border_style(text_popup_event);
    let root_node = get_root_node(text_popup_event);
    let border_node = get_border_node(text_popup_event, border_style);
    let text_node = get_text_node(text_popup_event, text_style);
    let action_node = get_action_node();
    let confirm_button_node = get_button_node(&text_popup_event.confirm_button);
    let confirm_button_text_node = get_button_text_node(&text_popup_event.confirm_button);
    let dismiss_button_node = get_button_node(&text_popup_event.dismiss_button);
    let dismiss_button_text_node = get_button_text_node(&text_popup_event.dismiss_button);
    spawn_text_popup(
        commands,
        time,
        text_popup_event,
        root_node,
        border_node,
        text_node,
        action_node,
        confirm_button_node,
        confirm_button_text_node,
        dismiss_button_node,
        dismiss_button_text_node,
    );
}

fn get_text_style(text_popup_event: &TextPopupEvent) -> TextStyle {
    if text_popup_event.font.is_some() {
        TextStyle {
            font: text_popup_event.font.clone().unwrap(),
            font_size: text_popup_event.font_size,
            color: text_popup_event.font_color,
        }
    } else {
        TextStyle {
            font_size: text_popup_event.font_size,
            color: text_popup_event.font_color,
            ..Default::default()
        }
    }
}

fn get_border_style(text_popup_event: &TextPopupEvent) -> Style {
    let mut style = Style {
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
            style.top = Val::Percent(0.);
        },
        TextPopupLocation::Left => style.left = Val::Percent(0.),
        TextPopupLocation::Center => {},
        TextPopupLocation::Right => style.right = Val::Percent(0.),
        TextPopupLocation::BottomLeft
        | TextPopupLocation::Bottom
        | TextPopupLocation::BottomRight => {
            style.bottom = Val::Percent(0.);
        },
    };
    style
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
    };
    let style = Style {
        position_type: PositionType::Absolute,
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        justify_content,
        align_items: AlignItems::Center,
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        ..Default::default()
    };
    let mut node_bundle = NodeBundle {
        style,
        ..Default::default()
    };
    if text_popup_event.modal.is_some() {
        node_bundle.background_color = text_popup_event.modal.unwrap().into();
    }
    TextPopupRootNode(node_bundle)
}

fn get_border_node(text_popup_event: &TextPopupEvent, border_style: Style) -> TextPopupBorderNode {
    TextPopupBorderNode(NodeBundle {
        style: border_style,
        border_color: text_popup_event.border_color.into(),
        background_color: text_popup_event.background_color.into(),
        ..Default::default()
    })
}

fn get_text_node(text_popup_event: &TextPopupEvent, text_style: TextStyle) -> TextPopupTextNode {
    let mut text_bundle = TextBundle::from_section(text_popup_event.content.clone(), text_style)
        .with_text_alignment(text_popup_event.text_alignment);
    text_bundle.z_index = text_popup_event.z_index;
    TextPopupTextNode(text_bundle)
}

fn get_action_node() -> TextPopupActionNode {
    TextPopupActionNode(NodeBundle {
        style: Style {
            padding: bevy::ui::UiRect::top(Val::Px(5.)),
            ..Default::default()
        },
        ..Default::default()
    })
}

fn get_button_node(text_popup_button: &Option<TextPopupButton>) -> Option<TextPopupButtonNode> {
    if text_popup_button.is_none() {
        return None;
    }
    Some(TextPopupButtonNode(ButtonBundle {
        style: Style {
            padding: bevy::ui::UiRect::all(Val::Px(5.0)),
            margin: bevy::ui::UiRect::all(Val::Px(5.0)),
            ..Default::default()
        },
        background_color: text_popup_button.clone().unwrap().background_color.into(),
        ..Default::default()
    }))
}

fn get_button_text_node(
    text_popup_button: &Option<TextPopupButton>,
) -> Option<TextPopupButtonTextNode> {
    if text_popup_button.is_none() {
        return None;
    }
    let text_popup_button = text_popup_button.clone().unwrap();
    let mut button_text_node = if text_popup_button.font.is_some() {
        TextBundle::from_section(
            text_popup_button.text.clone(),
            TextStyle {
                font: text_popup_button.font.clone().unwrap(),
                font_size: text_popup_button.font_size,
                color: text_popup_button.font_color,
            },
        )
    } else {
        TextBundle::from_section(
            text_popup_button.text.clone(),
            TextStyle {
                font_size: text_popup_button.font_size,
                color: text_popup_button.font_color,
                ..Default::default()
            },
        )
    };
    button_text_node = button_text_node.with_background_color(text_popup_button.background_color);
    Some(TextPopupButtonTextNode(button_text_node))
}

#[allow(clippy::too_many_arguments)]
fn spawn_text_popup(
    commands: &mut Commands,
    time: &Time,
    text_popup_event: &TextPopupEvent,
    root_node: TextPopupRootNode,
    border_node: TextPopupBorderNode,
    text_node: TextPopupTextNode,
    action_node: TextPopupActionNode,
    confirm_button_node: Option<TextPopupButtonNode>,
    confirm_button_text_node: Option<TextPopupButtonTextNode>,
    dismiss_button_node: Option<TextPopupButtonNode>,
    dismiss_button_text_node: Option<TextPopupButtonTextNode>,
) {
    let mut spawned_root = commands.spawn(root_node.0);
    let spawned_root = if let TextPopupTimeout::Seconds(seconds) = text_popup_event.timeout {
        spawned_root.insert((
            TextPopup,
            TextPopupExpires {
                expiration_time: time.elapsed_seconds_f64() + seconds as f64,
            },
        ))
    } else {
        spawned_root.insert((TextPopup, TextPopupNeverExpires))
    };
    let root_id = spawned_root.id();
    spawned_root.with_children(|commands| {
        commands.spawn(border_node.0).with_children(|commands| {
            commands.spawn(text_node.0);
            commands.spawn(action_node.0).with_children(|commands| {
                if let (Some(confirm_button_node), Some(confirm_button_text_node)) =
                    (confirm_button_node, confirm_button_text_node)
                {
                    commands
                        .spawn((
                            confirm_button_node.0,
                            TextPopupButtonActionData {
                                root_id,
                                action: text_popup_event.confirm_button.as_ref().unwrap().action,
                            },
                        ))
                        .with_children(|commands| {
                            commands.spawn(confirm_button_text_node.0);
                        });
                }
                if let (Some(dismiss_button_node), Some(dismiss_button_text_node)) =
                    (dismiss_button_node, dismiss_button_text_node)
                {
                    commands
                        .spawn((
                            dismiss_button_node.0,
                            TextPopupButtonActionData {
                                root_id,
                                action: text_popup_event.dismiss_button.as_ref().unwrap().action,
                            },
                        ))
                        .with_children(|commands| {
                            commands.spawn(dismiss_button_text_node.0);
                        });
                }
            });
        });
    });
}

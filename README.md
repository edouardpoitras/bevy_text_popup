# Bevy Text Popup

[![Bevy Text Popup](https://github.com/edouardpoitras/bevy_text_popup/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/edouardpoitras/bevy_text_popup/actions/workflows/rust.yml)
[![Latest version](https://img.shields.io/crates/v/bevy_text_popup.svg)](https://crates.io/crates/bevy_text_popup)
[![Documentation](https://docs.rs/bevy_text_popup/badge.svg)](https://docs.rs/bevy_text_popup)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![Apache](https://img.shields.io/badge/license-Apache-blue.svg)

Bevy plugin to easily create UI text popups with events.

Popups are meant to be short-lived and on top of all other UI components.

Useful for notifications and prompting user input.

Current Customization Options:
- Font: `cargo run --example custom_font`
- Background: Color and Transparency (image background to come)
- Border: `cargo run --example border`
- Buttons: `cargo run --example buttons`
- Timeouts: Dismiss automatically after X seconds
- Modal: `cargo run --example modal`

Upcoming Customization Options:
- Dismiss: Click anywhere to dismiss, X close button, etc.
- Input: Allow for user input.
- Hover/Click: Color change on button/popup hover/click.
- Animations: Open/Close/Dismiss/Click/etc.

## Examples

This example will display a modal popup for 10s with a 'Close' button.

```rust,ignore
use bevy::prelude::*;
use bevy_text_popup::{TextPopupEvent, TextPopupPlugin, TextPopupTimeout, TextPopupButton};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextPopupPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut text_popup_events: MessageWriter<TextPopupEvent>) {
    commands.spawn(Camera2d::default());

    text_popup_events.send(TextPopupEvent {
        content: "Modal Example".to_string(),
        modal: Some(Color::linear_rgba(0., 0., 1., 0.75).into()),
        timeout: TextPopupTimeout::Seconds(10),
        dismiss_button: Some(TextPopupButton {
            text: "Close".to_string(),
            text_color: TextColor::from(Color::BLACK),
            background_color: Color::linear_rgb(1., 1., 1.).into(),
            ..Default::default()
        }),
        name: Some(Name::new("custom_popup_name")), // Name component will be added to entity.
        ..default()
    });
}
```

### Buttons

`cargo run --example buttons`

![Buttons Example](examples/buttons.png?raw=true "Buttons")

### Border

`cargo run --example border`

![Border](examples/border.png?raw=true "Border")

### Custom Font

`cargo run --example custom_font`

![Custom Font Example](examples/custom_font.png?raw=true "Custom Font")

### Locations

`cargo run --example locations`

![Locations](examples/locations.png?raw=true "Locations")

And example showing custom pixel coordinates:

`cargo run --example custom_locations`

### Custom Components

`cargo run --example custom_components`

### Modal

`cargo run --example modal`

![Modal](examples/modal.png?raw=true "Modal")

### Transparency

`cargo run --example transparency`

![Transparency](examples/transparency.png?raw=true "Transparency")

## Bevy Compatibility

|bevy|bevy_text_popup|
|---|---|
|0.17|0.7|
|0.16|0.6|
|0.15|0.5|
|0.14|0.4|
|0.13|0.3|
|0.12|0.2|
|0.11|0.1|

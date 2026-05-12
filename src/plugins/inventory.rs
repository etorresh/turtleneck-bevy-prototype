use crate::plugins::input::KeyBindings;
use bevy::prelude::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInventory>();
        app.add_systems(Update, toggle_inventory);
    }
}

#[derive(Resource)]
pub struct PlayerInventory {
    pub slots: Vec<Option<String>>,
}

impl Default for PlayerInventory {
    fn default() -> Self {
        Self {
            slots: vec![None; 12],
        }
    }
}

fn toggle_inventory(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    keybinds: Res<KeyBindings>,
    mut query: Query<Entity, With<InventoryRoot>>,
    inventory: Res<PlayerInventory>,
) {
    if keys.just_pressed(keybinds.open_inventory) {
        if let Ok(entity) = query.single_mut() {
            commands.entity(entity).despawn();
        } else {
            spawn_ui_layout(commands, &inventory);
        }
    }
}

#[derive(Component)]
struct InventoryRoot;

fn spawn_ui_layout(mut commands: Commands, inventory: &PlayerInventory) {
    commands
        .spawn((
            InventoryRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                // BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                BackgroundColor((Srgba::new(0.15, 0.15, 0.15, 0.999)).into()),
                Node {
                    width: Val::Px(700.0),
                    height: Val::Px(900.0),
                    flex_wrap: FlexWrap::Wrap,
                    padding: UiRect::all(Val::Px(15.0)),
                    align_content: AlignContent::FlexStart,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|panel| {
                for slot_data in &inventory.slots {
                    panel
                        .spawn((
                            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                            Node {
                                width: Val::Px(100.0),
                                height: Val::Px(100.0),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                        ))
                        .with_children(|slot| {
                            if let Some(item_name) = slot_data {
                                slot.spawn((
                                    BackgroundColor(Color::srgb(0.0, 0.4, 0.8)), // Stylized Blue
                                    Node {
                                        width: Val::Percent(90.0),
                                        height: Val::Percent(90.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                ))
                                .with_children(|item_box| {
                                    item_box.spawn((
                                        Text::new(item_name),
                                        TextFont {
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));
                                });
                            }
                        });
                }
            });
        });
}

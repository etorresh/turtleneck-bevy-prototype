use bevy::prelude::*;


pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeyBindings>();
    }
}


#[derive(Resource)]
pub struct KeyBindings {
    pub retract_to_shell: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            retract_to_shell: KeyCode::Space,
        }
    }
}


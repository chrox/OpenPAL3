use crate::directors::sce_vm::{SceCommand, SceState};

use imgui::Ui;
use radiance::scene::SceneManager;

#[derive(Debug, Clone)]
pub struct SceCommandScriptRunMode {
    mode: i32,
}

impl SceCommand for SceCommandScriptRunMode {
    fn update(
        &mut self,
        scene_manager: &mut dyn SceneManager,
        ui: &mut Ui,
        state: &mut SceState,
        delta_sec: f32,
    ) -> bool {
        state.set_run_mode(self.mode);
        return true;
    }
}

impl SceCommandScriptRunMode {
    pub fn new(mode: i32) -> Self {
        Self { mode }
    }
}

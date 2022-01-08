use crate::directors::sce_vm::{SceCommand, SceState};
use imgui::Ui;
use radiance::scene::SceneManager;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SceCommandShowChatRest {
    config_file: String,
    enough_money_proc: u32,
    not_enough_money_proc: u32,
    after_rest_proc: u32,
}

impl SceCommand for SceCommandShowChatRest {
    fn update(
        &mut self,
        scene_manager: &mut dyn SceneManager,
        ui: &mut Ui,
        state: &mut SceState,
        delta_sec: f32,
    ) -> bool {
        true
    }
}

impl SceCommandShowChatRest {
    pub fn new(
        config_file: String,
        enough_money_proc: u32,
        not_enough_money_proc: u32,
        after_rest_proc: u32,
    ) -> Self {
        log::debug!(
            "aaaaaaaaaaaaa: {} {} {} ",
            enough_money_proc,
            not_enough_money_proc,
            after_rest_proc
        );
        Self {
            config_file,
            enough_money_proc,
            not_enough_money_proc,
            after_rest_proc,
        }
    }
}

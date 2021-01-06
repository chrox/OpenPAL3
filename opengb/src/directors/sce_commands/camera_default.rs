use crate::directors::sce_director::SceCommand;
use crate::directors::sce_state::SceState;
use crate::directors::SceneManagerExtensions;
use imgui::Ui;
use radiance::{
    math::Vec3,
    scene::{Scene, SceneManager},
};

#[derive(Clone)]
pub struct SceCommandCameraDefault {}

impl SceCommand for SceCommandCameraDefault {
    fn update(
        &mut self,
        scene_manager: &mut dyn SceneManager,
        ui: &mut Ui,
        state: &mut SceState,
        delta_sec: f32,
    ) -> bool {
        let target = Vec3::new(0., 0., 0.);
        scene_manager
            .scene_mut_or_fail()
            .camera_mut()
            .transform_mut()
            .set_position(&Vec3::new(300., 300., 300.))
            .look_at(&target);
        return true;
    }
}

impl SceCommandCameraDefault {
    pub fn new(unknown: i32) -> Self {
        Self {}
    }
}
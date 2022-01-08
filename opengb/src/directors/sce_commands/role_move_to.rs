use crate::directors::sce_vm::{SceCommand, SceState};

use crate::directors::SceneManagerExtensions;
use imgui::Ui;
use radiance::scene::Entity;
use radiance::{math::Vec3, scene::SceneManager};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SceCommandRoleMoveTo {
    role_id: i32,
    nav_x: f32,
    nav_z: f32,
    unknown: i32,
}

impl SceCommand for SceCommandRoleMoveTo {
    fn initialize(&mut self, scene_manager: &mut dyn SceneManager, state: &mut SceState) {
        scene_manager
            .get_resolved_role_mut(state, self.role_id)
            .and_then(|r| Some(r.run()));
    }

    fn update(
        &mut self,
        scene_manager: &mut dyn SceneManager,
        ui: &mut Ui,
        state: &mut SceState,
        delta_sec: f32,
    ) -> bool {
        const SPEED: f32 = 175.;

        let role = scene_manager
            .get_resolved_role(state, self.role_id)
            .unwrap();
        let to = {
            let scene = scene_manager.core_scene_or_fail();
            scene.nav_coord_to_scene_coord(role.nav_layer(), self.nav_x, self.nav_z)
        };

        let role = scene_manager
            .get_resolved_role_mut(state, self.role_id)
            .unwrap();
        let position = role.transform().position();
        let step = SPEED * delta_sec;
        let remain = Vec3::sub(&to, &position);
        let completed = remain.norm() < step;
        let new_position = if completed {
            to
        } else {
            Vec3::add(&position, &Vec3::dot(step, &Vec3::normalized(&remain)))
        };

        role.transform_mut()
            .look_at(&Vec3::new(to.x, position.y, to.z))
            .set_position(&new_position);

        if completed {
            role.idle();
        }

        completed
    }
}

impl SceCommandRoleMoveTo {
    pub fn new(role_id: i32, nav_x: i32, nav_z: i32, unknown: i32) -> Self {
        Self {
            role_id,
            nav_x: nav_x as f32,
            nav_z: nav_z as f32,
            unknown,
        }
    }
}

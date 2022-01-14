use crate::directors::sce_vm::{SceCommand, SceState};
use imgui::{Condition, MouseButton, Ui, Window};
use radiance::{input::Key, scene::SceneManager};

#[derive(Debug, Clone)]
pub struct SceCommandDlg {
    text: String,
    dlg_end: bool,
    adv_input_enabled: bool,
}

impl SceCommandDlg {
    const DLG_HEIGHT_FACTOR: f32 = 0.25;
    const DLG_Y_POSITION_FACTOR: f32 = 1. - SceCommandDlg::DLG_HEIGHT_FACTOR;
}

impl SceCommand for SceCommandDlg {
    fn initialize(&mut self, scene_manager: &mut dyn SceneManager, state: &mut SceState) {
        self.adv_input_enabled = state.global_state_mut().adv_input_enabled();
        state.global_state_mut().set_adv_input_enabled(false);
    }

    fn update(
        &mut self,
        scene_manager: &mut dyn SceneManager,
        ui: &mut Ui,
        state: &mut SceState,
        delta_sec: f32,
    ) -> bool {
        if self.dlg_end {
            // state.global_state_mut().set_adv_input_enabled(self.adv_input_enabled);
            return true;
        }

        let [window_width, window_height] = ui.io().display_size;
        let (dialog_x, dialog_width) = {
            if window_width / window_height > 4. / 3. {
                let dialog_width = window_height / 3. * 4.;
                let dialog_x = (window_width - dialog_width) / 2.;
                (dialog_x, dialog_width)
            } else {
                (0., window_width)
            }
        };

        let w = Window::new(" ")
            .collapsible(false)
            .title_bar(false)
            .resizable(false)
            .size(
                [
                    dialog_width,
                    window_height * SceCommandDlg::DLG_HEIGHT_FACTOR,
                ],
                Condition::Appearing,
            )
            .position(
                [
                    dialog_x,
                    window_height * SceCommandDlg::DLG_Y_POSITION_FACTOR,
                ],
                Condition::Appearing,
            );
        w.build(ui, || {
            ui.text_wrapped(&format!("{}", self.text));
        });

        // delay set_adv_input to the next frame so that the last kay pressed
        // won't trigger the sce proc again.
        self.dlg_end = state.input().get_key_state(Key::Space).pressed()
            || state.input().get_key_state(Key::GamePadEast).pressed()
            || state.input().get_key_state(Key::GamePadSouth).pressed()
            || ui.is_mouse_released(MouseButton::Left);

        false
    }
}

impl SceCommandDlg {
    pub fn new(text: String) -> Self {
        Self {
            text: text.replace("\\n", "\n"),
            dlg_end: false,
            adv_input_enabled: false,
        }
    }
}

use amethyst::{core::*, derive::SystemDesc, ecs::prelude::*, ui::*, utils::fps_counter::*};

#[derive(Default, SystemDesc)]
pub struct FpsSystem {
    fps_display: Option<Entity>,
}

impl<'a> System<'a> for FpsSystem {
    type SystemData = (
        WriteStorage<'a, UiText>,
        Read<'a, Time>,
        Read<'a, FpsCounter>,
        UiFinder<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut ui_text, time, fps_counter, finder) = data;

        if self.fps_display.is_none() {
            if let Some(fps_entity) = finder.find("fps_text") {
                self.fps_display = Some(fps_entity);
            }
        }
        if let Some(fps_entity) = self.fps_display {
            if let Some(fps_display) = ui_text.get_mut(fps_entity) {
                if time.frame_number() % 20 == 0 {
                    let fps = fps_counter.sampled_fps();
                    fps_display.text = format!("FPS: {:.*}", 2, fps);
                }
            }
        }
    }
}

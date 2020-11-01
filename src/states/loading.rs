use amethyst::{
    assets::{
        Completion, Handle, HotReloadBundle, Prefab, PrefabLoader, PrefabLoaderSystemDesc,
        ProgressCounter, RonFormat,
    },
    prelude::*,
    ui::{RenderUi, UiBundle, UiCreator, UiFinder, UiText},
};

use crate::composite::BasicScene;

#[derive(Default)]
pub struct Loading {
    progress: ProgressCounter,
}

impl SimpleState for Loading {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/fps.ron", &mut self.progress);
            creator.create("ui/loading.ron", &mut self.progress);
        });
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        match self.progress.complete() {
            Completion::Failed => {
                println!("Failed loading assets: {:?}", self.progress.errors());
                Trans::Quit
            }
            Completion::Complete => {
                println!("Assets loaded, swapping state");
                if let Some(entity) = data
                    .world
                    .exec(|finder: UiFinder<'_>| finder.find("loading"))
                {
                    let _ = data.world.delete_entity(entity);
                }
                Trans::Switch(Box::new(BasicScene::default()))
            }
            Completion::Loading => Trans::None,
        }
    }
}

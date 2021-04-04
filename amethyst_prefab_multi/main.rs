//! Demonstrates loading a custom prefab using the Amethyst engine.

use std::fmt::Debug;

use amethyst::{
    prelude::*,
    Error,
    assets::{
        Handle,
        DefaultLoader,
        LoaderBundle,
        Loader,
        prefab::{register_component_type,
                 serde_diff,
                 Prefab,
                 SerdeDiff,
                 legion_prefab,
                },
        // Loader,
        ProgressCounter,
    },
    core::Named,
    core::transform::components::Parent,
    utils::application_root_dir,
    ecs::{
        World,
    },
};

use derive_new::new;
use serde::{Deserialize, Serialize};
use type_uuid::TypeUuid;

#[derive(TypeUuid, Serialize, Deserialize, SerdeDiff, Clone, Default, Debug)]
#[serde(deny_unknown_fields)]
#[uuid = "f5780013-bae4-49f0-ac0e-a108ff52fd30"]
pub struct Position(pub f32, pub f32, pub f32);
register_component_type!(Position);

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Player {
    player: Named,
    position: Position,
}

#[derive(new)]
pub struct CustomPrefabState {
    /// Tracks loaded assets.
    #[new(default)]
    pub progress_counter: ProgressCounter,
    /// Handle to the loaded prefab.
    #[new(default)]
    pub prefab_handle: Option<Handle<Prefab<>>>,
}

impl SimpleState for CustomPrefabState {
    fn on_start(&mut self, data: StateData<'_, GameData>) {

        let loader = data.resources.get_mut::<DefaultLoader>().unwrap();
        let prefab_handle: Handle<Prefab> = loader.load("prefab/prefab_multi.ron");
        self.prefab_handle = Some(prefab_handle.clone());
        data.world.push((prefab_handle,));

        // Create one set of entities from the prefab.
        (0..1).for_each(|_| {
            data.world
                .push((prefab_handle.clone(),));
        });

        self.prefab_handle = Some(prefab_handle);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            self.display_loaded_prefab(&data.world);
            self.display_loaded_entities(&mut data.world);
            Trans::Quit
        } else {
            Trans::None
        }
    }
}

impl CustomPrefabState {
    // Displays the contents of the loaded prefab.
    fn display_loaded_prefab(&self, world: &World) {
        if let Some(handle) = self.prefab_handle.as_ref() {
            let prefab = handle
                .asset(Player)
                .expect("Expected prefab to be loaded.");

            println!("Prefab");
            println!("======");
            prefab
                .entities()
                .for_each(|entity| println!("{:?}", entity));
            println!();
        }
    }

    // Displays the `Component`s of entities in the `World`.
    fn display_loaded_entities(&self, world: &mut World) {
        println!("Entities");
        println!("========");
        println!();
        println!(
            "| {e:24} | {prefab_handle:22} | {parent:6} | {pos:23} | {named:22} |",
            e = "Entity",
            prefab_handle = "Handle<Prefab<Player>>",
            parent = "Parent",
            pos = "Position",
            named = "Player",
        );
        println!(
            "| {c:-^24} | {c:-^22} | {c:-^6} | {c:-^23} | {c:-^22} |",
            c = "",
        );
        world.exec(
            |(entities, prefab_handles, parents, positions, nameds): (
                Entities,
                ReadStorage<Handle<Prefab<Player>>>,
                ReadStorage<Parent>,
                ReadStorage<Position>,
                ReadStorage<Named>,
            )| {
                (
                    &entities,
                    prefab_handles.maybe(),
                    parents.maybe(),
                    positions.maybe(),
                    nameds.maybe(),
                )
                    .join()
                    .for_each(|(e, prefab_handle, parent, pos, named)| {
                        println!(
                            "| {e:24} | {prefab_handle:22} | {parent:6} | {pos:23} | {named:22} |",
                            e = format!("{:?}", e),
                            prefab_handle = Self::display(prefab_handle),
                            parent = Self::display(parent.map(|p| p.entity)),
                            pos = Self::display(pos),
                            named = Self::display(named),
                        )
                    });
            },
        )
    }

    fn display<T: Debug>(component: Option<T>) -> String {
        if let Some(component) = component {
            format!("{:?}", component)
        } else {
            format!("{:?}", component)
        }
    }
}

/// Wrapper around the main, so we can return errors easily.
fn main() -> Result<(), Error> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    // Add our meshes directory to the asset loader.
    let assets_dir = app_root.join("assets");
    let mut dispatcher_builder = DispatcherBuilder::default();

    dispatcher_builder
        .add_bundle(LoaderBundle);

    let game = Application::new(
        assets_dir,
        CustomPrefabState::new(),
        dispatcher_builder,
    )?;

    game.run();
    Ok(())
}

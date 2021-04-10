use amethyst::{prelude::*,
    utils::application_root_dir,
    SimpleState,
    GameDataBuilder,
    Application};

struct GameState;
impl SimpleState for GameState {
    fn on_start (&mut self, _data: StateData<'_, GameData>){}
}

fn main() -> amethyst::Result<()> {
    // create logger
    amethyst::start_logger(Default::default());

    // set up assets directory
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    // build GameData
    let game_data = GameDataBuilder::default();

    // build Application
    let mut game = Application::new(assets_dir, GameState, game_data)?;

    println!("Hello, world!");

    Ok(())
}

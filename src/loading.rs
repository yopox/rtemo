use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading)
                .with_collection::<Textures>()
                .continue_to_state(AppState::Editor),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct Textures {
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 9, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "icons.png")]
    pub icons: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 8., tile_size_y = 8., columns = 32, rows = 32, padding_x = 0., padding_y = 0.))]
    #[asset(path = "../../imperatrices/assets/MRMOTEXT EX.png")]
    pub mrmotext: Handle<TextureAtlas>,

    #[asset(path = "slot.png")]
    pub slot: Handle<Image>,
}
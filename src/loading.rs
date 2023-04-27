use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(AppState::Loading)
                    .continue_to_state(AppState::Editor),
            )
            .add_collection_to_loading_state::<_, Textures>(AppState::Loading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct Textures {
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 13, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "icons.png")]
    pub icons: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 8., tile_size_y = 8., columns = 32, rows = 32, padding_x = 0., padding_y = 0.))]
    #[asset(path = "MRMOTEXT EX.png")]
    pub mrmotext: Handle<TextureAtlas>,

    #[asset(path = "slot.png")]
    pub slot: Handle<Image>,

    #[asset(path = "color.png")]
    pub color: Handle<Image>,
}
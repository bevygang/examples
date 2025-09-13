use bevy::{asset::{Asset, LoadedUntypedAsset}, prelude::*, platform::collections::HashMap};

pub type LoadedAssets = Assets<LoadedUntypedAsset>;
pub type AssetResource<'w> = Res<'w, LoadedAssets>;

#[derive(Resource, Clone)]
pub struct AssetStore {
    pub(crate) asset_index: HashMap<String, Handle<LoadedUntypedAsset>>,
}

impl AssetStore {
    pub fn new() -> Self {
        Self {
            asset_index: HashMap::new(),
        }
    }

    pub fn get_handle<T>(&self, index: &str, assets: &LoadedAssets) -> Option<Handle<T>> where T: Asset {
        if let Some(handle_untyped) = self.asset_index.get(index)  {
            if let Some(handle) = assets.get(handle_untyped) {
                return Some(handle.handle.clone().typed::<T>());
            }
        }
        None
    }
}

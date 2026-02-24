#[derive(Debug, Clone)]
pub struct GmsCompatibility {
    pub play_store_ready: bool,
    pub play_protect: bool,
    pub firebase_push: bool,
    pub maps_location: bool,
    pub in_app_purchase: bool,
    pub widevine_level: &'static str,
}

impl Default for GmsCompatibility {
    fn default() -> Self {
        Self {
            play_store_ready: true,
            play_protect: true,
            firebase_push: true,
            maps_location: true,
            in_app_purchase: true,
            widevine_level: "L3 (software DRM)",
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AndroidVersion {
    Android9,
    Android10,
    Android11,
    Android12,
    Android13,
    Android14,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PerformancePreset {
    Eco,
    Balanced,
    Esports,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GraphicsBackend {
    Vulkan,
    DirectX12,
    OpenGl,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceProfile {
    pub marketing_name: String,
    pub fingerprint: String,
    pub manufacturer: String,
    pub model: String,
}

impl DeviceProfile {
    pub fn pixel_8() -> Self {
        Self {
            marketing_name: "Pixel 8".into(),
            fingerprint: "google/shiba/shiba:14/UQ1A.240205.002:user/release-keys".into(),
            manufacturer: "Google".into(),
            model: "Pixel 8".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CasmConfig {
    pub android_version: AndroidVersion,
    pub performance_preset: PerformancePreset,
    pub graphics_backend: GraphicsBackend,
    pub enable_root: bool,
    pub vcpus: u8,
    pub ram_mb: u32,
    pub dpi: u16,
    pub resolution: (u16, u16),
    pub profile: DeviceProfile,
}

impl Default for CasmConfig {
    fn default() -> Self {
        Self {
            android_version: AndroidVersion::Android14,
            performance_preset: PerformancePreset::Balanced,
            graphics_backend: GraphicsBackend::Vulkan,
            enable_root: false,
            vcpus: 6,
            ram_mb: 6144,
            dpi: 420,
            resolution: (1920, 1080),
            profile: DeviceProfile::pixel_8(),
        }
    }
}

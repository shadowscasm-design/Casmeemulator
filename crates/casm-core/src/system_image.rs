use crate::config::{AndroidDistribution, AndroidVersion, CasmConfig};

#[derive(Debug, Clone)]
pub struct SystemImagePlan {
    pub id: String,
    pub distribution: AndroidDistribution,
    pub android_version: AndroidVersion,
    pub root_supported: bool,
    pub play_services_ready: bool,
}

pub fn resolve_system_image(config: &CasmConfig) -> SystemImagePlan {
    let id = match (config.distribution, config.android_version) {
        (AndroidDistribution::CrDroid, AndroidVersion::Android14) => "crdroid-10-android14-x86_64",
        (AndroidDistribution::CrDroid, AndroidVersion::Android13) => "crdroid-9-android13-x86_64",
        (AndroidDistribution::CrDroid, _) => "crdroid-legacy-x86_64",
        (AndroidDistribution::Aosp, AndroidVersion::Android14) => "aosp-android14-generic-x86_64",
        (AndroidDistribution::Aosp, _) => "aosp-generic-x86_64",
    }
    .to_string();

    SystemImagePlan {
        id,
        distribution: config.distribution,
        android_version: config.android_version,
        root_supported: true,
        play_services_ready: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AndroidDistribution, CasmConfig};

    #[test]
    fn selects_crdroid_android14_image_by_default() {
        let cfg = CasmConfig::default();
        let image = resolve_system_image(&cfg);
        assert_eq!(image.id, "crdroid-10-android14-x86_64");
    }

    #[test]
    fn can_fallback_to_aosp() {
        let mut cfg = CasmConfig::default();
        cfg.distribution = AndroidDistribution::Aosp;
        let image = resolve_system_image(&cfg);
        assert_eq!(image.id, "aosp-android14-generic-x86_64");
    }
}

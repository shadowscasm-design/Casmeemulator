use casm_core::{CasmConfig, CasmEngine};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .without_time()
        .init();

    let config = CasmConfig::default();
    let engine = CasmEngine::new(config.clone());
    let report = engine.boot().await?;

    info!("CasmEmulator boot sequence completed");
    println!("=== CasmEmulator Prototype ===");
    println!("Distribution: {:?}", config.distribution);
    println!("Android: {:?}", config.android_version);
    println!("Graphics: {:?}", config.graphics_backend);
    println!("System image: {}", report.system_image.id);
    println!("Estimated boot: {} ms", report.boot_time_ms);
    println!("Gaming FPS target: {}", report.gaming.max_fps_target);
    println!("GMS ready: {}", report.gms.play_store_ready);
    println!("Sandbox enabled: {}", report.security.sandbox_mode);

    Ok(())
}

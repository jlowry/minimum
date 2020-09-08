fn main() {
    // Setup logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    // Spawn the daemon in a background thread. This could be a different process, but
    // for simplicity we'll launch it here.
    std::thread::spawn(move || {
        minimum::daemon::create_default_asset_daemon()
            .with_importer("prefab", minimum::pipeline::PrefabImporter::default())
            .run();
    });

    example_sdl2::run();
}

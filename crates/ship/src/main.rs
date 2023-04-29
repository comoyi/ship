use ship_gui::GuiFlags;
use ship_internal::application::app::AppManager;
use ship_internal::application::settings::SettingsManager;
use ship_internal::application::update::update_manage::UpdateManager;
use ship_internal::version::version_manage::VersionManager;
use ship_internal::App;
use std::sync::{Arc, Mutex};

fn main() {
    let version_manager = VersionManager::new();
    let version_manager_ptr = Arc::new(Mutex::new(version_manager));
    let version_manager_ptr_1 = Arc::clone(&version_manager_ptr);
    let version_manager_ptr_2 = Arc::clone(&version_manager_ptr);

    let settings_manager = SettingsManager::default();
    let settings_manager_ptr = Arc::new(Mutex::new(settings_manager));
    let settings_manager_ptr_1 = Arc::clone(&settings_manager_ptr);
    let settings_manager_ptr_2 = Arc::clone(&settings_manager_ptr);

    let app_manager = AppManager::default();
    let app_manager_ptr = Arc::new(Mutex::new(app_manager));
    let app_manager_ptr_1 = Arc::clone(&app_manager_ptr);
    let app_manager_ptr_2 = Arc::clone(&app_manager_ptr);

    let update_manager = UpdateManager::default();
    let update_manager_ptr = Arc::new(Mutex::new(update_manager));
    let update_manager_ptr_1 = Arc::clone(&update_manager_ptr);
    let update_manager_ptr_2 = Arc::clone(&update_manager_ptr);

    App::new(
        version_manager_ptr_1,
        settings_manager_ptr_1,
        app_manager_ptr_1,
        update_manager_ptr_1,
    )
    .run();
    ship_gui::start(GuiFlags::new(
        version_manager_ptr_2,
        settings_manager_ptr_2,
        app_manager_ptr_2,
        update_manager_ptr_2,
    ));
}

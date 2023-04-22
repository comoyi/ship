use ship_gui::GuiFlags;
use ship_internal::App;

fn main() {
    App::new().run();
    ship_gui::start(GuiFlags::new());
}

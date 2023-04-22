use ship_gui::GuiFlags;

#[derive(Default)]
pub struct App {}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&self) {
        ship_gui::start(GuiFlags::new());
    }
}

#[derive(Default, Debug)]
pub enum LaunchStatus {
    #[default]
    Idle,
    Starting,
    // Failed,
    Started,
}

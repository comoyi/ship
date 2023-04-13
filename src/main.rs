mod app;
mod cache;
mod config;
mod data;
mod error;
mod gui;
mod i18n;
mod log;
mod requests;
mod scan;
mod sync;
mod utils;
mod version;

fn main() {
    app::start();
}

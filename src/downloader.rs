use crate::info::InfoManager;
use log::info;
use std::sync::{Arc, Mutex};

pub struct Downloader {}

impl Downloader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&self, im1: Arc<Mutex<InfoManager>>) {
        info!("start downloader");
        let im_guard = im1.lock().unwrap();
        im_guard.add("start downloader.");
        drop(im_guard);
    }
}

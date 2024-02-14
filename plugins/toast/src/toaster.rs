use queue::Queue;
use std::{collections::HashMap, sync::Mutex, time::Instant};

#[cfg(target_os = "macos")]
use crate::macos::toast::Toast;

pub struct Store {
    toasts: HashMap<String, Toast>,
    queue: Queue<String>,
    throttle: Instant,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            toasts: Default::default(),
            queue: Default::default(),
            throttle: Instant::now(),
        }
    }
}

#[derive(Default)]
pub struct Toaster(pub Mutex<Store>);

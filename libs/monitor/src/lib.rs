use core_graphics::display::CGDirectDisplayID;
use serde::{Deserialize, Serialize};
use tauri::{PhysicalPosition, PhysicalSize};

#[cfg(target_os = "macos")]
mod macos;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Monitor {
    id: CGDirectDisplayID,
    uuid: Option<String>,
    name: Option<String>,
    size: PhysicalSize<u32>,
    position: PhysicalPosition<i32>,
    scale_factor: f64,
    has_cursor: bool,
    is_primary: bool,
}

impl Monitor {
    pub fn id(&self) -> CGDirectDisplayID {
        self.id
    }

    pub fn uuid(&self) -> Option<&String> {
        self.uuid.as_ref()
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn position(&self) -> PhysicalPosition<i32> {
        self.position
    }

    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    pub fn has_cursor(&self) -> bool {
        self.has_cursor
    }

    pub fn is_primary(&self) -> bool {
        self.is_primary
    }
}

pub fn get_monitor_with_cursor() -> Option<Monitor> {
    #[cfg(target_os = "macos")]
    return macos::monitor::get_monitor_with_cursor();
}

pub fn get_monitors() -> Vec<Monitor> {
    #[cfg(target_os = "macos")]
    return macos::monitor::get_monitors();
}

use serde::{Deserialize, Serialize};
use tauri::{PhysicalPosition, PhysicalSize};

#[cfg(target_os = "macos")]
mod macos;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisibleArea {
    size: PhysicalSize<f64>,
    position: PhysicalPosition<f64>,
}

impl VisibleArea {
    pub fn size(&self) -> PhysicalSize<f64> {
        self.size
    }

    pub fn position(&self) -> PhysicalPosition<f64> {
        self.position
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Monitor {
    id: u32,
    uuid: Option<String>,
    name: Option<String>,
    size: PhysicalSize<f64>,
    position: PhysicalPosition<f64>,
    scale_factor: f64,
    has_cursor: bool,
    is_primary: bool,
    visible_area: VisibleArea,
}

impl Monitor {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn uuid(&self) -> Option<&String> {
        self.uuid.as_ref()
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn size(&self) -> PhysicalSize<f64> {
        self.size
    }

    pub fn visible_area(&self) -> VisibleArea {
        self.visible_area.clone()
    }

    pub fn position(&self) -> PhysicalPosition<f64> {
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
    #[cfg(target_os = "windows")]
    {
        unimplemented!()
    }

    #[cfg(target_os = "linux")]
    {
        unimplemented!()
    }

    #[cfg(target_os = "macos")]
    {
        return macos::monitor::get_monitor_with_cursor();
    }
}

pub fn get_monitors() -> Vec<Monitor> {
    #[cfg(target_os = "windows")]
    {
        unimplemented!()
    }

    #[cfg(target_os = "linux")]
    {
        unimplemented!()
    }

    #[cfg(target_os = "macos")]
    {
        return macos::monitor::get_monitors();
    }
}

use serde::{Deserialize, Serialize};

#[cfg(target_os = "macos")]
mod macos;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Menubar {
    scale_factor: f64,
    height: f64,
}

impl Menubar {
    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }
}

/// Get the menubar info of the current monitor
pub fn get_menubar() -> Option<Menubar> {
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
        return macos::menubar::get_menubar();
    }
}

use serde::{Deserialize, Serialize};

#[cfg(target_os = "macos")]
mod macos;

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Menubar {
    height: f64,
}

impl Menubar {
    pub fn height(&self) -> f64 {
        #[cfg(target_os = "windows")]
        {
            unimplemented!()
        }

        #[cfg(target_os = "linux")]
        {
            unimplemented!()
        }

        #[cfg(target_os = "macos")]
        macos::menubar::get_height()
    }
}

/// Get info about the system-wide Menubar
pub fn get_menubar() -> Menubar {
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
        return Menubar::default();
    }
}

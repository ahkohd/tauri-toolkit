#![cfg(test)]

use crate::macos::menubar;

#[test]
fn it_get_menubar_for_monitor() {
    let menubar_height = menubar::get_height();

    assert!(menubar_height >= 22.0);
}

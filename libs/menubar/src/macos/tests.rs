#![cfg(test)]

use crate::macos::menubar::get_menubar;

#[test]
fn it_get_menubar_for_monitor() {
    let menubar = get_menubar();

    assert!(menubar.is_some());

    let menubar = menubar.unwrap();

    assert!(menubar.height() > 0.0);

    assert!(menubar.scale_factor() > 0.0);
}

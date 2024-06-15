#![cfg(test)]

use crate::macos::menubar;

#[test]
fn it_gets_the_menubar_height() {
    let menubar_height = menubar::get_height();

    assert!(menubar_height >= 22.0);
}

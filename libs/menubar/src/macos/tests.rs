#![cfg(test)]

#[test]
fn it_gets_the_menubar_height() {
    let menubar_height = super::menubar::get_height();

    assert!(menubar_height >= 22.0);
}

#![cfg(test)]

use crate::get_monitor_with_cursor;

use super::monitor::get_monitors;

#[test]
fn it_gets_monitor_with_cursor() {
    let monitor = get_monitor_with_cursor();

    assert!(monitor.is_some());

    let monitor = monitor.unwrap();

    assert!(monitor.id() > 0);
}

#[test]
fn it_gets_all_monitors() {
    let monitors = get_monitors();

    assert!(!monitors.is_empty());
}

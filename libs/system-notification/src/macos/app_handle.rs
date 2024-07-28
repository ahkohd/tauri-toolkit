use std::ffi::CString;

use block::ConcreteBlock;
use cocoa::base::{id, nil};
use objc::{class, msg_send, sel, sel_impl};
use tauri::{AppHandle, Runtime};

pub trait WorkspaceListener<R: Runtime> {
    fn register_application_notification_listener(name: &str, callback: Box<dyn Fn()>);

    fn register_workspace_notification_listener(name: &str, callback: Box<dyn Fn()>);

    fn listen_notification(&self, name: &str, callback: fn(AppHandle<R>));

    fn listen_workspace(&self, name: &str, callback: fn(AppHandle<R>));
}

impl<R: Runtime> WorkspaceListener<R> for AppHandle<R> {
    fn register_application_notification_listener(name: &str, callback: Box<dyn Fn()>) {
        let notification_center: id =
            unsafe { msg_send![class!(NSNotificationCenter), defaultCenter] };

        let block = ConcreteBlock::new(move |_notif: id| {
            callback();
        });

        let block = block.copy();

        let name: id =
            unsafe { msg_send![class!(NSString), stringWithCString: CString::new(name).unwrap()] };

        unsafe {
            let _: () = msg_send![
                notification_center,
                addObserverForName: name object: nil queue: nil usingBlock: block
            ];
        }
    }

    fn listen_notification(&self, name: &str, callback: fn(AppHandle<R>)) {
        let app_handle = self.clone();

        AppHandle::<R>::register_application_notification_listener(
            name,
            Box::new(move || callback(app_handle.clone())),
        );
    }

    fn register_workspace_notification_listener(name: &str, callback: Box<dyn Fn()>) {
        let workspace: id = unsafe { msg_send![class!(NSWorkspace), sharedWorkspace] };

        let notification_center: id = unsafe { msg_send![workspace, notificationCenter] };

        let block = ConcreteBlock::new(move |_notif: id| {
            callback();
        });

        let block = block.copy();

        let name: id =
            unsafe { msg_send![class!(NSString), stringWithCString: CString::new(name).unwrap()] };

        unsafe {
            let _: () = msg_send![
                notification_center,
                addObserverForName: name object: nil queue: nil usingBlock: block
            ];
        }
    }

    fn listen_workspace(&self, name: &str, callback: fn(AppHandle<R>)) {
        let app_handle = self.clone();

        AppHandle::<R>::register_workspace_notification_listener(
            name,
            Box::new(move || callback(app_handle.clone())),
        );
    }
}

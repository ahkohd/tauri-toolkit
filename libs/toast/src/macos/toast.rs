extern crate markdown;

use std::{
    ptr::NonNull,
    sync::{Arc, Mutex},
};

use block2::RcBlock;
use monitor::get_monitor_with_cursor;
use objc2::{
    define_class, extern_methods, msg_send, rc::Retained, AllocAnyThread, MainThreadOnly, Message,
};
use objc2_app_kit::{
    NSAnimatablePropertyContainer, NSAnimationContext, NSAutoresizingMaskOptions,
    NSBackingStoreType, NSColor, NSLineBreakMode, NSMutableParagraphStyle, NSPanel, NSTextField,
    NSVisualEffectBlendingMode, NSVisualEffectMaterial, NSVisualEffectState, NSVisualEffectView,
    NSWindowCollectionBehavior, NSWindowOrderingMode, NSWindowStyleMask,
};
use objc2_foundation::{
    MainThreadMarker, NSAttributedString, NSAttributedStringKey, NSMutableAttributedString,
    NSObjectProtocol, NSPoint as CGPoint, NSRange, NSRect, NSSize as CGSize, NSString, NSTimer,
    NSUTF8StringEncoding,
};
use tauri::{AppHandle, Manager, Runtime};

#[derive(Clone)]
struct Ivars;

define_class!(
    #[unsafe(super = NSPanel)]
    #[name = "ToastPanel"]
    #[thread_kind = MainThreadOnly]
    #[ivars = Ivars]
    struct ToastPanel;

    unsafe impl NSObjectProtocol for ToastPanel {}

    unsafe impl NSAnimatablePropertyContainer for ToastPanel {}

    impl ToastPanel {
      #[unsafe(method(canBecomeKeyWindow))]
      fn __can_become_key_window() -> bool {
         false
      }

      #[unsafe(method(show:))]
      fn __show(&self, fade_duration: f64) {
          unsafe {
            self.setAlphaValue(0.0);

            self.orderFrontRegardless();

            NSAnimationContext::currentContext().setDuration(fade_duration);

            self.animator().setAlphaValue(1.0);
            };
        }

        #[unsafe(method(hide:))]
        fn __hide(&self, fade_duration: f64) {
          unsafe { self.setAlphaValue(1.0) };

          let group: Box<dyn Fn(NonNull<NSAnimationContext>)> = Box::new(|ctx| {
              let context = unsafe { ctx.as_ref() };

              unsafe { context.setDuration(fade_duration) };

              let panel = self.retain();

              let cb: Box<dyn Fn()> = Box::new(move ||{
                  panel.orderOut(None);
              });

              let cb = RcBlock::new(cb);

              unsafe { context.setCompletionHandler(Some(&cb)) };

              unsafe { self.animator().setAlphaValue(0.0) };
          });

          let group = RcBlock::new(group);

          unsafe {
              NSAnimationContext::runAnimationGroup(&group)
          };
        }


    }
);

impl ToastPanel {
    extern_methods!(
        #[unsafe(method(new))]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[allow(non_snake_case)]
        #[unsafe(method(setFrame:display:))]
        pub fn setFrame_display(&self, frame_rect: NSRect, flag: bool);
    );
}

impl ToastPanel {
    fn default() -> Retained<ToastPanel> {
        let mtm = MainThreadMarker::new().unwrap();

        let panel = unsafe { ToastPanel::new(mtm) };

        panel.setFrame_display(NSRect::ZERO, false);

        panel.setStyleMask(NSWindowStyleMask::NonactivatingPanel | NSWindowStyleMask::Borderless);

        unsafe { panel.setBackingType(NSBackingStoreType::Buffered) };

        unsafe {
            panel.setFloatingPanel(true);

            panel.setHidesOnDeactivate(false);
        };

        #[allow(non_snake_case)]
        let NSModalPanelWindowLevel = 8;

        panel.setLevel(NSModalPanelWindowLevel);

        unsafe {
            panel.setBackgroundColor(Some(&NSColor::clearColor()));
        };

        panel.setOpaque(false);

        unsafe { panel.setAlphaValue(0.0) };

        let content_view = panel.contentView().unwrap();

        let bounds = content_view.bounds();

        unsafe {
            panel.setCollectionBehavior(
                NSWindowCollectionBehavior::CanJoinAllSpaces
                    | NSWindowCollectionBehavior::Stationary,
            )
        };

        let visual_effect_view = unsafe { NSVisualEffectView::new(mtm) };

        unsafe {
            visual_effect_view.setFrame(bounds);

            visual_effect_view.setBlendingMode(NSVisualEffectBlendingMode::BehindWindow);

            visual_effect_view.setState(NSVisualEffectState::Active);

            visual_effect_view.setMaterial(NSVisualEffectMaterial::Popover);

            visual_effect_view.setAutoresizingMask(
                NSAutoresizingMaskOptions::ViewWidthSizable
                    | NSAutoresizingMaskOptions::ViewHeightSizable,
            );

            content_view.setWantsLayer(true);

            content_view.addSubview_positioned_relativeTo(
                &visual_effect_view,
                NSWindowOrderingMode::Below,
                None,
            );
        }

        panel.orderOut(None);

        panel
    }

    fn show(&self, fade_duration: f64) {
        let () = unsafe { msg_send![self, show: fade_duration] };
    }

    fn hide(&self, fade_duration: f64) {
        let () = unsafe { msg_send![self, hide: fade_duration] };
    }
}

struct Panel(Retained<ToastPanel>);

unsafe impl Sync for Panel {}

unsafe impl Send for Panel {}

struct ToastTimer(Retained<NSTimer>);

unsafe impl Sync for ToastTimer {}

unsafe impl Send for ToastTimer {}

#[derive(Debug, Clone)]
pub enum ToastPosition {
    #[allow(dead_code)]
    Top,
    Bottom,
    At(f64, f64),
}

#[derive(Debug, Clone)]
pub struct ToastConfig {
    pub font_size: f64,
    pub padding: (f64, f64),
    pub position: ToastPosition,
    pub offset: f64,
    pub duration: f64,
    pub fade_duration: f64,
    pub shadow: bool,
}

impl Default for ToastConfig {
    fn default() -> Self {
        ToastConfig {
            font_size: 16.0,
            padding: (15.0, 10.0),
            position: ToastPosition::Bottom,
            offset: 150.0,
            duration: 3.0,
            fade_duration: 0.3,
            shadow: false,
        }
    }
}

pub struct Toast {
    panel: Panel,
    timer: Arc<Mutex<Option<ToastTimer>>>,
    options: ToastConfig,
}

impl Default for Toast {
    fn default() -> Self {
        Self {
            panel: Panel(ToastPanel::default()),
            timer: Arc::new(Mutex::new(None)),
            options: Default::default(),
        }
    }
}

impl Toast {
    pub fn new(options: ToastConfig) -> Self {
        Self {
            panel: Panel(ToastPanel::default()),
            options,
            ..Default::default()
        }
    }

    fn message(
        &self,
        message: &str,
        options: Option<&ToastConfig>,
    ) -> Retained<NSMutableAttributedString> {
        let font_size = {
            if let Some(opt) = options {
                opt.font_size
            } else {
                self.options.font_size
            }
        };

        let html = format!(
          "<html><head><meta charset=\"utf-8\"/><style>body {{ font: caption; font-size: {}px; }} p {{ display: inline-block; text-align: center; }}</style></head><body>{}</body>",
          font_size,
          markdown::to_html(message)
        );

        let html = NSString::from_str(&html);

        let data = unsafe { html.dataUsingEncoding(NSUTF8StringEncoding) }.unwrap();

        use cocoa::base::{id, nil};
        use objc::{class, msg_send, sel, sel_impl};

        let data = Retained::into_raw(data);

        let attributed_string: id = unsafe { msg_send![class!(NSAttributedString), alloc] };

        let attributed_string: id =
            unsafe { msg_send![attributed_string, initWithHTML:data documentAttributes:nil] };

        let attributed_string: Retained<NSAttributedString> =
            unsafe { Retained::from_raw(attributed_string.cast()) }.unwrap();

        let paragraph_style = unsafe { NSMutableParagraphStyle::new() };

        unsafe {
            paragraph_style.setLineBreakMode(NSLineBreakMode::ByTruncatingTail);
        }

        let final_attributed_string = NSMutableAttributedString::alloc();

        let final_attributed_string = NSMutableAttributedString::initWithAttributedString(
            final_attributed_string,
            &attributed_string,
        );

        let length = final_attributed_string.length();

        unsafe {
            final_attributed_string.addAttribute_value_range(
                &NSAttributedStringKey::from_str("NSParagraphStyleAtrributeName"),
                &paragraph_style,
                NSRange {
                    location: 0,
                    length,
                },
            )
        }

        final_attributed_string
    }

    fn message_frame(
        &self,
        attributed_string: Retained<NSMutableAttributedString>,
        options: Option<&ToastConfig>,
    ) -> NSRect {
        let attributed_string = Retained::into_raw(attributed_string);

        let size: CGSize = unsafe { objc2::msg_send![attributed_string, size] };

        let origin = {
            if let Some(opts) = &options {
                CGPoint {
                    x: opts.padding.0,
                    y: -opts.padding.1,
                }
            } else {
                CGPoint {
                    x: self.options.padding.0,
                    y: -self.options.padding.1,
                }
            }
        };

        let size = {
            if let Some(opts) = options {
                CGSize {
                    width: size.width.ceil() + opts.padding.0 * 2.0,
                    height: size.height.ceil() + opts.padding.1 * 2.0,
                }
            } else {
                CGSize {
                    width: size.width.ceil() + self.options.padding.0 * 2.0,
                    height: size.height.ceil() + self.options.padding.1 * 2.0,
                }
            }
        };

        NSRect { origin, size }
    }

    fn window_frame(&self, msg_frame: NSRect, options: Option<&ToastConfig>) -> NSRect {
        let monitor = get_monitor_with_cursor().unwrap();

        let scale_factor = monitor.scale_factor();

        let monitor_size = monitor.size().to_logical::<f64>(scale_factor);

        let monitor_position = monitor.position().to_logical::<f64>(scale_factor);

        let width = msg_frame.size.width;

        let height = msg_frame.size.height;

        let offset = {
            if let Some(opts) = options.cloned() {
                opts.offset
            } else {
                self.options.offset
            }
        };

        let position = {
            if let Some(opts) = options.cloned() {
                opts.position
            } else {
                self.options.position.clone()
            }
        };

        NSRect {
            origin: CGPoint {
                x: match position {
                    ToastPosition::At(x, _) => monitor_position.x + x,
                    _ => monitor_position.x + monitor_size.width / 2.0 - width / 2.0,
                },
                y: match position {
                    ToastPosition::Top => monitor_position.y + monitor_size.height - offset,
                    ToastPosition::Bottom => monitor_position.y + offset,
                    ToastPosition::At(_, y) => (monitor_position.y + monitor_size.height) - y,
                },
            },
            size: CGSize { width, height },
        }
    }

    fn label(
        &self,
        attributed_string: Retained<NSMutableAttributedString>,
        frame: NSRect,
    ) -> Retained<NSTextField> {
        let mtn = MainThreadMarker::new().expect("run on the main thread");

        let label = unsafe { NSTextField::new(mtn) };

        unsafe {
            label.setFrame(frame);

            label.setEditable(false);

            let attributed_string = Retained::into_super(attributed_string);

            label.setAttributedStringValue(&attributed_string);

            label.setTextColor(Some(&NSColor::labelColor()));

            label.setDrawsBackground(false);

            label.setBordered(false);

            label.setTag(1);
        }

        label
    }

    fn resize(&self, message_frame: NSRect, options: Option<&ToastConfig>) {
        let panel_frame = self.window_frame(message_frame, options);

        self.panel.0.setFrame_display(panel_frame, true);

        let content_view = self.panel.0.contentView().unwrap();

        let layer = unsafe { content_view.layer().unwrap() };

        let frame = content_view.frame();

        layer.setCornerRadius(frame.size.height / 2.0);
    }

    fn update_label(&self, message: Retained<NSMutableAttributedString>, frame: NSRect) {
        let content_view = self.panel.0.contentView().unwrap();

        let label = self.label(message, frame);

        if let Some(view) = unsafe { content_view.viewWithTag(1) } {
            unsafe { content_view.replaceSubview_with(&view, &label) };
        } else {
            unsafe { content_view.addSubview(&label) };
        }
    }

    pub fn toast(&self, text: &str, options: Option<ToastConfig>) {
        let is_timer = self.timer.lock().unwrap().is_some();

        if is_timer {
            unsafe { self.timer.lock().unwrap().as_ref().unwrap().0.invalidate() };
        }

        let message = self.message(text, options.as_ref());

        let frame = self.message_frame(
            NSMutableAttributedString::from_attributed_nsstring(&message),
            options.as_ref(),
        );

        self.resize(frame, options.as_ref());

        self.update_label(message, frame);

        let fade_duration = {
            if let Some(opts) = &options {
                opts.fade_duration
            } else {
                self.options.fade_duration
            }
        };

        let shadow = {
            if let Some(opts) = &options {
                opts.shadow
            } else {
                self.options.shadow
            }
        };

        self.panel.0.setHasShadow(shadow);

        self.panel.0.show(fade_duration);

        let panel = self.panel.0.retain();

        let callback: Box<dyn Fn(NonNull<NSTimer>)> = Box::new(move |_| {
            panel.hide(fade_duration);
        });

        let duration = {
            if let Some(opts) = &options {
                opts.duration
            } else {
                self.options.duration
            }
        };

        let timer = unsafe {
            NSTimer::scheduledTimerWithTimeInterval_repeats_block(
                duration,
                false,
                &RcBlock::new(callback),
            )
        };

        *self.timer.lock().unwrap() = Some(ToastTimer(timer));
    }
}

pub trait ToastExt {
    fn toast(&self, message: &str, options: Option<ToastConfig>);
}

impl<R: Runtime> ToastExt for AppHandle<R> {
    fn toast(&self, message: &str, options: Option<ToastConfig>) {
        let message = message.to_owned();

        let app_handle = self.clone();

        self.run_on_main_thread(move || {
            app_handle.state::<Toast>().toast(&message, options);
        })
        .unwrap();
    }
}

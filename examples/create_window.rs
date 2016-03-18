extern crate egli;
extern crate x11;
extern crate libc;

use egli::{ Display };

fn main() {
    println!("This example requires EGL and xlib installed.");
    println!("On Ubuntu they are named `libegl1-mesa-dev`, `libx11-dev`.");

    let display_and_window = X11DisplayAndWindow::new("Hello EGL", 640, 480);

    let egl_display = Display::from_display_id(display_and_window.display as *mut _)
        .expect("failed to get EGL display");

    println!("Using EGL {}", egl_display.initialize_and_get_version().expect("failed to initialize"));

    let config = egl_display.config_filter()
        .with_red_size(4)
        .with_green_size(4)
        .with_blue_size(4)
        .choose_configs()
        .expect("failed to get configurations")
        .first()
        .expect("no compatible EGL configuration was found")
        .clone();

    let surface = egl_display.create_window_surface(config, display_and_window.window as *mut _)
        .expect("failed to create window surface");

    display_and_window.wait_for_close();
}

use std::ffi::CString;
use std::mem::zeroed;
use std::ptr::{
  null,
  null_mut,
};
use std::os::raw::c_uint;
use x11::xlib;

/// Minimal helper to initialize X11 display and window.
struct X11DisplayAndWindow {
    pub window: std::os::raw::c_ulong,
    pub display: *mut x11::xlib::Display,
    wm_delete_window: std::os::raw::c_ulong,
    wm_protocols: std::os::raw::c_ulong,
}

impl X11DisplayAndWindow {
    pub fn new(title: &'static str, default_width: c_uint, default_height: c_uint) -> X11DisplayAndWindow {
        let window;
        let display;
        let wm_delete_window;
        let wm_protocols;

        unsafe {
            // Open display
            display = xlib::XOpenDisplay(null());
            if display == null_mut() {
                panic!("can't open display");
            }

            // Load atoms
            let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();
            let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();

            wm_delete_window = xlib::XInternAtom(display, wm_delete_window_str.as_ptr(), xlib::False);
            wm_protocols = xlib::XInternAtom(display, wm_protocols_str.as_ptr(), xlib::False);

            if wm_delete_window == 0 || wm_protocols == 0 {
                panic!("can't load atoms");
            }

            // Create window
            let screen_num = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen_num);
            let white_pixel = xlib::XWhitePixel(display, screen_num);

            let mut attributes: xlib::XSetWindowAttributes = zeroed();
            attributes.background_pixel = white_pixel;

            window = xlib::XCreateWindow(display, root, 0, 0, default_width, default_height, 0, 0,
                                         xlib::InputOutput as c_uint, null_mut(),
                                         xlib::CWBackPixel, &mut attributes);

           // Set window title
           let title_str = CString::new(title).unwrap();
           xlib::XStoreName(display, window, title_str.as_ptr() as *mut _);
        }

        X11DisplayAndWindow {
            window: window,
            display: display,
            wm_delete_window: wm_delete_window,
            wm_protocols: wm_protocols,
        }
    }

    pub fn wait_for_close(&self) {
        unsafe {
          // Subscribe to delete (close) events
          let mut protocols = [self.wm_delete_window];

          if xlib::XSetWMProtocols(self.display, self.window, &mut protocols[0] as *mut xlib::Atom, 1) == xlib::False {
            panic!("can't set WM protocols");
          }

          // Show window
          xlib::XMapWindow(self.display, self.window);

          // Main loop
          let mut event: xlib::XEvent = zeroed();

          loop {
            xlib::XNextEvent(self.display, &mut event);
            match event.get_type() {
              xlib::ClientMessage => {
                let xclient: xlib::XClientMessageEvent = From::from(event);

                // WM_PROTOCOLS client message
                if xclient.message_type == self.wm_protocols && xclient.format == 32 {
                  let protocol = xclient.data.get_long(0) as xlib::Atom;

                  // WM_DELETE_WINDOW (close event)
                  if protocol == self.wm_delete_window {
                    break;
                  }
                }
              },

              _ => {},
            }
          }
        }
    }
}

impl Drop for X11DisplayAndWindow {
    fn drop(&mut self) {
        unsafe {
            xlib::XDestroyWindow(self.display, self.window);
            xlib::XCloseDisplay(self.display);
        }
    }
}

use std::io::{IsTerminal, Write};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard, Once};

pub const Z: &str = "\x1b[0m";
pub const G: &str = "\x1b[1;32m";
pub const Y: &str = "\x1b[1;33m";
pub const R: &str = "\x1b[1;31m";

const ERASE: &str = "\r\x1b[2K\r";

static LIVE: AtomicBool = AtomicBool::new(false);
static STOPPED: AtomicBool = AtomicBool::new(false);
static LINES: AtomicUsize = AtomicUsize::new(0);
static ARM: Once = Once::new();
static OUT: Mutex<()> = Mutex::new(());

fn out() -> MutexGuard<'static, ()> {
    match OUT.lock() {
        Ok(guard) => guard,
        Err(poison) => poison.into_inner(),
    }
}

pub fn arm() {
    ARM.call_once(enable_colour);
}

#[cfg(windows)]
fn enable_colour() {
    const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;
    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

    #[link(name = "kernel32")]
    extern "system" {
        fn GetStdHandle(which: u32) -> *mut core::ffi::c_void;
        fn GetConsoleMode(handle: *mut core::ffi::c_void, mode: *mut u32) -> i32;
        fn SetConsoleMode(handle: *mut core::ffi::c_void, mode: u32) -> i32;
    }

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle.is_null() {
            return;
        }
        let mut mode: u32 = 0;
        if GetConsoleMode(handle, &mut mode) == 0 {
            return;
        }
        let _ = SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
    }
}

#[cfg(not(windows))]
fn enable_colour() {}

pub fn mark_stopped() {
    STOPPED.store(true, Ordering::SeqCst);
}

pub fn stopped() -> bool {
    STOPPED.load(Ordering::SeqCst)
}

fn erase() {
    if std::io::stdout().is_terminal() {
        print!("{}", ERASE);
    }
}

pub fn clear_live() {
    let _out = out();
    if LIVE.swap(false, Ordering::SeqCst) {
        erase();
        let _ = std::io::stdout().flush();
    }
}

pub fn emit(color: &str, msg: &str) {
    let _out = out();
    if LIVE.swap(false, Ordering::SeqCst) {
        erase();
    }
    println!("{}{}{}", color, msg, Z);
    LINES.fetch_add(1, Ordering::Relaxed);
    let _ = std::io::stdout().flush();
}

pub fn live(color: &str, msg: &str) {
    let _out = out();
    if stopped() {
        return;
    }
    if std::io::stdout().is_terminal() {
        if LIVE.load(Ordering::SeqCst) {
            erase();
        }
        print!("{}{}{}", color, msg, Z);
        LIVE.store(true, Ordering::SeqCst);
    } else {
        println!("{}{}{}", color, msg, Z);
        LINES.fetch_add(1, Ordering::Relaxed);
    }
    let _ = std::io::stdout().flush();
}

pub fn hard_blank() {
    let _out = out();
    if LIVE.swap(false, Ordering::SeqCst) {
        erase();
        println!();
    } else {
        println!();
    }
    LINES.fetch_add(1, Ordering::Relaxed);
    let _ = std::io::stdout().flush();
}

pub fn stop_line(msg: &str) {
    mark_stopped();
    let _out = out();
    if LIVE.swap(false, Ordering::SeqCst) {
        erase();
    }
    println!("{}{}{}", R, msg, Z);
    let _ = std::io::stdout().flush();
}

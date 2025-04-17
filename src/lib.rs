#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use windows::{
  Win32::Foundation::HWND,
  Win32::UI::Input::KeyboardAndMouse::{keybd_event, GetAsyncKeyState, SetFocus, VK_MENU, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP},
  Win32::UI::WindowsAndMessaging::{SetForegroundWindow},
  Win32::System::LibraryLoader::GetModuleHandleW,
};

#[napi]
pub fn focus_window(handle_buffer: &[u8]) {
  // Convert buffer to a LONG_PTR, then to HWND
  let mut handle_bytes = [0u8; std::mem::size_of::<isize>()];
  handle_bytes.copy_from_slice(&handle_buffer[..handle_bytes.len()]);
  let handle = isize::from_ne_bytes(handle_bytes);
  let hwnd = HWND(handle  as *mut _);

  // Imitate pressing the Alt key if not already pressed
  let mut pressed = false;
  unsafe {
    if GetAsyncKeyState(VK_MENU.0 as i32) & 0x8000 == 0 {
      pressed = true;
      keybd_event(VK_MENU.0 as u8, 0, KEYEVENTF_EXTENDEDKEY, 0);
    }

    SetForegroundWindow(hwnd);
    SetFocus(hwnd);

    if pressed {
      keybd_event(VK_MENU.0 as u8, 0, KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
    }
  }
}


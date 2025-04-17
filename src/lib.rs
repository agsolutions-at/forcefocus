#![deny(clippy::all)]

use napi::bindgen_prelude::{Buffer, Error, Result};
use napi_derive::napi;

use windows::{
  Win32::Foundation::HWND,
  Win32::UI::Input::KeyboardAndMouse::{
    keybd_event, GetAsyncKeyState, SetFocus, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, VK_MENU,
  },
  Win32::UI::WindowsAndMessaging::SetForegroundWindow,
};

#[napi]
pub fn focus_window(handle_buffer: Buffer) -> Result<()> {
  // Validate size
  if handle_buffer.len() != size_of::<isize>() {
    return Err(Error::from_reason("Invalid HWND buffer size"));
  }

  // Convert &[u8] to isize
  let handle = isize::from_ne_bytes(
    handle_buffer[..]
      .try_into()
      .expect("slice with incorrect length"),
  );

  let hwnd = HWND(handle as *mut _);

  // Imitate pressing the Alt key if not already pressed
  let mut pressed = false;
  unsafe {
    if (GetAsyncKeyState(VK_MENU.0 as i32) as u16 & 0x8000) == 0 {
      pressed = true;
      keybd_event(VK_MENU.0 as u8, 0, KEYEVENTF_EXTENDEDKEY, 0);
    }

    let _ = SetForegroundWindow(hwnd);
    let _ = SetFocus(Some(hwnd));

    if pressed {
      keybd_event(
        VK_MENU.0 as u8,
        0,
        KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP,
        0,
      );
    }
  }

  Ok(())
}

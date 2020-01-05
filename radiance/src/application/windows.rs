extern crate winapi;
use std::ptr::null_mut;
use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::{HWND, POINT};
use winapi::um::errhandlingapi;
use winapi::um::libloaderapi;
use winapi::um::winuser;

macro_rules! utf16_ptr {
    ( $x:expr ) => {
        append_zero($x)
            .encode_utf16()
            .collect::<Vec<u16>>()
            .as_ptr()
    };
}

pub struct Platform {
    instance: HINSTANCE,
    hwnd: HWND,
}

impl Platform {
    pub fn new() -> Self {
        let instance = unsafe { libloaderapi::GetModuleHandleW(std::ptr::null_mut()) };
        let hwnd = Platform::create_window(instance, "Radiance");
        if hwnd.is_null() {
            println!("{}", unsafe { errhandlingapi::GetLastError() });
        }

        Self { instance, hwnd }
    }

    pub fn show_error_dialog(title: &str, msg: &str) {
        unsafe {
            winuser::MessageBoxW(
                null_mut(),
                utf16_ptr!(msg),
                utf16_ptr!(title),
                winuser::MB_OK | winuser::MB_ICONERROR,
            );
        }
    }

    pub fn initialize(&self) {
        unsafe { winuser::ShowWindow(self.hwnd, winuser::SW_SHOW) };
    }

    pub fn process_message(&self) -> bool {
        unsafe {
            let mut msg = winuser::MSG {
                hwnd: null_mut(),
                message: 0,
                wParam: 0,
                lParam: 0,
                time: 0,
                pt: POINT { x: 0, y: 0 },
            };
            let has_msg = winuser::PeekMessageW(&mut msg, null_mut(), 0, 0, winuser::PM_REMOVE) > 0;
            if !has_msg {
                return true;
            }

            if msg.message != winuser::WM_SYSKEYDOWN {
                winuser::TranslateMessage(&msg);
                winuser::DispatchMessageW(&msg);
            }

            match msg.message {
                winuser::WM_QUIT => false,
                _ => true,
            }
        }
    }

    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }

    fn create_window(instance: HINSTANCE, title: &str) -> HWND {
        unsafe {
            let wnd_class = winuser::WNDCLASSW {
                style: winuser::CS_HREDRAW | winuser::CS_VREDRAW,
                lpfnWndProc: Some(Platform::window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: instance,
                hIcon: winuser::LoadIconW(null_mut(), winuser::IDI_APPLICATION),
                hCursor: winuser::LoadCursorW(null_mut(), winuser::IDC_ARROW),
                hbrBackground: null_mut(),
                lpszMenuName: null_mut(),
                lpszClassName: utf16_ptr!(WINDOW_CLASS_NAME),
            };

            winuser::RegisterClassW(&wnd_class);
            winuser::CreateWindowExW(
                winuser::WS_EX_OVERLAPPEDWINDOW,
                utf16_ptr!(WINDOW_CLASS_NAME),
                utf16_ptr!(title),
                winuser::WS_OVERLAPPEDWINDOW,
                winuser::CW_USEDEFAULT,
                winuser::CW_USEDEFAULT,
                800,
                640,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                instance,
                std::ptr::null_mut(),
            )
        }
    }

    extern "system" fn window_proc(
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match message {
            winuser::WM_ERASEBKGND => 1,
            winuser::WM_DESTROY => {
                unsafe { winuser::PostQuitMessage(0) };
                0
            }
            _ => unsafe { winuser::DefWindowProcW(hwnd, message, wparam, lparam) },
        }
    }
}

const WINDOW_CLASS_NAME: &str = "RADIANCE_WINDOW";

/*fn utf16_ptr!<T: Into<String>>(s: T) -> * const u16 {
    append_zero(s).encode_utf16().collect::<Vec<u16>>().as_ptr()
}*/

fn append_zero<T: Into<String>>(s: T) -> String {
    format!("{}\0", s.into())
}

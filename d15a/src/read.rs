extern crate kernel32;
extern crate winapi;

use winapi::HANDLE;
use winapi::wincon::INPUT_RECORD;
use winapi::WORD;
use winapi::DWORD;

static mut CONSOLE_HANDLE: Option<HANDLE> = None;

fn get_input_handle() -> HANDLE
{
    unsafe
	{
        if let Some(handle) = CONSOLE_HANDLE
		{
            return handle;
        }
		else
		{
            let handle = kernel32::GetStdHandle(winapi::STD_INPUT_HANDLE);
            CONSOLE_HANDLE = Some(handle);
            return handle;
        }
    }
}

pub fn read() -> i64
{
    let handle = get_input_handle();
    if handle == winapi::INVALID_HANDLE_VALUE
	{
        panic!("NoConsole")
    }
	let mut buffer = INPUT_RECORD {
		EventType: 0 as WORD,
		Event: [0, 0, 0, 0],
	};
	let mut events: DWORD = 0;
	unsafe
	{
		kernel32::PeekConsoleInputW(handle, &mut buffer, 1, &mut events);
	}
	if events > 0 && buffer.EventType == 1
	{
		unsafe
		{
			kernel32::ReadConsoleInputW(handle, &mut buffer, 1, &mut events);  
			return buffer.KeyEvent().wVirtualKeyCode as i64;
		}
	}
	0
}

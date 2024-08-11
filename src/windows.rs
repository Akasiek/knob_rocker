use log::info;

pub fn hide_console_window() {
    #[cfg(not(windows))]
    {
        return;
    }

    info!("Authenticated with Spotify successfully. Hiding console window.");
    unsafe {
        #[cfg(windows)]
        winapi::um::wincon::FreeConsole()
    };
}

pub fn show_console_window() {
    #[cfg(not(windows))]
    {
        return;
    }

    info!("Showing console window.");
    unsafe {
        #[cfg(windows)]
        winapi::um::consoleapi::AllocConsole()
    };

    println!("Console window shown. To hide it, use the tray menu. DO NOT close this window manual.");
    println!("To get more information, set the log level to debug in the .env file.");
}
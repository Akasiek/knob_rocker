#[cfg(windows)]
extern crate windres;

fn main() {
	#[cfg(windows)]
	if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
    	windres::Build::new().compile("tray.rc").unwrap();
	}
}

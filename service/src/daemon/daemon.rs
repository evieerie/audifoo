use std::io::Result;

use launchctl::Service;

fn start_service() {
    match std::env::consts::OS {
        "macos" => start_macos().unwrap(),
        _ => panic!("Cannot identify OS"),
    }
}

//TODO plist file? idk i need to research launchd more.
fn start_macos() -> Result<()> {
    let basic_service = Service::builder().name("com.audifoo.service").build();

    basic_service.start()
}

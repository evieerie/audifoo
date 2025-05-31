use std::env::home_dir;

use stream::stream::Streamer;

mod daemon;
mod stream;

fn main() {
    println!("starting stream");

    // idk why this path is so fucked. i blame apple
    let path = home_dir()
        .unwrap()
        .join("Music")
        .join("Music")
        .join("Media")
        .join("Music")
        .join("underscores")
        .join("fishmonger")
        .join("09 The fish song.wav");
    let path2 = home_dir()
        .unwrap()
        .join("Music")
        .join("Music")
        .join("Media")
        .join("Music")
        .join("underscores")
        .join("fishmonger")
        .join("04 Kinko's field trip 2006.wav");

    let streamer = Streamer::new().unwrap();
    streamer.append_to_queue(path);
    streamer.append_to_queue(path2);
    streamer.play();
    streamer.skip();

    loop {}
}

use vv::sound;

fn main() {
    let file_path = "assets/audio.wav";
    sound::play(&file_path[..]);

    loop {
        println!("Hello, world!");
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}

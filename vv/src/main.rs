use vv::sound;

fn main() {
    let file_path = "audio.wav";
    sound::play(&file_path[..]);
}

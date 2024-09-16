use vv::{sound, voicevox};

#[tokio::main(flavor = "current_thread")]
async fn main() {

    let text_path = "assets/text.txt";

    let voicevox_engine = voicevox::VoiceVoxEngine::new();
    let audio_path = voicevox_engine.synthesis(&text_path[..]).await;
    sound::play(&audio_path[..]);

    // loop {
    //     println!("Hello, world!");
    //     std::thread::sleep(std::time::Duration::from_secs(5));
    // }
}

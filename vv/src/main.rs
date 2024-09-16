use vv::{sound, voicevox};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let voicevox_engine = voicevox::VoiceVoxEngine::new();

    let text = "こんにちは、音声合成の世界へようこそなのだ";
    let text_id = "xxx";

    let audio_path = voicevox_engine.synthesis(&text_id, &text).await;
    sound::play(&audio_path[..]);

    // loop {
    //     println!("Hello, world!");
    //     std::thread::sleep(std::time::Duration::from_secs(5));
    // }
}

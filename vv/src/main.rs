use vv::platform::twitch::Twitch;
use vv::sound;
use vv::voicevox::VoiceVoxEngine;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let voicevox_engine = VoiceVoxEngine::new();

    let platform = Twitch::new();
    let text = platform.get_chat_message();
    let text_id = "xxx";

    let audio_path = voicevox_engine.synthesis(&text_id, &text).await;
    sound::play(&audio_path[..]);

    // loop {
    //     println!("Hello, world!");
    //     std::thread::sleep(std::time::Duration::from_secs(5));
    // }
}

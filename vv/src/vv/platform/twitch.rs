pub struct Twitch {}

impl Twitch {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_chat_message(&self) -> String {
        String::from("こんにちは、音声合成の世界へようこそなのだ")
    }
}

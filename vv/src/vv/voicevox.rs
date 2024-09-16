use std::fs;
use std::path::Path;

use reqwest::{Body, Client};


const VOICEVOX_TMP_DIR: &str = "./tmp/sounds";

pub struct VoiceVoxEngine {
    url: String,
    port: i32,
    client: Client,
}

impl VoiceVoxEngine {
    pub fn new() -> Self {
        VoiceVoxEngine {
            url: String::from("http://127.0.0.1"),
            port: 50021,
            client: Client::new(),
        }
    }

    async fn audio_query(&self, text: &str, speaker_id: &str, file_path: &Path) {
        let response = self.client.post(&format!("{}:{}/{}", self.url, self.port, "audio_query"))
            .header("Content-Type", "application/json")
            .query(&[("speaker", &speaker_id), ("text", &text)])
            .send()
            .await;

        let response_text = response.expect("response error").text().await.unwrap();
        fs::write(&file_path, response_text).expect("write error");
    }

    pub async fn synthesis(&self, text_id: &str, text: &str) -> String {
        let speaker_id = "1";

        let dst_dir = Path::new(VOICEVOX_TMP_DIR).join(text_id);
        let query_path = dst_dir.join("query.json");
        let audio_path = dst_dir.join("audio.wav");
        if !dst_dir.exists() {
            fs::create_dir_all(&dst_dir).expect("create dir error");
        }

        // audio queryを生成
        self.audio_query(&text, &speaker_id, &query_path.as_path()).await;

        // audio queryから音声合成
        let query = fs::read_to_string(query_path).expect("file not found");
        let response = self.client.post(format!("{}:{}/synthesis", self.url, self.port))
            .header("Content-Type", "application/json")
            .query(&[("speaker", &speaker_id)])
            .body(Body::from(query))
            .send()
            .await;

        let response_bytes = response.expect("response error").bytes().await.unwrap();
        fs::write(&audio_path, response_bytes).expect("write error");
        audio_path.into_os_string().into_string().unwrap()
    }
}

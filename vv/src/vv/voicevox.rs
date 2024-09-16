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

    pub async fn synthesis(&self, file_path: &str) -> String {
        let text_id = "xxx";
        let speaker_id = "1";

        let query_path = Path::new("assets/query.json");  // TODO
        let dst_dir = Path::new(VOICEVOX_TMP_DIR).join(text_id);
        let audio_path = dst_dir.join("audio.wav");

        let query = fs::read_to_string(query_path).expect("file not found");

        if !dst_dir.exists() {
            fs::create_dir_all(&dst_dir).expect("create dir error");
        }

        let response = self.client.post(format!("{}:{}/synthesis", self.url, self.port))
            .header("Content-Type", "application/json")
            .query(&[("speaker", &speaker_id)])
            .body(Body::from(query))
            .send()
            .await;

        if response.is_err() {
            println!("response error = {:?}", response);
        }

        let response_bytes = response.expect("response error").bytes().await.unwrap();
        fs::write(&audio_path, response_bytes).expect("write error");
        audio_path.into_os_string().into_string().unwrap()
    }
}

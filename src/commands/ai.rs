use super::CommandHandler;
use clap::Subcommand;
use lopdf::Document;
use serde::{Deserialize, Serialize};
use std::{env, error::Error, fs, path::Path};

#[derive(Serialize)]
struct GenerationRequest {
    contents: Vec<Content>,
    generation_config: GenerationConfig,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct GenerationConfig {
    temperature: f32,
    max_output_tokens: i32,
}

// Estructuras para la respuesta
#[derive(Deserialize, Debug)]
struct GenerationResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize, Debug)]
struct Candidate {
    content: CandidateContent,
}

#[derive(Deserialize, Debug)]
struct CandidateContent {
    parts: Vec<PartResponse>,
}

#[derive(Deserialize, Debug)]
struct PartResponse {
    text: String,
}

#[derive(Debug, Subcommand, Clone)]
pub enum AICommands {
    Q { question: String },
    RS { filename: String },
}

impl AICommands {
    pub async fn ask(question: &str) -> Result<String, Box<dyn Error>> {
        let api_key = env::var("GEMINI_API_KEY").expect("Failred to get Gemini API KEY");

        let url = format!(
          "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
         api_key);

        let question = format!("{}", question);

        let request = GenerationRequest {
            contents: vec![Content {
                parts: vec![Part { text: question }],
            }],
            generation_config: GenerationConfig {
                temperature: 0.7,
                max_output_tokens: 2048,
            },
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .json::<GenerationResponse>()
            .await?;

        if let Some(candidate) = response.candidates.first() {
            if let Some(part) = candidate.content.parts.first() {
                return Ok(part.text.clone());
            }
        }

        Err("No se recibio respuesta del modelo".into())
    }

    pub async fn resume_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let file_path = Path::new(filename);
        let ext = file_path
            .extension()
            .expect("Failred to obtain extension file");

        match ext.to_str() {
            Some("txt") => {
                let content_file = fs::read_to_string(file_path).expect("Failed to read file");
                let content_file = format!("Summarize this content to the minimum but so that the point in your language is understood: {}", &content_file);
                let response = AICommands::ask(&content_file).await?;
                println!("{}", response);
            }
            Some("pdf") => {
                let doc = Document::load(file_path);
                let mut content_file = String::new();
                match doc {
                    Ok(doc) => {
                        let pages = doc.get_pages();
                        for (i, _) in pages.iter().enumerate() {
                            let page_number = (i + 1) as u32;
                            let text = doc.extract_text(&[page_number]);
                            content_file.push_str(&text.unwrap_or_default());
                        }
                    }
                    Err(err) => {
                        eprintln!("{err}")
                    }
                }

                let content_file = format!("Summarize this content to the minimum but so that the point in your language is understood {}", &content_file);
                let response = AICommands::ask(&content_file).await?;
                println!("Resume file response: {}", response);
            }
            _ => todo!(),
        }

        Ok(())
    }
}

impl AICommands {
    async fn execute_async(&self) {
        match self {
            AICommands::Q { question } => match Self::ask(question).await {
                Ok(result) => {
                    println!("{}", result);
                }
                Err(e) => {
                    eprintln!("Error al llamar a la API: {}", e);
                }
            },
            AICommands::RS { filename } => {
                if let Err(e) = Self::resume_file(&self, filename).await {
                    eprintln!("Failed to resume file: {}", e);
                }
            }
        }
    }
}

impl CommandHandler for AICommands {
    async fn execute(&self) {
        self.execute_async().await;
    }
}

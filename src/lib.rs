#[path = "impl.rs"]
mod implementation;

pub use implementation::*;

use std::str::FromStr;
use reqwest::Error;

const API_BASE_URL: &str = "https://api.otakugifs.xyz";

// Estructura para representar la información de la banlist
pub struct BanlistInfo {
    pub id: String,
    pub risk: String,
    pub reason: String,
    pub image_urls: Vec<String>,
}

// Definimos una estructura para representar la API Wrapper
pub struct OtakuGifsApi {
    client: reqwest::Client,
}

impl OtakuGifsApi {
    // Constructor para la API Wrapper
    pub fn new() -> OtakuGifsApi {
        // Crear una nueva instancia del cliente reqwest con la configuración para HTTPS
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Error al construir el cliente reqwest.");

        OtakuGifsApi { client }
    }

    // Método para obtener una imagen aleatoria basada en una reacción específica
    pub async fn fetch_random_gif(&self, reaction: &str) -> Result<String, Error> {
        let reaction_enum= Reaction::from_str(reaction).unwrap();
        let url = format!("{}/gif?reaction={}", API_BASE_URL, reaction_enum.as_str());
        let response = self.client.get(url).send().await?.text().await?;
        let data: serde_json::Value = serde_json::from_str(&response).unwrap_or("Ocurrió un Error".into());
        let gif_url = data["url"].as_str().unwrap_or_default().to_string();
        Ok(gif_url)
    }

    // Método para obtener todas las reacciones disponibles
    pub async fn fetch_all_reactions(&self) -> Result<Vec<String>, Error> {
        let url = format!("{}/gif/allreactions", API_BASE_URL);
        let response = self.client.get(url).send().await?.text().await?;
        let data: serde_json::Value = serde_json::from_str(&response).unwrap();
        let reactions: Vec<String> = serde_json::from_value(data["reactions"].clone()).unwrap();
        Ok(reactions)
    }
}
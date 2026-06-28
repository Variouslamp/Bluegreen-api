use axum::{routing::get, Json, Router}; // 
use serde::Serialize; //
use chrono::{Utc, FixedOffset}; // Libreria encargada de la creacion y uso de timestamp


const VERSION: &str = "V2.0";

// 1. Definimos la estructura del JSON que queremos responder
#[derive(Serialize)]
struct Respuesta {
    mensaje: String,
    version: String,
    hora: String,
}

// 2. Esta es la función que maneja la petición GET
async fn hola_mundo() -> Json<Respuesta> {
    let utc_menos_5 = FixedOffset::west_opt(5 * 3600).unwrap();
    let ahora = Utc::now().with_timezone(&utc_menos_5);

    let respuesta = Respuesta {
        version: VERSION.to_string(),
        mensaje: "ok".to_string(),
        hora: ahora.format("%d-%m-%Y %H:%M:%S").to_string(),
    };

    Json(respuesta)
}

#[tokio::main]
async fn main() {
    // 3. Creamos el enrutador y mapeamos el path "/" a nuestra función
    let app = Router::new().route("/", get(hola_mundo));

    // 4. Encendemos el servidor en el puerto 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Servidor corriendo en http://localhost:8080");
    
    axum::serve(listener, app).await.unwrap();
}

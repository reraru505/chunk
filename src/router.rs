use std::{io::Write, path::PathBuf};

use actix_multipart::form::MultipartFormConfig;
use actix_web::web::service;
use actix_web::{
    dev::Response, get, mime::Name, post, web, App, HttpServer, Responder 
};
use actix_web::HttpResponse;
use actix_files::{Files , NamedFile};
use sanitize_filename::sanitize;
use actix_multipart::form::{tempfile::TempFile , text::Text , MultipartForm};
use serde::Serialize;


pub struct RouterConfigure{
    port : u16,
    ip : String,
}

impl RouterConfigure{
    
    pub fn new(ip : String , port : u16) -> Self {
        Self { port: port, ip: ip }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        
        println!("Server started at {}:{}", self.ip , self.port);
       
        HttpServer::new(
            || {
                App::new()                                                       
                    .app_data(
                        MultipartFormConfig::default()
                        .total_limit(5 * 1024 * 1024 * 1024) // 5GB total limit
                    )
                    .route("/upload", web::post().to(upload_file))
                    .configure(Self::route_files)
                    .service(list_files)
            }
        )
            .bind((self.ip.clone(), self.port))?
            .run()
            .await

        
    }

    pub fn  route_files(config : &mut web::ServiceConfig){

        config.route("/", web::get().to(
                async || ->std::io::Result<NamedFile> {
                    Ok(NamedFile::open(PathBuf::from("html/index.html"))?)
                }
        ));

        config.route("/upload", web::get().to(
                async || ->std::io::Result<NamedFile> {
                    Ok(NamedFile::open(PathBuf::from("html/upload.html"))?)
                }
        ));
        
        config.route("/upload-script", web::get().to(
                async || ->std::io::Result<NamedFile> {
                    Ok(NamedFile::open(PathBuf::from("html/upload.js"))?)
                }
        ));
 
        config.route("/watch", web::get().to(
                async || ->std::io::Result<NamedFile> {
                    Ok(NamedFile::open(PathBuf::from("html/watch.html"))?)
                }
        ));
        
        config.route("/watch-script", web::get().to(
                async || ->std::io::Result<NamedFile> {
                    Ok(NamedFile::open(PathBuf::from("html/watch.js"))?)
                }
        ));
 

    }


}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "5GiB")]
    file: TempFile,
    file_name: Text<String>,
}

async fn upload_file(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {

    let upload_dir = PathBuf::from("uploads");
    if !upload_dir.exists() {
        std::fs::create_dir(upload_dir.clone()).unwrap();
    }

    // FIX: Take ownership of the TempFile
    let file = form.file;
    let original_filename = file.file_name.as_ref().unwrap_or(&form.file_name);
    let safe_filename = sanitize(original_filename);
    let final_filename = if safe_filename.is_empty() {
        "upload.bin".to_string()
    } else {
        safe_filename.clone()
    };

    let destination_path = upload_dir.join(&final_filename);

    // This now works because `file` is owned
    match file.file.persist(&destination_path) {
        Ok(_) => {
            println!(
                "File '{}' successfully uploaded to '{}'.",
                original_filename,
                destination_path.display()
            );
            HttpResponse::Ok().json(format!("File '{}' uploaded successfully!", final_filename))
        }
        Err(e) => {
            eprintln!("Failed to save file: {}", e);
            HttpResponse::InternalServerError().json("Failed to save file.")
        }
    }
}


 // A simple struct to hold the file info for JSON response
#[derive(Serialize)]
struct FileInfo {
    name: String,
    url: String,
}

#[get("/list-files")]
async fn list_files() -> impl Responder {
    let upload_dir = PathBuf::from("uploads");
    if !upload_dir.exists() {
        return HttpResponse::Ok().json(Vec::<FileInfo>::new());
    }

    let mut files = Vec::new();
    
    // Read the directory, filter for common video types
    if let Ok(entries) = std::fs::read_dir(upload_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    // Basic check for video extensions
                    if name.ends_with(".mp4") || name.ends_with(".webm") || name.ends_with(".ogg") {
                        files.push(FileInfo {
                            name: name.to_string(),
                            url: format!("/uploads/{}", name),
                        });
                    }
                }
            }
        }
    }

    HttpResponse::Ok().json(files)
}


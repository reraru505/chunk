use std::{io::Write, path::PathBuf};

use actix_web::{
    dev::Response, get, mime::Name, post, web, App, HttpServer, Responder 
};
use actix_files::{Files , NamedFile};
use actix_easy_multipart::{text::Text, MultipartForm};



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
                    .app_data(web::PayloadConfig::new(200 * 1024 * 1024))
                    .service(upload_chunk)
                    .configure(Self::route_files)
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


    }


}


#[derive(Debug, MultipartForm)]
pub struct ChunkForm {
    pub file_name: Text<String>,
    pub start: Text<u64>,
    pub end: Text<u64>,
    pub user_file: actix_easy_multipart::tempfile::Tempfile,
}

#[post("/upload_chunk")]
async fn upload_chunk(form: MultipartForm<ChunkForm>) -> impl Responder {
    // Ensure media folder exists
    std::fs::create_dir_all("media").unwrap();

    let path = format!("media/{}", form.file_name.0);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .unwrap();

    // Copy chunk to file
    std::io::copy(&mut form.user_file.file.as_file(), &mut file).unwrap();

    format!("Chunk {}-{} uploaded", form.start.0, form.end.0)
}

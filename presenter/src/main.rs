#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use std::{
    env,
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
};

use rocket::{
    config::{Config, Environment, LoggingLevel},
    http::{
        hyper::header::{ContentEncoding, Encoding},
        ContentType,
    },
    response::{self, Responder, Response},
    Request,
};

const ADDR: &str = "0.0.0.0";
const PORT: u16 = 8000;
const INDEX: &str = "index.html";

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Need to pass an arg which is the path to serve");
    let root = Path::new(&path);
    env::set_current_dir(&root).expect("Unable to set path to serve");
    {
        File::open(INDEX).expect("Unable to open index file");
    }

    println!(
        "Listening for requests at http://{}:{} from path {:?}",
        ADDR,
        PORT,
        root.display()
    );

    let config = Config::build(Environment::Production)
        .address(ADDR)
        .port(PORT)
        .log_level(LoggingLevel::Off)
        .expect("Unable to configure the server");

    rocket::custom(config)
        .mount("/", routes![index, files])
        .launch();
}

#[get("/")]
fn index() -> Encoder<File> {
    let index_file = File::open(INDEX).expect("Unable to serve index file");
    let (content_type, responder) = (ContentType::HTML, index_file);
    Encoder {
        content_type,
        encoding_gzip: false,
        responder,
    }
}

#[get("/<filename..>")]
fn files(filename: PathBuf) -> Encoder<File> {
    let path = Path::new(".").join(filename);
    let extension = path.extension().and_then(OsStr::to_str).unwrap_or("txt");
    let content_type = ContentType::from_extension(extension).unwrap_or(ContentType::Plain);
    if let Ok(responder) = File::open(&path) {
        return Encoder {
            content_type,
            encoding_gzip: false,
            responder,
        };
    }
    let gzpath = path.to_str().unwrap().to_owned() + ".gz";
    if let Ok(responder) = File::open(&gzpath) {
        return Encoder {
            content_type,
            encoding_gzip: true,
            responder,
        };
    }
    index()
}

struct Encoder<R> {
    content_type: ContentType,
    encoding_gzip: bool,
    responder: R,
}

impl<'r, R: Responder<'r>> Responder<'r> for Encoder<R> {
    #[inline(always)]
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        let mut response = Response::build()
            .merge(self.responder.respond_to(request)?)
            .header(self.content_type)
            .finalize();
        if self.encoding_gzip {
            response.set_header(ContentEncoding(vec![Encoding::Gzip]));
        }
        Ok(response)
    }
}

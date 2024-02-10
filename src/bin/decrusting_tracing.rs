use std::io::Read;
// notes and code from decrusting the tracing crate: https://www.youtube.com/watch?v=21rtHinFA40
use tracing::{debug, error, info, span, trace, warn, Level};

fn main() {
    tracing_subscriber::fmt::init();

    let span = span!(Level::INFO, "main");
    /*NB: guards cant be just underscores, otherwise they will be dropped immediately */
    let _guard = span.enter();
    for file in std::env::args() {
        let span = span!(Level::INFO, "file");
        let _guard = span.enter();
        info!("opening the file");
        let mut file = std::fs::File::open(file).unwrap();
        info!("reading file contents");
        let mut bytes = Vec::new();
        file.read_exact(&mut bytes).unwrap();
        info!(bytes.number = 0, "parsing");
        info!("parsing");
        // do work
        info!("done with file");
    }
}

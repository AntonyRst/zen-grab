mod args;
mod extractor;

use clap::Parser;
use args::ZenArgs;
use std::process;

fn main() {
    let args = ZenArgs::parse();

    // Ejecutamos y si hay error, lo imprimimos bonito en lugar de un panic
    if let Err(e) = extractor::extraer_datos(&args) {
        eprintln!("Error crítico: {}", e);
        process::exit(1);
    }
}
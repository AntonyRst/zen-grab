use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "zen_grab", version = "1.0", about = "Extractor de código pro")]
pub struct ZenArgs {
    /// Ruta de la carpeta a buscar
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Extensiones de archivos (ej: rs,js,scss)
    #[arg(short, long, value_delimiter = ',')]
    pub extensions: Vec<String>,

    /// Nombre del archivo de salida
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// No mostrar mensajes en la terminal
    #[arg(short, long)]
    pub silent: bool,
}
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "zen_grab", 
    version = "1.0", 
    about = "Zen-Grab: Extractor de código minimalista y eficiente"
)]
pub struct ZenArgs {
    /// Archivo de salida donde se guardará el reporte (Ej: todo.txt)
    #[arg(help = "Nombre del archivo de salida")]
    pub output: PathBuf,

    /// Extensiones a filtrar separadas por espacios (Ej: -e rs scss js)
    #[arg(
        short, 
        long, 
        value_delimiter = ' ', 
        num_args = 1..,
        help = "Lista de extensiones (ej: rs scss)"
    )]
    pub extensions: Vec<String>,

    /// Ruta de la carpeta a buscar (por defecto es el directorio actual)
    #[arg(short, long, default_value = ".")]
    pub path: PathBuf,

    /// Modo silencioso: no muestra mensajes en la terminal
    #[arg(short, long)]
    pub silent: bool,
}
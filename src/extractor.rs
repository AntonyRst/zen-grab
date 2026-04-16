use crate::args::ZenArgs;
use std::fs;
use std::io::{self};
use walkdir::WalkDir;

pub fn extraer_datos(args: &ZenArgs) -> io::Result<()> {
    let mut contenido_total = String::new();

    // 1. Pre-procesamos las extensiones una sola vez fuera del bucle
    // Aplicamos trim() para borrar espacios y to_lowercase() para uniformidad
    let extensiones_limpias: Vec<String> = args.extensions
        .iter()
        .map(|e| e.trim().to_lowercase())
        .collect();

    if !args.silent {
        println!("🚀 Zen-Grab: Escaneando {:?}", args.path);
    }

    // 2. Iteración sobre el sistema de archivos
    for entrada in WalkDir::new(&args.path).into_iter().filter_map(|e| e.ok()) {
        let ruta = entrada.path();

        if ruta.is_file() {
            // Lógica de filtrado robusta
            let match_extension = extensiones_limpias.is_empty() || {
                ruta.extension()
                    .and_then(|s| s.to_str())
                    .map(|ext| {
                        // Comparamos en minúsculas para evitar errores entre .RS y .rs
                        let ext_lower = ext.to_lowercase();
                        extensiones_limpias.contains(&ext_lower)
                    })
                    .unwrap_or(false)
            };

            if match_extension {
                if !args.silent {
                    println!("  -> Añadiendo: {:?}", ruta.display());
                }

                // Intentamos leer el archivo como texto
                if let Ok(texto) = fs::read_to_string(ruta) {
                    contenido_total.push_str(&format!("\n--- FILE: {:?} ---\n", ruta));
                    contenido_total.push_str(&texto);
                    contenido_total.push_str("\n");
                }
            }
        }
    }

    // 3. Gestión de la salida (Archivo o Standard Output)
    match &args.output {
        Some(ruta_salida) => {
            fs::write(ruta_salida, contenido_total)?;
            if !args.silent {
                println!("✅ Reporte generado en: {:?}", ruta_salida);
            }
        }
        None => {
            if !contenido_total.is_empty() {
                println!("{}", contenido_total);
            } else if !args.silent {
                println!("⚠️ No se encontraron archivos que coincidan con los filtros.");
            }
        }
    }

    Ok(())
}
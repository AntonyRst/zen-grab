use crate::args::ZenArgs;
use std::fs;
use std::io::{self, BufReader, Read};
use walkdir::WalkDir;

pub fn extraer_datos(args: &ZenArgs) -> io::Result<()> {
    let mut contenido_total = String::new();

    // 1. Pre-procesamos las extensiones
    let extensiones_limpias: Vec<String> = args.extensions
        .iter()
        .map(|e| e.trim().to_lowercase())
        .collect();

    if !args.silent {
        println!("🚀 Zen-Grab: Escaneando {:?}", args.path);
    }

    // 2. Iteración sobre los archivos
    for entrada in WalkDir::new(&args.path).into_iter().filter_map(|e| e.ok()) {
        let ruta = entrada.path();

        if ruta.is_file() {
            let match_extension = extensiones_limpias.is_empty() || {
                ruta.extension()
                    .and_then(|s| s.to_str())
                    .map(|ext| {
                        let ext_lower = ext.to_lowercase();
                        extensiones_limpias.contains(&ext_lower)
                    })
                    .unwrap_or(false)
            };

            if match_extension {
                if !args.silent {
                    println!("  -> Añadiendo: {:?}", ruta.display());
                }

                // Blindaje con BufReader
                if let Ok(archivo_origen) = fs::File::open(ruta) {
                    let mut lector = BufReader::new(archivo_origen);
                    
                    contenido_total.push_str(&format!("\n--- FILE: {:?} ---\n", ruta));
                    
                    if let Err(e) = lector.read_to_string(&mut contenido_total) {
                        if !args.silent {
                            eprintln!("  [!] Error al leer {:?}: {}", ruta, e);
                        }
                    }
                    contenido_total.push_str("\n");
                }
            }
        }
    }

    // 3. Gestión de la salida simplificada
    // Como 'output' ahora es obligatorio en args.rs, ya no necesitamos el match Some/None
    if !contenido_total.is_empty() {
        fs::write(&args.output, contenido_total)?;
        if !args.silent {
            println!("✅ Reporte generado en: {:?}", args.output);
        }
    } else {
        if !args.silent {
            println!("⚠️ No se encontraron archivos con esas extensiones. No se creó el reporte.");
        }
    }

    Ok(())
}
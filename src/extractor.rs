use crate::args::ZenArgs;
use std::fs;
use std::io::{self, Write};
use walkdir::WalkDir;

pub fn extraer_datos(args: &ZenArgs) -> io::Result<()> {
    let mut contenido_total = String::new();

    if !args.silent {
        println!("🚀 Zen-Grab: Escaneando {:?}", args.path);
    }

    for entrada in WalkDir::new(&args.path).into_iter().filter_map(|e| e.ok()) {
        let ruta = entrada.path();

        if ruta.is_file() {
            // Lógica de filtrado por extensiones
            let match_extension = args.extensions.is_empty() || {
                ruta.extension()
                    .and_then(|s| s.to_str())
                    .map(|ext| args.extensions.contains(&ext.to_string()))
                    .unwrap_or(false)
            };

            if match_extension {
                if !args.silent {
                    println!("  -> Añadiendo: {:?}", ruta.display());
                }

                // Usamos read_to_string, pero manejamos el error por si no es texto
                if let Ok(texto) = fs::read_to_string(ruta) {
                    contenido_total.push_str(&format!("\n--- FILE: {:?} ---\n", ruta));
                    contenido_total.push_str(&texto);
                    contenido_total.push_str("\n");
                }
            }
        }
    }

    // Decidir salida
    match &args.output {
        Some(ruta_salida) => {
            fs::write(ruta_salida, contenido_total)?;
            if !args.silent {
                println!("✅ Reporte generado en: {:?}", ruta_salida);
            }
        }
        None => println!("{}", contenido_total),
    }

    Ok(())
}
 Zen-Grab CLI

Zen-Grab es una herramienta minimalista, rápida y segura escrita en Rust. Su objetivo es consolidar múltiples archivos de código en un único documento de texto, facilitando auditorías, revisiones de código o generación de contexto para IAs.
 ¿Para qué sirve?

Imagina que tienes un proyecto con decenas de archivos y quieres enviárselos a un mentor o a una IA. En lugar de copiar y pegar manualmente, Zen-Grab recorre tus carpetas, filtra lo que necesitas y lo entrega todo listo en un solo archivo.
 Cómo funciona (Bajo el capó)

    Escaneo Eficiente: Utiliza recursividad para navegar por directorios sin importar la profundidad.

    Filtrado Selectivo: Tú decides qué extensiones incluir. El programa ignora binarios, imágenes y archivos pesados innecesarios.

    Lectura Blindada (BufReader): A diferencia de otras herramientas, Zen-Grab utiliza un sistema de lectura por búfer que protege la memoria RAM de tu sistema, permitiendo procesar archivos de forma estable incluso en entornos con pocos recursos.

Instalación
Opción 1: Instalación Rápida (Recomendado)

Si tienes Rust instalado, puedes instalarlo directamente desde el repositorio:
Bash

cargo install --git https://github.com/AntonyRst/zen-grab

Opción 2: Compilación Manual

    Clona el código:
    Bash

    git clone https://github.com/AntonyRst/zen-grab.git
    cd zen-grab

    Compila la versión optimizada:
    Bash

    cargo build --release

    Añádelo a tu PATH:
    Bash

    cp target/release/zen_grab ~/.local/bin/

Guía de Uso

La nueva sintaxis es más limpia y "Zen". El primer argumento siempre es el archivo de salida.
1. Guardar todo el código de la carpeta actual
Bash

zen_grab reporte.txt

2. Filtrar por extensiones (sin comas, solo espacios)
Bash

zen_grab mi_codigo.txt -e rs scss js

3. Especificar una carpeta diferente
Bash

zen_grab salida.txt -p ./src/componentes -e rs

 Notas de Seguridad y Limitaciones

    Gestión de Memoria: Zen-Grab está diseñado para ser seguro. Sin embargo, se recomienda usar el flag -e (extensiones) cuando se escaneen directorios que contengan archivos binarios muy grandes para evitar consumos de RAM innecesarios.

    Privacidad: La herramienta corre 100% local. No envía datos a internet ni guarda información del usuario.

    Compatibilidad: Probado extensamente en Arch Linux, pero es totalmente multiplataforma (Windows, macOS y Linux).

Estructura del Proyecto

    main.rs: El punto de entrada que coordina la ejecución.

    args.rs: Definición de la interfaz de comandos usando Clap.

    extractor.rs: El motor de la herramienta; gestiona la lectura eficiente de archivos y el filtrado.

 Sobre el Autor

Soy Juan Antonio González, un desarrollador enfocado en sistemas, eficiencia y minimalismo. Me apasiona crear herramientas CLI que simplifiquen el flujo de trabajo de otros programadores.

    LinkedIn: Juan Antonio González Erazo

Desarrollado con Rust en Arch Linux.




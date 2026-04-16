# 🦀 Zen-Grab CLI

**Zen-Grab** es una herramienta sencilla pero poderosa creada en **Rust**. Su función es entrar en tus carpetas, buscar archivos de código y "agarrar" (grab) todo su contenido para ponerlo en un solo lugar. 

---

## ¿Para qué sirve?

Imagina que tienes un proyecto con 50 archivos de código y quieres enviárselos a un mentor o a una IA para que los revise. En lugar de copiar y pegar archivo por archivo, **Zen-Grab** lo hace por ti en menos de un segundo.

## Cómo funciona (Para principiantes)

Si estás aprendiendo a programar, esto es lo que hace este programa "bajo el capó":

1. **Escaneo Recursivo:** El programa entra en la carpeta que le digas y, si encuentra más carpetas adentro, entra en ellas también. No deja ningún rincón sin revisar.
2. **Filtrado:** Tú le dices: "Solo quiero archivos `.rs` (Rust) o `.js` (JavaScript)". El programa ignora todo lo demás (imágenes, videos, carpetas ocultas).
3. **Consolidación:** Abre cada archivo, lee lo que tiene escrito y lo va pegando en un "buffer" (una memoria temporal) para finalmente escribirlo todo en un solo archivo de texto.

---

## Cómo instalarlo en tu PC

Como este programa está hecho en **Rust**, necesitas tener instalado el lenguaje en tu sistema (si usas **Arch Linux**, el comando `sudo pacman -S rust` es tu amigo).

1. **Descarga el código:**
   ```bash
   git clone [https://github.com/tu-usuario/zen-grab.git](https://github.com/tu-usuario/zen-grab.git)
   cd zen-grab

2. **Prepáralo para usar:**

Ejecuta este comando para que Rust cree el ejecutable optimizado:

    ```bash
    cargo build --release
    ```

3. **Muévelo a tus comandos personales:**
Para poder usarlo desde cualquier parte de tu terminal:

    ```bash
    cp target/release/zen_grab ~/.local/bin/
    ```

**Guía de uso rápido**

**Es muy fácil de usar. Solo tienes que abrir tu terminal y escribir:**
1. **El uso más básico**

    ```bash
    zen_grab .
    ```

    (El punto . significa "esta carpeta". Esto imprimirá todo el código de la carpeta actual en tu pantalla).

2. **Guardar en un archivo (Lo más útil)**

    ```bash
    zen_grab . -o mi_codigo.txt
    ```
    (Esto crea un archivo llamado mi_codigo.txt con todo tu trabajo adentro).

3. **Buscar solo archivos específicos**

    ```bash
    zen_grab . -e rs,scss -o reporte.txt
    ```

    (Aquí le estamos diciendo: "Busca solo archivos de Rust y de SASS, y guárdalos en reporte.txt").

**Entendiendo el código (Módulos)**

Si tienes curiosidad de cómo está organizado el código en este repositorio:

    main.rs: Es el director de orquesta. Recibe tus órdenes y llama a los demás.

    args.rs: Aquí es donde el programa "aprende" a entender las banderas que escribes (como -e o -o).

    extractor.rs: Aquí está la magia. Es el código que realmente abre los archivos y lee su contenido.


**Sobre el Autor**

Soy Juan Antonio González, un desarrollador apasionado por los sistemas y la eficiencia. Me encanta crear herramientas que ayuden a otros programadores a trabajar mejor.

    LinkedIn: [www.linkedin.com/in/juan-antonio-gonzález-erazo-96841a403]

**Este proyecto fue desarrollado en Arch Linux. ¡La terminal es el límite!**






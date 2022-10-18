# Primeros pasos con Rust 

1. [Instalar Rust](#Instalar-rust)
2. [Hola Mundo](#hola-mundo)
3. [Hola cargo](#Hola-cargo)


---
## Instalar Rust
El primer paso es instalar Rust. Descargaremos Rust a través rustupde , una herramienta de línea de comandos para administrar las versiones de Rust y las herramientas asociadas. Necesitará una conexión a Internet para la descarga.

Guia de Instalación: https://www.rust-lang.org/es/tools/install

---
## Hola Mundo
Ahora que ha instalado Rust, escribamos su primer programa Rust. Cuando se aprende un nuevo idioma, es tradicional escribir un pequeño programa que imprima el texto `¡Hola Mundo!` en la pantalla, ¡así que haremos lo mismo aquí!

~~~rust
fn main() {
    println!("Hello, world!");
}
~~~

- La función `main()` es especial: Es el punto de entrada de cada aplicación de Rust, desde esta función se inicia la ejecución de un programa en Rust. 
- `println!` llama a una macro de Rust. Si hubiera llamado a una función en su lugar, se ingresaría como `println` (sin el !). Con esta instrucción podemos imprimer datos por pantalla. 

Compilación de una aplicacion en Rust. Para compilar una aplicación de Rust podemos usar el comando `rustc nombre-archivo`, esto no va generar un ejecutable con código binario. 

~~~
rustc main.rs
./main
~~~
---
## Hola Cargo

Cargo es el sistema de compilación y administrador de paquetes de Rust. La mayoría de los rustaceanos usan esta herramienta para administrar sus proyectos de Rust porque Cargo maneja muchas tareas por usted, como construir su código, descargar las bibliotecas de las que depende su código y construir esas bibliotecas. (Llamamos a las bibliotecas que su código necesita dependencias ).

~~~
cargo --version 
~~~

Creando un proyecto con cargo 

~~~
cargo new nombre-proyecto
~~~
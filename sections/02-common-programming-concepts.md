# Conceptos comunes de programación

1. [Variables](#Variables)
2. [Constantes](#Constantes)
3. [Variable Sombreado](#Variable-Sombreado)
4. [Tipos de datos](#Tipos-de-datos)
5. [Funciones](#Funciones)
6. []

---
## Variables
Definición de variables sin inicializar. y luego puedes inicializar. 

~~~rust
    let a :u8; //Definición de variable 
    a = 45; // Inicialización de variable 
   
    println!("Valor de a: {}", a);
~~~

Definición de variables de forma dinamica inicializando. 

~~~rust
    // Definición y inicialización de variable 
    let age = 26;
    let name = "Alex";
    println!("Hola soy {name} y tengo {age} años");
~~~

Las variables en Rust son inmutables, una vez asignado un valor a una variable yo no se puede modificarse. 

~~~rust
    // Definición y inicialización de variable 
    let age = 26;
    let name = "Alex";
    println!("Hola soy {name} y tengo {age} años");

    name = "Roel";

    println!("Hola soy {name} y tengo {age} años");
~~~

Resultado del código anterior. 

~~~
error[E0384]: cannot assign twice to immutable variable `name`
  --> src\main.rs:12:5
   |
9  |     let name = "Alex";
   |         ----
   |         |
   |         first assignment to `name`
   |         help: consider making this binding mutable: `mut name`
...
12 |     name = "Roel";
   |     ^^^^^^^^^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `prueva` due to previous error
~~~

### Variables mutables  
Para que una variable sea mutable tenemos que agregar la palabra reservada `mut`. 

~~~rust
    let  mut name = "Alex"; //Variable  mutable
    println!("Hola soy {name} y tengo {age} años");

    name = "Roel";

    println!("Hola soy {name} y tengo {age} años");
~~~

---
## Constantes
Al igual que las variables inmutables, las constantes son valores que están vinculados a un nombre y no se les permite cambiar, pero existen algunas diferencias entre las constantes y las variables.

- Primero, no está permitido usar `mut` con constantes. 
- Las constantes no solo son inmutables de forma predeterminada, siempre son inmutables. 
- Las constantes se declaran utilizando la palabra clave  `const` en lugar de la palabra clave `let` y se debe anotar el tipo de dato y valor.

~~~rust
   const VALUE_PI:f32 = 3.141592;

    println!("Valor de PI: {VALUE_PI}");
~~~

--- 
## Variable Sombreado 
- Con Rust puede declarar una nueva variable con el mismo nombre que una variable anterior. 
- Los rustaceanos dicen que la primera variable está sombreada por la segunda, lo que significa que la segunda variable es lo que verá el compilador cuando use el nombre de la variable.
- En efecto, la segunda variable eclipsa a la primera, tomando cualquier uso del nombre de la variable para sí misma hasta que se oscurece o finaliza el alcance.

~~~rust
    let a = 5;

    let a = a + 5; 

    println!("Valor de a: {a}")
~~~

La otra diferencia entre `mut` y `shadowing` es que debido a que estamos creando efectivamente una nueva variable cuando usamos la palabra clave `let` nuevamente, podemos cambiar el tipo de valor pero reutilizar el mismo nombre. Por ejemplo, digamos que nuestro programa le pide a un usuario que muestre cuántos espacios quiere entre un texto ingresando caracteres de espacio, y luego queremos almacenar esa entrada como un número:

~~~rust
    let spaces = "   ";
    println!("{spaces}");

    let spaces = spaces.len();
    println!("{spaces}");

    let spaces = 45;
    println!("{spaces}")
~~~

---
## Tipos de datos

Cada valor en Rust es de un determinado tipo de datos , lo que le dice a Rust qué tipo de datos se especifican para que sepa cómo trabajar con esos datos. Veremos dos subconjuntos de tipos de datos: escalar y compuesto.

### Tipos escalares
Un tipo escalar representa un único valor. Rust tiene cuatro tipos escalares primarios: enteros, números de coma flotante, booleanos y caracteres. Puede reconocerlos de otros lenguajes de programación. Veamos cómo funcionan en Rust.

- Tipo entero 

~~~rust
    /* Tipos enteros 
    i8 - i128 - isize -> Números enteros 
    u8 - u 128 - usize -> Números enteros  positivos
    */
    let a:i8=25;
    let age:u8= 26;
~~~

- Tipo decimal

~~~rust
    let a=2.0; //f64
    let b:f32= 4.5; //f32
~~~

- Tipos booleanos 

~~~rust
    let t = true;

    let f: bool = false; // indicando el tipo de dato
~~~

- Tipo carácter 

~~~rust
    let c = 'c';
    let z: char = 'ℤ'; //con tipo de datos explicito
    let heart_eyed_cat = '🦀';
~~~

### Tipos compuestos
Los tipos compuestos pueden agrupar varios valores en un solo tipo. Rust tiene dos tipos de compuestos primitivos: tuplas y matrices.

- Tipo tupla

~~~rust
    let tup = (500, 6.4, 'A'); // Definición de una tupla

    let (a, b, c) = tup; //Desestructuración

    println!("El valor de a es {a}");
    println!("El valor de b es {b}");
    println!("El valor de c es {c}");

    //Acediendo a su valo medainte indece
    println!("El valor es {}", tup.0);
~~~


- Tipo matriz 

~~~rust
    let arr1 = [10,20,30,40,50]; //Creando una matriz

    let a = [3; 5]; //Creando una matriz 5 elementos con 3

    //Mostrando datos de una matriz
    println!("{}", arr1[0]); 
    println!("{}", a[0]);
~~~

---
## Funciones

~~~rust
fn main() {
    hello("Alex");
    println!("La suma es {}", sum(10, 40))
}

fn hello(name:&str){
    println!("Hola, {name}")
}

fn sum(a:i32, b :i32)->i32{
    //return a + b;
    a+b 
}
~~~





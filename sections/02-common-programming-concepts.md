# Conceptos comunes de programación

1. [Variables](#Variables)
2. [Constantes](#Constantes)
3. [Variable Sombreado](#Variable-Sombreado)
4. [Tipos de datos](#Tipos-de-datos)
5. [Funciones](#Funciones)
6. [Declaraciones y Expresiones](#Declaraciones-y-Expresiones)
7. [Condiciones](#Condiciones)
8. [Bucles](#Bucles)

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
    let crab = '🦀';
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
Definimos una función en Rust ingresando fnseguido de un nombre de función y un conjunto de paréntesis. Los corchetes le dicen al compilador dónde comienza y termina el cuerpo de la función.

~~~rust
fn main() {
    hello("Alex"); //Llamar una función
    println!("La suma es {}", sum(10, 40))
}


//Función parametros
fn hello(name:&str){
    println!("Hola, {name}")
}

//Funciones con retorno
fn sum(a:i32, b :i32)->i32{
    return a + b; 
}
~~~

---
## Declaraciones y Expresiones 
Los cuerpos de las funciones se componen de una serie de sentencias que opcionalmente terminan en una expresión. Hasta ahora, las funciones que hemos cubierto no han incluido una expresión final, pero ha visto una expresión como parte de una instrucción. Debido a que Rust es un lenguaje basado en expresiones, es importante comprender esta distinción. Otros lenguajes no tienen las mismas distinciones, así que veamos qué son las declaraciones y expresiones y cómo sus diferencias afectan los cuerpos de las funciones.

- Declaraciones no devuelven valores y tienes punto y coma al final. 
- Las expresiones devuelven valores y no tienen punto y coma al final. 

~~~rust
    let result:bool; //Declaración
    let a = 10;
    let b = 5;

    //Aplicando expresiones anidadas
    result = (a*b-2*b) >= 20 && !((a % b) != 0);

    println!("Resultado: {result}");
~~~

Llamar a una función es una expresión. Llamar a una macro es una expresión. Un nuevo bloque de alcance creado con corchetes es una expresión, por ejemplo:

~~~rust
    let y = {
        let x = 3;
        x + 1
    };

    println!("El valor de y es: {y}");
~~~

En Rust, el valor de retorno de la función es sinónimo del valor de la expresión final en el bloque del cuerpo de una función. Puede regresar antes de una función usando la palabra clave `return` y especificando un valor, pero la mayoría de las funciones devuelven la última expresión implícitamente. He aquí un ejemplo de una función que devuelve un valor.

~~~rust
//Funciones con retorno
fn sum(a:i32, b :i32)->i32{
    a + b
}
~~~

---
## Condiciones 
Una expresión `if` le permite ramificar su código dependiendo de las condiciones. Proporcionas una condición y luego dices: “Si se cumple esta condición, ejecuta este bloque de código. Si no se cumple la condición, no ejecute este bloque de código”.

~~~rust
    let a = 18;

    if a > 0 {
        println!("El número es positivo");
    } else if a < 0 {
        println!("El número es positivo");
    }else{
        println!("El valor de a es {a}")
    }
~~~

### Uso if en una declaración let
Debido a que `if` es una expresión, podemos usarla en el lado derecho de una let declaración para asignar el resultado a una variable.

~~~rust
    let a = 10;
    let msm = if a % 2 == 0 {"PAR"}else{"IPAR"};
    
    println!("El valor de a es {msm}")
~~~

---
## Bucles

- La palabra clave `loop` le dice a Rust que ejecute un bloque de código una y otra vez para siempre o hasta que le digas explícitamente que se detenga.

~~~rust
    let mut c = 0;

    loop{
        if c == 10{
            break; //Rompe con el bucle
        }

        if c == 5{
            continue;//Salta a la siguiente
        }
        println!("Valor de c es {c}");
        c += 1;
    }
~~~

- Devolver valores de bucles

~~~rust
   let mut c = 0;

    let result = loop{
        c += 1;
        if c == 10{
            break c * 2; //Rompe con el bucle
        }
    };

    println!("Resultado: {result}")
~~~

- Etiquetas de bucle para desambiguar entre múltiples bucles

~~~rust
    let mut c = 0;
    'end: loop {
        println!("Contador = {c}");
        let mut r = 10;

        loop {
            println!("Restantes = {r}");
            if r == 9 {
                break;
            }
            if c == 2 {
                break 'end;
            }
            r -= 1;
        }

        c += 1;
    }
    println!("Fin de contador = {c}");
~~~

- Bucles condicionales con `while`

~~~rust
    let mut n = 3;

    while n != 0 {
        println!("{n}!");
        n -= 1;
    }

    println!("BOOM!!!");
~~~

- Bucle `for`

~~~rust
    let array = [10, 20, 30, 40, 50];

    for dato in array {
        println!("El valor es: {dato}");
    }
~~~

- For con rango 

~~~rust
    for dato in (1..4).rev(){
        println!("{dato}!")
    }
    println!("BOOOM!!!")
~~~






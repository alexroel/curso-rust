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

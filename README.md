# RUST (.rs)
 A repository referring to those made on the RUST language, and kinsley's Developments
 Welcome to my RUST repository, here you will find my main projects developed. I hope you enjoy!
# RUST Basic's
<h3>RUST's `Hello World`: </h3>

```rs
fn main() {
    println!("Hello, world!");
}
```
<h3>Variáveis e Mutabilidade</h3>

```rs
fn main() {
    let x = 5; // Imutável
    let mut y = 10; // Mutável

    println!("x = {}, y = {}", x, y);

    y = 20;
    println!("Agora y = {}", y);
}
```

<h3>Exemplos de tipos de variáveis</h3>

```rs
fn main (){
    let inteiro: i32 = 9999999999;
    let decimal: f64 = 3.14;
    let letra: char = 'A';
    let texto: &str = "Olá";
    let logico: bool = true;
    println!("{}-{}-{}-{}-{}",inteiro,decimal,letra,texto,logico);
}
```

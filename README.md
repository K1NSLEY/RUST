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

<h3>Interação com input</h3>

```rs
use std::io;

fn main (){
    let mut entrada = String::new();
    println!("Digite seu nome:");
    io::stdin().read_line(&mut entrada).unwrap();
    println!("Olá, {}", entrada.trim());
}
```

<h3>Input calcula Idade</h3>

```rs
use std::io;

fn main() {
    let mut idade = String::new();
    println!("Digite sua idade:");

    io::stdin()
        .read_line(&mut idade)
        .expect("Falha ao ler a entrada");

    let idade: u32 = idade.trim().parse().expect("Digite um número válido");

    if idade >= 18 {
        println!("Maior de idade");
    } else {
        println!("Menor de idade");
    }
}
```
<h3>Calculadora de Soma</h3>

```rs
fn soma(a:i32,b:i32) -> i32{
    a+b
}

fn main(){
    let resultado = soma(5,3);
    println!("Soma = {}", resultado);
}
```

<h3>Remoção de valores de Pilhas</h3>

```rs
fn main() {
    let mut pilha: Vec<i32> = Vec::new(); // cria uma pilha de inteiros

    pilha.push(10);
    pilha.push(20);
    pilha.push(30);

    println!("Pilha: {:?}", pilha);

    if let Some(valor) = pilha.pop() {
        println!("Removido da pilha: {}", valor);
    }

    println!("Pilha após remoção: {:?}", pilha);
}

```


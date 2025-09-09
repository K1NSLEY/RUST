fn main() {
    println!("---------------------------------");
    println!("Hello, just a basic guide to Rust");
    println!("---------------------------------");
    
    println!("---------------------------------");
    println!("           Var types");
    println!("---------------------------------");
    let x = 5; // Imutável
    let mut y = 10; // Mutável
    println!("x = {}, y = {}", x, y);
    y = 20; // Valor alterado
    println!("The Value of Y are changed = {}", y);

    let inteiro: i32 = 1;           // If too big will crash
    let decimal: f64 = 1.05;        // For Decimal numbers
    let letra: char = 'C';          // Only Characters
    let texto: &str = "Word";       // Only Words
    let logico: bool = false;
    println!("{} - {} - {} - {} - {}",inteiro,decimal,letra,texto,logico);

    println!("---------------------------------");
    println!("       Rust Concat");
    println!("---------------------------------");
    let nome1: &str = "Kinsley";
    let nome2: &str = "Eduardo";
    let nome3: &str = "Fisher";
    println!("Now {}, {} e {} are friends",nome1,nome2,nome3);
    
    
    println!("---------------------------------");
    println!("       Laces");
    println!("---------------------------------");
}

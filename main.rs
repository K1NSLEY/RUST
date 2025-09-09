fn main() {
    println!("-----------------------------------------------------");
    println!("         Hello, just a basic guide to Rust");
    
    println!("-----------------------------------------------------");
    println!("                   Var types");
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
    println!("-----------------------------------------------------");

    println!();
    println!("-----------------------------------------------------");
    println!("              Rust Concat");
    
    let mut nomes = vec!["Kinsley", "Eduardo", "Fisher"];
    println!("The {} are friends", nomes.join(", "));
    
    let nome1: &str = "Kinsley";
    let nome2: &str = "Eduardo";
    let nome3: &str = "Fisher";
    println!("Now {}, {} e {} now are enemies", nome1, nome2, nome3);
    println!("-----------------------------------------------------");
    
    
    println!();
    println!("-----------------------------------------------------");
    println!("              Repeat Loops (for)");
    println!("-----------------------------------------------------");
    for nome in &nomes {
        println!("Nome: {} na lista", nome)
    }
    
    println!();
    println!("-----------------------------------------------------");
    println!("             Interactions with batteries");
    println!("-----------------------------------------------------");
    //let mut nomes = vec!["Kinsley", "Eduardo", "Fisher"]; already exists
    nomes.push("Lucas");  // Adiciona "Lucas" à pilha
    nomes.pop();  // Remove o último elemento (topo da pilha)
    
    println!("A pilha está vazia? {}", nomes.is_empty());  // Verifica se a pilha está vazia
    
    for nome in &nomes {
        println!("Nome: {} na lista", nome)
    }
}

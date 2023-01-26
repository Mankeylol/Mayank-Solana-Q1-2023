pub fn run() {
    println!("Hello from the print.rs file");

    println!("{} is a {}", "Mankey", "Pokemon");

    println!("{0} is a {1} and {0} evovles into {2}", "Mankey", "Pokemon", "Primeape");

    println!(
        "{name} is {adjective}",
        name = "Solana",
        adjective = "Lightspeed"
      );

      println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

      println!("{:?}", (12, true, "hello"));

      println!("10 + 10 = {}", 10 + 10);
}  
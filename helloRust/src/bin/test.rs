use std::collections::HashMap;

fn main() {
   
let mut moneyheistcities = HashMap::new();
moneyheistcities.insert("spain","madrid");
moneyheistcities.insert("france","paris");
moneyheistcities.insert("italy","rome");
println!("Capital of France is {}", moneyheistcities["france"]);
println!("Capital of Italy is {}", moneyheistcities["italy"]);
}
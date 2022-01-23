fn main() {
    let color = std::env::args().nth(1).expect("Please provide a color");
    let animal = std::env::args().nth(2).expect("Please provide an animal");
    let friend = std::env::args().nth(3).expect("Please provide a friend's name");
    
    println!("Roses can be {}, {} could be blue, {} is open minded and so should be you.", color, animal, friend);
}

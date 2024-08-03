
#[derive(Debug, PartialEq, PartialOrd)]
struct Centemeters(f64);

#[derive(Debug)]
struct Inches(i32);

struct Sheep {}
struct Cow{}

trait Animal {
    fn noise(&self) -> &'static str; 
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "mooooh!"
    }
}
impl Inches {
    fn to_centemeters(&self)-> Centemeters{
        let &Inches(inches) = self;
        Centemeters(inches as f64 * 2.54)
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal>{
    if random_number < 0.5 {
        Box::new(Sheep{})
    } else {
        Box::new(Cow{})
    }
}
fn main() {
    let foot = Inches(12);
    println!("One foot equals {:?}", foot);
    let meter = Centemeters(100.0);

    let cmp = 
    if foot.to_centemeters() < meter {
        "smaller"
    } else {
        "bigger"
    };
    println!("One foot is {:p} than meter.", cmp);

    println!("===> Return Traits with dyn");

    let random_number = 0.7;
    let animal = random_animal(random_number);
    println!("You've randomly chose an animal, and it says {}", animal.noise());

    println!("===> Operator Overloading")
}

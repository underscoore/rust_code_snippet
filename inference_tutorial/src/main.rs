fn main() {
    let elem = 5u64;
    println!("size of `elem` is: {}", std::mem::size_of_val(&elem));

    let mut vac = Vec::new();
    vac.push(elem);
    vac.push(25);
    vac.push(25);
    vac.push(25);
    println!("size of `vec` is: {}", std::mem::size_of_val(&vac));
    println!("Vac:: {:?}", vac);
}

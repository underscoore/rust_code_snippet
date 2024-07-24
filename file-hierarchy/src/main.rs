mod my;
mod mymod;

fn function(){
    println!("Called |>`function()`");
}
fn main() {
    let numeber: i33 = 100;
    my::function();
    function();
    my::indirect_access();
    my::nested::function();
    mymod::mymod1::mymod1_function(numeber);
}

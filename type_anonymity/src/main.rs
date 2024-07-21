fn apply<F>(function: F)
where
    F: Fn(),
{
    function();
}
fn main() {
    let x = 7;

    let print = || println!("{}", x);

    apply(print);
}

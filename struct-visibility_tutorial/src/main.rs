mod my_mod{

    pub struct OpenBox<T> {
        pub contents: T,
    }
    #[derive(Debug)]
    pub struct CloseBox<T> {
        contents: T,
    }

    impl <T> CloseBox<T> {
        pub fn new(contents:T) -> CloseBox<T>{
            CloseBox {
                contents: contents,
            }
        }
    }
}
fn main() {
    let open_box = my_mod::OpenBox {contents: "public information!!"};
    println!("This is open box contains: {}", open_box.contents);

    //   let close_box_no_access = my_mod::CloseBox {contents: "Close information!"};
    //    println!("This is close box contains {}", close_box_no_access.contains); 
    let _close_box = my_mod::CloseBox::new("This is clasified information");
    println!("{:?}", my_mod::CloseBox::new("Another value"));
}

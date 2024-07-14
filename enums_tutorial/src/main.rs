#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddrKind2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrKind3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
struct Ipv4Addr{
    address: String,
}

#[derive(Debug)]
struct Ipv6Addr{
    address: (u8, u8, u8, u8),
}

#[derive(Debug)]
enum IpAddrKind4{
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// Create an 'enum' to clessify a web event. Note how both
// name and type information togehter specify the varient:
// 'PageLoad != PageUnload' and 'KeyPress(char) != Paste(String)'
// Each is different and independent.
enum WebEvents {
    // An 'enum' varient may either be 'unit-link',
    PageLoad,
    PageUnload,
    // Link tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-link structure
    Click{x: i64, y:i64}
}

// A function which takes a 'WebEvent' enum as an argument
// return nothing
fn inspect(event: WebEvents){
    match event {
        WebEvents::PageLoad => println!("page loaded"),
        WebEvents::PageUnload => println!("page unloaded"),
        //Destructure 'c' from inside the 'enum' variant.
        WebEvents::KeyPress(c) => println!("pressed '{}'", c),
        WebEvents::Paste(s) => println!("pasted \"{}\"", s),
        //Destructure 'Click' into 'x' and 'y'.
        WebEvents::Click {x,y} => {
            println!("clicked at x = {}, y={}", x,y)
        },
    }
}

        fn main() {
            let home_address = IpAddr {
                kind: IpAddrKind::V4,
                address: String::from("127.0.0.1"),
            };

            let loopback_address = IpAddr {
                kind: IpAddrKind::V6,
                address: String::from("::1"),
            };
            println!("{:?}", home_address);
            println!("{:?}", loopback_address);
            println!("\n=== Enum IP address of kind 2 ===");

            let home_address2 = IpAddrKind2::V4(String::from("127.0.0.1"));
            let loopback_address2 = IpAddrKind2::V6(String::from("::1"));
            println!("{:?}", home_address2);
            println!("{:?}", loopback_address2);
            println!("{:?}", IpAddrKind2::V4("121212".to_string()));
            println!("{:?}", IpAddrKind3::V4(127,0,0,1));
            println!("{:?}", IpAddrKind3::V6("127.0.0.2".to_string()));
            println!("{:?}", IpAddrKind3::V4(127,0,0,1));
            println!("\n=== Enum IP Address of Kind 3");
            println!("{:?}", IpAddrKind4::V4(Ipv4Addr {address: "127.0.0.3".to_string()}));
            println!("{:?}", IpAddrKind4::V6(Ipv6Addr {address: (127,0,0,4)}));

            println!("\n\n=== Enums By Examples ===");

            let pressed = WebEvents::KeyPress('x');
            let pasted = WebEvents::Paste("My text".to_owned());
            let click = WebEvents::Click {x: 20, y: 80};
            let load = WebEvents::PageLoad;
            let unload = WebEvents::PageUnload;

            inspect(pressed);
            inspect(pasted);
            inspect(click);
            inspect(load);
            inspect(unload);
        }

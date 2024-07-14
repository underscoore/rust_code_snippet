#[warn(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    signed_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32, i32);

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, i32);

struct Point{
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Immutable instace of struct defination
    let user1 = User {
        active: true,
        username: String::from("Unique UserName"),
        email: String::from("someemail@example.com"),
        signed_in_count: 10,
    };
    println!("Here are the user details 
             \nUsername: {0} 
             \nUser Email: {1}
             \nIs this user is active: {2}", 
             user1.username,
             user1.email,
             user1.active);

    let mut user2 = User {
        active: true,
        username: String::from("Unique UserName"),
        email: String::from("someemail@example.com"),
        signed_in_count: 10,
    };
    println!("Here are the user details 
             \nUsername2: {0} 
             \nUser Email: {1}
             \nIs this user is active: {2}", 
             user2.username,
             user2.email,
             user2.active);

    user2.email = String::from("updatedemail@example.com");
    user2.active = false;
    println!("Here are the updated user details 
             \nUsername2: {0} 
             \nUser Email: {1}
             \nIs this user is active: {2}", 
             user2.username,
             user2.email,
             user2.active);

    let mut user3 = build_user("userbuilder1@exmple.com".to_string(), "UserBuilder1".to_string());
    println!("Here  is the user 1 creaed by usind user builder 
             \nUsername2: {0} 
             \nUser Email: {1}
             \nIs this user is active: {2}", 
             user3.username,
             user3.email,
             user3.active);
    user3.active = user2.active;
    println!("Here  is the user 1 creaed by usind user builder 
             \nUsername2: {0} 
             \nUser Email: {1}
             \nIs this user is active: {2}", 
             user3.username,
             user3.email,
             user3.active);

    let user4 = User {
        email: String::from("another@example.com"),
        ..user3

    };
    println!("Here  is the user 4 creaed by usind user builder 
             \nUsername4: {0} 
             \nUser Email: {1}
             \nIs this user is active: {2}", 
             user4.username,
             user4.email,
             user4.active);
    println!("=== Tuple Struct ===");

    let black = Color(4,0,0,0);
    println!("{:?}", black);
    println!("\n\n=== Sruct By Exmaple ===");

    let name = String::from("Ajay");
    let age = 29;
    let ajay = Person{name, age};
    println!("{:?}", ajay);
    println!("My name is:{}, and my age is {}", ajay.name, ajay.age);

    // Instantiate a 'Point
    let point = Point{x:1.1, y:1.2};
    let another_point = Point{x:2.1, ..point};
    println!("X = {}, Y = {}", another_point.x, another_point.y);

    let pair = Pair(1,10);
    let Pair(first_number, second_number) = pair;
    println!("First Number is: {first_number}, Second Number is: {second_number}");
    println!("Area of rectangle");

    let area1 = rect_area(point.x, point.y);
    println!("Area is: {area1}");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        signed_in_count: 1,
    }
}

fn rect_area(s1:f32, s2:f32) -> f32{
    let area = s1*s2;
    area
}

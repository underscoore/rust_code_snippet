mod concepts;
use concepts::raii::create_box;
use std::ptr;
struct Pointt{
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    // Allocate an integer on the heap
    let mut point = Pointt{x: 10, y:20, z:30}; 

    // A nested scope;
    {
        // Allocate an integer on heap
        let _box3 = Box::new(7i32);
        println!("{_box3}");
        // _box3 is destroyed here, and momory got freed here.
    }
    //creating lots of boxes just of fun;
    for _ in 0u32..1000 {
        create_box();     
    }


    let ref borrow_point= &point;
    let other_point = &point;

    //    let mut mut_pointer = &mut point; // you can not mutably borrow value because is imutably shard
    let address = ptr::addr_of!(borrow_point);
    let address2 = ptr::addr_of!(other_point);
    let address3 = ptr::addr_of!(point);
    println!("{}, {}, {}", borrow_point.x, other_point.y, borrow_point.z);
    println!("{:?}", address); 
    println!("{:?}", address2); 
    println!("{:?}", address3); 
    println!("{:?}", address3); 
    println!("{}, {}, {}", borrow_point.x, other_point.y, borrow_point.z);
    // lets borrow mutable
    let _mutable_borrow= &mut point; // value is been used and borrowed referecne is now freed

    let point2 = point;
    let address4 = ptr::addr_of!(point2);
    println!("{:?}", address4); 

    let val: i32 = 10;
    let ref_a = &val;
    println!("{}", ref_a);

    let val_a: i32 = 10;
    let address_of_val_a = &val_a;
    println!("Address of value val_a: {:p}", address_of_val_a);
    let string: String = "10".to_string(); 
    let address_of_val_a = &string;
    println!("Address of value string: {:p}", address_of_val_a);

    let mut new_value: i32 = 1000;
    let borrow_new_value = &new_value;
    //let mut mut_new_value = &mut new_value;// here is the error cause the borrowed value is used
    // afte this statemet :>
    println!("{}", borrow_new_value);
}

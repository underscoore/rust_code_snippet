fn main() {
    let mut array:[i32; 3] = [0; 3];

    println!("{:?}", array);

    array[1]=1;
    array[2]=2;
    println!("{:?}", array);
    //jprintln!("{}", array[1]);
    assert_eq!([1,2], &array[1..]);

    for x in  array {
        print!("{x} ");
    }

    // looping array by using reference
    for x in &array {
        print!("{x} ");
    }

    // You can use <ArrayType:: try_from(slice) or slce.try_into() to get an array from a slice:

    let bytes: [u8; 3] = [1, 0, 2];
    let res1 = assert_eq!(1, u16:: from_le_bytes(<[u8; 2]>::try_from(&bytes[0..2]).unwrap()));
    let res2 = assert_eq!(512, u16::from_le_bytes(bytes[1..3].try_into().unwrap()));

    println!("Result 1: {:?}, Result 2: {:?}", res1, res2);

    //You can move a slice to move elements out of the array
    fn move_away(string : String){
        println!("Moved String {}", string);

    }

    let [john, roa] = ["John".to_string(), "Rao".to_string()];
    move_away(john);
    move_away(roa);

    //Arrays can be created from homogeneous tuples of appropriate lenght:
    let tuple: (u32, u32, u32) = (1, 2, 3);
    let array1: [u32; 3] = tuple.into();

    println!("Array created from Tuple: {:?}", array1);


    //Iterate Arrays
    let array2: [i32; 3] = [4, 5, 6]; //her [0, 3] means that every item of this array will be 0
                                      // till the leght of an array which is 3

                                      //this way is of 2015 
    for item in array2.into_iter().enumerate(){
        let (i, x): (usize, i32) = item; // there is the error in the offecial document
        println!("array[{i}] = {x}");
    }

    // this way is of 2018

    println!("____________________________");
    for item in array2.iter().enumerate(){
        let (i, x): (usize, &i32) = item;
        println!("array[{i}] = {x}");
    }

    for item in array2{
        println!("{item}");
    }

println!("_________________________");
    for item in &array2[0..2]{
        println!("{item}");
    }  
}

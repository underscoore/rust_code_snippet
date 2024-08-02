// raii.rs

pub fn create_box(){
    // Allocalte an integer in heap memory
   let _box1 = Box::new(3_i32);
    // _box1 is destroyed here, and memory gets freed
}

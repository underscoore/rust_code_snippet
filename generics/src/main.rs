mod operations;
mod sorting;
use operations::max::get_max;
use operations::min::get_min;
use operations::n_sorting::{asc_sorting, dsc_sorting};
use operations::implx;
mod data;
use sorting::sorting::{d_sorting, a_sorting};
fn main() {
    println!("==> This is for MAX");
    println!("{}", get_max(&data::get_small_list()));
    println!("{}", get_max(&data::get_long_list()));
    println!("{}", get_max(&data::get_float_list()));
    println!("{}", get_max(&data::get_char_list()));
    println!("==> This is for MIN");
    println!("{}", get_min(&data::get_small_list()));
    println!("{}", get_min(&data::get_long_list()));
    println!("{}", get_min(&data::get_float_list()));
    println!("{}", get_min(&data::get_char_list()));
    println!("==> This is for SORTING-DSC");
    println!("{:?}", dsc_sorting(&data::get_small_list()));
    println!("{:?}", dsc_sorting(&data::get_long_list()));
    println!("{:?}", dsc_sorting(&data::get_float_list()));
    println!("{:?}", dsc_sorting(&data::get_char_list()));
    println!("{:?}", dsc_sorting(&data::get_vlong_list()));
    println!("==> This is for SORTING-ASC");
    println!("{:?}", asc_sorting(&data::get_small_list()));
    println!("{:?}", asc_sorting(&data::get_long_list()));
    println!("{:?}", asc_sorting(&data::get_float_list()));
    println!("{:?}", asc_sorting(&data::get_char_list()));
    println!("{:?}", asc_sorting(&data::get_vlong_list()));
    println!("==> Sorting using structs Impl");
    println!("{:?}", d_sorting(&data::list()));
    println!("{:?}", a_sorting(&data::list()));
    
    // Initailize the value of structs
    let x = implx::Val{val: 3.33};
    let y = implx::GenVal{gen_val: 4i32};

    println!("The value of VAL struct: {}", x.value());
    println!("This value of GENVAL struct: {}", y.value());
}

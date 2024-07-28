mod operations;
use operations::max::get_max;
use operations::min::get_min;
use operations::sorting::{asc_sorting, dsc_sorting};
mod data;
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

}

use crate::rust_code::{print_input, Status};
use std::os::raw::{c_int, c_uint};

#[repr(C)]
pub struct Data {
    pub values: *const c_uint,
    pub values_len: c_int,
    pub status: Status,
}

#[no_mangle]
pub unsafe extern "C" fn print_c(input: *const Data) {
    if input.is_null() {
        println!("Invalid input");
        return;
    }

    let input = &*input;
    let values = std::slice::from_raw_parts(input.values, input.values_len as usize);
    let values = values.iter().map(|&x| x).collect::<Vec<_>>();
    let status = &input.status;
    let rust_data = crate::rust_code::Data {
        values,
        status: *status,
    };
    print_input(&rust_data);
}

#[cfg(test)]
mod tests {
    use crate::c_api::{print_c, Data};
    use crate::rust_code::Status;
    use std::os::raw::{c_int};
    #[test]
    fn test_print_c() {
        // Create a sample input Data struct
        let values: [u32; 4] = [1, 2, 3, 4];
        let input = Data {
            values: values.as_ptr(),
            values_len: values.len() as c_int,
            status: Status::Ok,
        };

        // Call the print_c function from Rust
        unsafe {
            print_c(&input);
        }
    }
}

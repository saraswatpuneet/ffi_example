#[derive(Debug)]
pub struct Data {
    pub values: Vec<u32>,
    pub status: Status,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum Status {
    OK,
    Error,
}

pub fn print_input(input: &Data) {
    println!("Input: {:?}", input);
}

#include <stdio.h>
#include "ffi_example.h"

int main() {
    // Create a sample Data struct
    Data data;
    data.values = (uint32_t[]){1, 2, 3};
    data.values_len = 3;
    data.status = Ok;

    // Call the print_input function from Rust
    print_c(&data);

    return 0;
}

#![no_main]
wp1_zkvm::entrypoint!(main);

extern "C" {
    fn syscall_bls12381_fp2_mul(p: *mut u32, q: *const u32);
}

pub fn main() {
    let a: [u8; 96] = [
        137, 214, 20, 51, 154, 5, 119, 78, 103, 112, 15, 10, 160, 81, 219, 107, 10, 97, 141, 56,
        183, 221, 124, 243, 118, 41, 236, 122, 149, 40, 29, 6, 119, 244, 21, 26, 40, 203, 20, 28,
        8, 39, 167, 1, 125, 45, 134, 18, 98, 253, 223, 82, 158, 161, 179, 84, 30, 122, 142, 227,
        46, 167, 60, 30, 129, 1, 79, 174, 115, 21, 175, 43, 2, 206, 23, 252, 188, 97, 51, 163, 2,
        236, 163, 189, 114, 56, 183, 105, 10, 193, 201, 137, 107, 55, 152, 22,
    ];
    let b: [u8; 96] = [
        32, 226, 105, 47, 227, 133, 211, 239, 149, 35, 250, 248, 46, 146, 20, 143, 195, 201, 170,
        108, 195, 100, 248, 55, 45, 167, 243, 50, 253, 214, 24, 100, 88, 92, 45, 128, 153, 168, 99,
        203, 98, 55, 228, 148, 210, 50, 87, 22, 112, 255, 46, 135, 164, 189, 141, 245, 248, 44,
        117, 120, 42, 57, 123, 53, 117, 50, 167, 211, 70, 151, 85, 56, 214, 199, 47, 194, 141, 15,
        17, 153, 151, 37, 100, 62, 56, 114, 54, 82, 102, 182, 224, 32, 165, 225, 91, 20,
    ];
    let mul: [u8; 96] = [
        101, 231, 36, 124, 231, 42, 239, 255, 92, 206, 46, 150, 82, 142, 221, 160, 46, 98, 250,
        248, 105, 216, 148, 36, 107, 51, 144, 124, 125, 244, 118, 252, 214, 172, 88, 119, 252, 70,
        59, 177, 72, 114, 154, 244, 188, 253, 177, 4, 142, 253, 57, 227, 130, 57, 79, 220, 86, 120,
        110, 82, 151, 187, 34, 83, 245, 177, 253, 54, 35, 222, 71, 204, 132, 199, 133, 45, 243, 59,
        242, 110, 211, 252, 222, 84, 80, 48, 208, 240, 152, 8, 192, 24, 85, 171, 6, 23,
    ];

    let mut out = a.clone();
    unsafe {
        syscall_bls12381_fp2_mul(out.as_mut_ptr() as *mut u32, b.as_ptr() as *const u32);
    }
    assert_eq!(out, mul);

    println!("done");
}

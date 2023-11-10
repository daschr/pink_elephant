use std::mem::MaybeUninit;
use std::ptr::{read_volatile, write_volatile};
use std::thread;

struct U64(*mut u64);
unsafe impl Send for U64 {}

const ALT: [u64; 2] = [0xf0f0_f0f0_f0f0_f0f0u64, 0x0f0f_0f0f_0f0f_0f0fu64];
fn writer(v: U64) {
    let v: *mut u64 = v.0;

    let mut i = 0;
    loop {
        unsafe { write_volatile(v, ALT[i]) };
        i ^= 1;
    }
}

fn main() {
    let mut v = MaybeUninit::new(0u64);

    let v_c = U64(v.as_mut_ptr());
    thread::spawn(move || writer(v_c));

    let v_o: *mut u64 = v.as_mut_ptr();

    loop {
        let temp = unsafe { read_volatile(v_o) };
        if temp != ALT[0] && temp != ALT[1] {
            println!("{:016x}", temp);
        }
    }
}

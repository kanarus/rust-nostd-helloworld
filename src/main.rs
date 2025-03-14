#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn __libc_start_main() {
    _start();
}

#[unsafe(export_name = "main")]
pub extern "C" fn _start() {
    let message = "Hello, world!\n";

    unsafe {core::arch::asm!(
        "syscall",
        in("rax") 1, /* write */
        in("rdi") 1, /* stdout */
        in("rsi") message.as_ptr(),
        in("rdx") message.len(),
    )};

    unsafe {core::arch::asm! {
        "syscall",
        in("rax") 60, /* exit */
        in("rdi") 0, /* status */
    }};
}

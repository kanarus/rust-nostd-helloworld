#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
fn __libc_start_main() {
    _main();
}

#[unsafe(export_name = "main")]
fn _main() {
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

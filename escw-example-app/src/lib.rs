#![no_std]
#![no_main]

#[cfg(feature = "io")]
mod io;

#[cfg(feature = "uart")]
mod uart;

#[no_mangle]
pub extern "C" fn app() {
    #[cfg(feature = "io")]
    io::io_example();

    #[cfg(feature = "uart")]
    uart::uart_example();

    loop {
        #[cfg(feature = "io")]
        io::io_example_tick();

        #[cfg(feature = "uart")]
        uart::uart_example_tick();
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

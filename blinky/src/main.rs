#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    Board,
    hal::timer::Timer,
};
use embedded_hal::{delay::DelayNs, digital::OutputPin};

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    // initialization
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let blink_delay_ms: u32 = 500;

    board.display_pins.col3.set_low().unwrap();
    loop {
        // application logic

        // 1) turn the led on,
        board.display_pins.row3.set_high().unwrap();


        // 2) sleep
        timer.delay_ms(blink_delay_ms);
        

        // 3) turn the led off
        board.display_pins.row3.set_low().unwrap();


        // 4) sleep
        timer.delay_ms(blink_delay_ms);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    Board,
    display::blocking::Display,
    hal::timer::Timer
};
use embedded_hal::delay::DelayNs;

#[entry]
fn main() -> ! {
    // initialization
    let M : [[u8; 5];5] = [
         [1, 0, 0, 0, 1],
         [1, 1, 0, 1, 1],
         [1, 0, 1, 0, 1],
         [1, 0, 0, 0, 1],
         [1, 0, 0, 0, 1]
    ];

    let U: [[u8;5];5] = [
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0]
    ];

    let Z: [[u8;5];5] = [
        [1, 1, 1, 1, 1],
        [0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [1, 1, 1, 1, 1]
    ];

    let A: [[u8;5];5] = [
        [0, 0, 1, 0, 0],
        [0, 1, 0, 1, 0],
        [0, 1, 1, 1, 0],
        [0, 1, 0, 1, 0],
        [0, 1, 0, 1, 0]
    ];

    let H:[[u8;5];5] = [
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1]
    ];

    let I:[[u8;5];5] = [
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0]
    ];

    let R:[[u8;5];5] = [
        [0, 1, 1, 1, 0],
        [0, 1, 0, 0, 1],
        [0, 1, 1, 1, 0],
        [0, 1, 0, 1, 0],
        [0, 1, 0, 0, 1]
    ];
    let delay_ms : u32 = 2000;
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    loop {
        // application logic

        display.show(&mut timer, M, delay_ms);
        display.clear();

        display.show(&mut timer, U, delay_ms);
        display.clear();

        display.show(&mut timer, Z, delay_ms);
        display.clear();

        display.show(&mut timer, A, delay_ms);
        display.clear();

        display.show(&mut timer, H, delay_ms);
        display.clear();

        display.show(&mut timer, I, delay_ms);
        display.clear();

        display.show(&mut timer, R, delay_ms);
        display.clear();

        timer.delay_ms(delay_ms + delay_ms);
    }

}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {

    loop{}
}
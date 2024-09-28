#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::hal::{gpio::Level, pac::CorePeripherals, saadc::Saadc, saadc::SaadcConfig, timer::Timer};
use microbit::{adc::Adc, Board, Peripherals};
use embedded_hal::{delay::DelayNs, digital::OutputPin}; 

//use defmt::*;
//use defmt_rtt as _;
//use rtt_target::rtt_init;
//use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {

    rtt_init_print!();
    rprintln!("Log Added 1");

    // Get access to the device peripherals
    let peripherals = Peripherals::take().unwrap();

    rprintln!("Log Added 1.7");
    let core_peripherals = CorePeripherals::take().unwrap();

    rprintln!("Log Added 2");

    // Initialize the board (pins, i2c, etc.)
    let board = Board::new(peripherals, core_peripherals);
    //let mut board = Board::take().unwrap();

    rprintln!("Log Added 2.5");

    // Set up mic_run pin as an output (initially turn on the microphone)
    let mic_run = board
        .microphone_pins
        .mic_run
        .into_push_pull_output(Level::High);
    
    rprintln!("Log Added 3");

    // Set up mic_in pin as an analog input
    let mut mic_in = board.microphone_pins.mic_in;

    // Create an ADC to read the analog value from the mic_in pin
    rprintln!("Log Added 4");
    let saadc_config = SaadcConfig::default();
    let mut adc = Adc::new(board.ADC, saadc_config);
    rprintln!("Log Added 5");
    //let saadc = Saadc::new(peripherals.SAADC, saadc_config);

    let mut timer = Timer::new(board.TIMER0);
    let blink_delay_ms: u32 = 500;
    
    loop {
        // Read the microphone value from the mic_in pin
        let mic_value = adc.read_channel(&mut mic_in).unwrap();

        // Print the microphone value
        rprintln!("Microphone value: {}", mic_value);
        timer.delay_ms(blink_delay_ms);
        // Optionally, you could turn off the microphone by setting mic_run to Low
        // mic_run.set_low().unwrap();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
#![no_std]
#![no_main]
#![allow(unused_parens)]

use cortex_m_rt::entry;
use microbit::gpio::DisplayPins;
use microbit::hal::{gpio::Level, pac::CorePeripherals, saadc::SaadcConfig, timer::Timer};
use microbit::{adc::Adc, Board, Peripherals};
use embedded_hal::digital::OutputPin; 

use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {

    rtt_init_print!();

    // Get access to the device peripherals
    let peripherals = Peripherals::take().unwrap();
    let core_peripherals = CorePeripherals::take().unwrap();

    // Initialize the board (pins, i2c, etc.)
    let mut board = Board::new(peripherals, core_peripherals);

    // init led pin to low
    init_gpio_pins(&mut board);

    let mut display_pins = board.display_pins;

    // Set up mic_run pin as an output (initially turn on the microphone)
    let mic_run = board
        .microphone_pins
        .mic_run
        .into_push_pull_output(Level::High);

    // Set up mic_in pin as an analog input
    let mut mic_in = board.microphone_pins.mic_in;

    // Create an ADC to read the analog value from the mic_in pin
    let saadc_config = SaadcConfig::default();
    let mut adc = Adc::new(board.ADC, saadc_config);
    
    let threshold: i16 = 400;

    // Get instance to Display to display led pattern
    let mut timer: Timer<microbit::pac::TIMER0> = Timer::new(board.TIMER0);
    let delay_ms : u32 = 400;
    let mut show_pattern : bool = false;
    let mut last_detection_time = 0;

    timer.start(4294967295);            // maximul u32 value

    loop {
        // Read the microphone value from the mic_in pin
        let mic_value = adc.read_channel(&mut mic_in).unwrap();
        let elasped_timer_ms = timer.read()/1000;

        if(mic_value > threshold && elasped_timer_ms - last_detection_time > delay_ms)
        {
            // Print the microphone value
            rprintln!("Microphone value: {}", mic_value);

            show_pattern = !show_pattern;

            turn_led(&mut display_pins, show_pattern);
            last_detection_time = elasped_timer_ms;
        }

        if (timer.reset_if_finished())
        {
            timer.start(4294967295);            // if timer has stopped, start again
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn init_gpio_pins(board: &mut Board)
{
    board.display_pins.col2.set_low().unwrap();
    board.display_pins.col3.set_low().unwrap();
    board.display_pins.col4.set_low().unwrap();
}

fn turn_led(display_pins: &mut DisplayPins, on:bool)
{
    if(on)
    {
        display_pins.row3.set_high().unwrap();
    }
    else 
    {
        display_pins.row3.set_low().unwrap();
    }
}
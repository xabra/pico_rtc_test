#![no_std]
#![no_main]

// ----- PICO REAL TIME CLOCK TEST -----

use rp_pico::entry;
use panic_halt as _;
use rp_pico::hal;
use rp_pico::hal::pac;
use rp_pico::hal::prelude::*;
use hal::rtc::{RealTimeClock, DateTime, DayOfWeek};

use defmt::*;
use defmt_rtt as _;

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The delay object lets us wait for specified amounts of time (in milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Make a DateTime struct to init the RTC
    let initial_date_time = DateTime {
        year: 2022,
        month: 10,
        day: 7,
        day_of_week: DayOfWeek::Friday,
        hour: 23,
        minute: 30,
        second: 0,
    };
    
    info!("init time:   {:?},  {:?},  {:?},  {:?}, ",  initial_date_time.day, initial_date_time.hour, initial_date_time.minute, initial_date_time.second);
    let rtc =  RealTimeClock::new(pac.RTC, clocks.rtc_clock , &mut pac.RESETS, initial_date_time).expect("ERROR IN NEW RTC");
    delay.delay_us(14); // <-- DELAYS LESS THAN 14 us PRODUCE ERRONEOUS RESULTS
    let now = rtc.now().expect("Error in RTC now");
    info!("now:   {:?},  {:?},  {:?},  {:?}, ",  now.day, now.hour, now.minute, now.second);

    loop {
        // forever
    }

}
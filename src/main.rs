//! # Laminator Controller
//!
//! Runs the custom laminator controller powered by the Pico W
//!
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// Test git

static FILTER_TAPS: [f32; 128] = [   
-0.00011294256402843338,
0.00017718671265226185,
0.00020801061201477204,
0.0002899170040148084,
0.0004063655844640212,
0.0005521611053486371,
0.0007254597239672028,
0.0009248212082829229,
0.001148106653482973,
0.0013919601852818227,
0.0016516431977742386,
0.0019208891413295141,
0.0021918551635332446,
0.0024551742346868677,
0.002700483739684856,
0.002916561390501056,
0.003089925141212581,
0.0032087638380859074,
0.0032599349126737785,
0.003231423508752996,
0.003112019328728966,
0.002892198743043144,
0.0025646907935557976,
0.002125014698392097,
0.001572026115204269,
0.0009082893285912567,
0.00014047573064284555,
-0.0007203211785275121,
-0.0016583570565608208,
-0.0026534074303888582,
-0.0036809191669052426,
-0.004711811404195509,
-0.005713740452228984,
-0.006651054659180601,
-0.007486025398180753,
-0.008179490087742778,
-0.00869195012460169,
-0.008984692836005809,
-0.009020949374783553,
-0.008767176908596474,
-0.008194171750384047,
-0.007278214858428536,
-0.0060021086630750635,
-0.004356158957522061,
-0.0023388318614019164,
0.00004276105020527881,
0.002772306097731268,
0.005824492740407066,
0.009164925387553738,
0.01275077916105693,
0.01653138572993423,
0.02044919469237556,
0.024440958747676503,
0.02843904665391764,
0.03237309324897252,
0.036171574061218695,
0.039763569407856865,
0.043080473306350266,
0.04605776511390201,
0.04863664974318549,
0.050765512749573256,
0.0524013735705554,
0.05351089109779341,
0.054071402262888014,
0.054071402262888014,
0.05351089109779341,
0.0524013735705554,
0.050765512749573256,
0.04863664974318549,
0.04605776511390201,
0.043080473306350266,
0.039763569407856865,
0.036171574061218695,
0.03237309324897252,
0.02843904665391764,
0.024440958747676503,
0.02044919469237556,
0.01653138572993423,
0.01275077916105693,
0.009164925387553738,
0.005824492740407066,
0.002772306097731268,
0.00004276105020527881,
-0.0023388318614019164,
-0.004356158957522061,
-0.0060021086630750635,
-0.007278214858428536,
-0.008194171750384047,
-0.008767176908596474,
-0.009020949374783553,
-0.008984692836005809,
-0.00869195012460169,
-0.008179490087742778,
-0.007486025398180753,
-0.006651054659180601,
-0.005713740452228984,
-0.004711811404195509,
-0.0036809191669052426,
-0.0026534074303888582,
-0.0016583570565608208,
-0.0007203211785275121,
0.00014047573064284555,
0.0009082893285912567,
0.001572026115204269,
0.002125014698392097,
0.0025646907935557976,
0.002892198743043144,
0.003112019328728966,
0.003231423508752996,
0.0032599349126737785,
0.0032087638380859074,
0.003089925141212581,
0.002916561390501056,
0.002700483739684856,
0.0024551742346868677,
0.0021918551635332446,
0.0019208891413295141,
0.0016516431977742386,
0.0013919601852818227,
0.001148106653482973,
0.0009248212082829229,
0.0007254597239672028,
0.0005521611053486371,
0.0004063655844640212,
0.0002899170040148084,
0.00020801061201477204,
0.00017718671265226185,
-0.00011294256402843338
];
  

// The macro for our start-up function
use rp_pico::entry;

// GPIO traits
use embedded_hal::digital::v2::OutputPin;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;
use rp_pico::hal::rom_data::float_funcs::fmul;

use defmt::*;
use defmt_rtt as _;

/// Entry point
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
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

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set the pin to be an output
    let mut test_pin = pins.gpio8.into_push_pull_output();



    // Blink the LED at 1 Hz
    loop {
        test_pin.set_high().unwrap();

        //delay.delay_us(1);
        let sum = array_square_32_fmul(FILTER_TAPS);
        //let sum = 0.0_f32; //array_square_64_nothing(filter_taps);

        test_pin.set_low().unwrap();

        if sum>1.0 {
            info!("DATA:  {:?}", sum);
        }
        

        delay.delay_us(1);
    }
}

fn array_square_32(a: [f32;128]) -> f32{
    let mut sum = 0.0_f32;
    for i in 0..128 {
        sum += a[i]*a[i];
    }
    sum
}

fn array_square_64(a: [f64;128]) -> f32{
    let mut sum = 0.0_f64;
    for i in 0..128 {
        sum += a[i]*a[i];
    }
    sum as f32
}

fn array_square_64_nothing(a: [f64;128]) -> f32{
    (a[60] as f32) *3.2345
}

fn array_square_32_fmul(a: [f32;128]) -> f32{
    let mut sum = 0.0_f32;
    for i in 0..64 {
        sum += fmul(a[i], a[i]);
    }
    sum
}

fn array_square_64_fmul(a: [f64;128]) -> f32{
    let mut sum = 0.0_f64;
    for i in 0..128 {
        sum += a[i]*a[i];
    }
    sum as f32
}

// End of file

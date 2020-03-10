#![no_std]
#![no_main]

extern crate panic_halt;

use atsamd10c14a as samd;
use cortex_m::peripheral::syst;
use cortex_m::peripheral::Peripherals as CortexPeripherals;
use cortex_m_rt::entry;
use cortex_m_rt::exception;

const LED: u32 = 5;

#[entry]
fn main() -> ! {
    let cortex_peripherals = CortexPeripherals::take().unwrap();
    let mut systick = cortex_peripherals.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(10_000);
    systick.clear_current();
    systick.enable_counter();

    loop {}
}

#[exception]
fn SysTick() {
    samd::Peripherals::take()
        .unwrap()
        .PORT
        .outtgl0
        .write(|w| unsafe { w.bits(1 << LED) });
}

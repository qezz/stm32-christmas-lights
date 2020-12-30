#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_default;

use stm32f1xx_hal as hal;
use ws2812_spi as ws2812;

use hal::delay::Delay;
use hal::pac;
use hal::prelude::*;
use hal::spi::Spi;
use ws2812::Ws2812;
use cortex_m::peripheral::Peripherals;

use smart_leds::{SmartLedsWrite, RGB8};

use cortex_m_rt::entry;

const _THE_LEN: usize = 31;
const DELAY: u16 = 700;
const BRIGHTNESS: u8 = 0x11;

const RED: RGB8 = RGB8 {
    r: BRIGHTNESS,
    g: 0,
    b: 0,
};

const GREEN: RGB8 = RGB8 {
    r: 0,
    g: BRIGHTNESS,
    b: 0,
};

const BLUE: RGB8 = RGB8 {
    r: 0,
    g: 0,
    b: BRIGHTNESS,
    };

fn rolling(data: &mut [RGB8; _THE_LEN], step: usize, in_reverse: bool) {
    let (p1, p2, p3) =
        if in_reverse {
            match step {
                0 => (RED, GREEN, BLUE),
                1 => (GREEN, BLUE, RED),
                2 => (BLUE, RED, GREEN),
                _ => (RED, GREEN, BLUE)
            }
        } else {
            match step {
                0 => (RED, GREEN, BLUE),
                1 => (BLUE, RED, GREEN),
                2 => (GREEN, BLUE, RED),
                _ => (RED, GREEN, BLUE)
            }
        };

    for x in (0..(data.len()-1)).step_by(3) {
        data[x] = p1;
        data[x+1] = p2;
        data[x+2] = p3;
    }
}

#[entry]
fn main() -> ! {
    rtt_init_default!();

    if let (Some(dp), Some(cp)) = (pac::Peripherals::take(), Peripherals::take()) {
        // Take ownership over the raw flash and rcc devices and convert them into the corresponding
        // HAL structs
        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();

        // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
        // `clocks`
        let clocks = rcc
            .cfgr
            .sysclk(48.mhz())
            .pclk1(24.mhz())
            .freeze(&mut flash.acr);

        // Acquire the GPIOA peripheral
        let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

        let pins = (
            gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh),
            gpiob.pb14.into_floating_input(&mut gpiob.crh),
            gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh),
        );
        let mut delay = Delay::new(cp.SYST, clocks);

        let spi = Spi::spi2(dp.SPI2, pins, ws2812::MODE, 3.mhz(), clocks, &mut rcc.apb1);


        let mut data: [RGB8; _THE_LEN] = [RGB8::default(); _THE_LEN];
        let mut ws = Ws2812::new(spi);
        loop {
            for i in 0..3 {
                rolling(&mut data, i, false);
                ws.write(data.iter().cloned()).unwrap();
                delay.delay_ms(DELAY as u16);
            }
        }
    }
    loop {
        continue;
    }
}

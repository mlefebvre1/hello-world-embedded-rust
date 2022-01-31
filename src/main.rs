#![no_main]
#![no_std]

use crate::board::{
    hal::delay::Delay, hal::gpio::Edge, hal::pac, hal::prelude::*, hal::timer::Timer, led::Leds,
};
use panic_halt as _;
use stm32f407g_disc as board;

use cortex_m::interrupt::free;
use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

mod led_pattern;
use led_pattern::{
    circle_pattern, crisscross_pattern, cross_pattern, hammer_pattern, zigzag_pattern, LedPattern,
    G_TIMER_TIM2, G_USER_BUTTON, PATTERN,
};

#[entry]
fn main() -> ! {
    if let (Some(mut p), Some(c)) = (pac::Peripherals::take(), Peripherals::take()) {
        let gpiod = p.GPIOD.split();
        let gpioa = p.GPIOA.split();
        let mut syscfg = p.SYSCFG.constrain();

        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiod);

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();
        let mut user_button = gpioa.pa0.into_pull_down_input();
        user_button.make_interrupt_source(&mut syscfg);

        user_button.make_interrupt_source(&mut syscfg);
        user_button.enable_interrupt(&mut p.EXTI);
        user_button.trigger_on_edge(&mut p.EXTI, Edge::RISING);

        // // Create a 1s periodic interrupt from TIM2 to debounce the USER BUTTON
        let timer = Timer::tim2(p.TIM2, 1.hz(), clocks);

        pac::NVIC::unpend(pac::Interrupt::TIM2);
        pac::NVIC::unpend(pac::Interrupt::EXTI0);
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::TIM2);
            pac::NVIC::unmask(pac::Interrupt::EXTI0);
        }
        free(|cs| {
            G_USER_BUTTON.borrow(cs).replace(Some(user_button));
            G_TIMER_TIM2.borrow(cs).replace(Some(timer));
        });

        // Get delay provider
        let mut delay = Delay::new(c.SYST, clocks);
        loop {
            unsafe {
                match PATTERN {
                    LedPattern::Hammer => hammer_pattern(&mut leds, &mut delay),
                    LedPattern::Circle => circle_pattern(&mut leds, &mut delay),
                    LedPattern::Zigzag => zigzag_pattern(&mut leds, &mut delay),
                    LedPattern::CrissCross => crisscross_pattern(&mut leds, &mut delay),
                    LedPattern::Cross => cross_pattern(&mut leds, &mut delay),
                }
            }
        }
    }

    loop {
        continue;
    }
}

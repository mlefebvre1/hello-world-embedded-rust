use core::cell::RefCell;
use core::ops::DerefMut;

use crate::board::{
    hal::delay::Delay,
    hal::gpio::gpioa,
    hal::gpio::{Input, PullDown},
    hal::pac,
    hal::prelude::*,
    hal::stm32::interrupt,
    hal::timer::{Event, Timer},
    led::{LedColor, Leds},
};
use cortex_m::interrupt::{free, Mutex};

#[derive(Debug)]
pub enum LedPattern {
    Hammer,
    Circle,
    Zigzag,
    CrissCross,
    Cross,
}

pub fn hammer_pattern(leds: &mut Leds, delay: &mut Delay) {
    const PERIOD: u16 = 200;
    leds[LedColor::Orange].off();
    leds[LedColor::Green].off();
    leds[LedColor::Red].off();
    leds[LedColor::Blue].off();
    delay.delay_ms(PERIOD);
    leds[LedColor::Orange].on();
    leds[LedColor::Green].on();
    leds[LedColor::Red].on();
    leds[LedColor::Blue].on();
    delay.delay_ms(PERIOD);
}

pub fn circle_pattern(leds: &mut Leds, delay: &mut Delay) {
    const PERIOD: u16 = 100;
    leds[LedColor::Orange].off();
    leds[LedColor::Green].off();
    leds[LedColor::Red].off();
    leds[LedColor::Blue].on();
    delay.delay_ms(PERIOD);
    leds[LedColor::Blue].off();
    leds[LedColor::Green].on();
    delay.delay_ms(PERIOD);
    leds[LedColor::Green].off();
    leds[LedColor::Orange].on();
    delay.delay_ms(PERIOD);
    leds[LedColor::Orange].off();
    leds[LedColor::Red].on();
    delay.delay_ms(PERIOD);
    leds[LedColor::Red].off();
}
pub fn zigzag_pattern(leds: &mut Leds, delay: &mut Delay) {
    const PERIOD: u16 = 100;
    leds[LedColor::Orange].off();
    leds[LedColor::Green].off();
    leds[LedColor::Red].off();
    leds[LedColor::Blue].on();
    delay.delay_ms(PERIOD);
    leds[LedColor::Blue].off();
    leds[LedColor::Red].on();
    delay.delay_ms(PERIOD);
    leds[LedColor::Red].off();
    leds[LedColor::Green].on();
    delay.delay_ms(PERIOD);
    leds[LedColor::Green].off();
    leds[LedColor::Orange].on();
    delay.delay_ms(PERIOD);
    leds[LedColor::Orange].off();
}

pub fn crisscross_pattern(leds: &mut Leds, delay: &mut Delay) {
    const PERIOD: u16 = 200;
    leds[LedColor::Orange].off();
    leds[LedColor::Green].off();
    leds[LedColor::Red].off();
    leds[LedColor::Blue].on();
    delay.delay_ms(PERIOD);
    leds[LedColor::Blue].off();
    leds[LedColor::Orange].on();
    delay.delay_ms(PERIOD);
    leds[LedColor::Orange].off();
    leds[LedColor::Red].on();
    delay.delay_ms(PERIOD);
    leds[LedColor::Red].off();
    leds[LedColor::Green].on();
    delay.delay_ms(PERIOD);
    leds[LedColor::Green].off();
}

pub fn cross_pattern(leds: &mut Leds, delay: &mut Delay) {
    const PERIOD: u16 = 200;
    leds[LedColor::Red].off();
    leds[LedColor::Green].off();
    leds[LedColor::Orange].on();
    leds[LedColor::Blue].on();
    delay.delay_ms(PERIOD);
    leds[LedColor::Blue].off();
    leds[LedColor::Orange].off();
    leds[LedColor::Red].on();
    leds[LedColor::Green].on();
    delay.delay_ms(PERIOD);
    leds[LedColor::Red].off();
    leds[LedColor::Green].off();
}

type UserButton = gpioa::PA0<Input<PullDown>>;
pub static G_USER_BUTTON: Mutex<RefCell<Option<UserButton>>> = Mutex::new(RefCell::new(None));
pub static G_TIMER_TIM2: Mutex<RefCell<Option<Timer<pac::TIM2>>>> = Mutex::new(RefCell::new(None));
pub static mut G_TIM2_EXPIRED: bool = true;
pub static mut PATTERN: LedPattern = LedPattern::Hammer;

#[interrupt]
fn EXTI0() {
    free(|cs| unsafe {
        if let (Some(ref mut user_button), Some(timer)) = (
            G_USER_BUTTON.borrow(cs).borrow_mut().deref_mut(),
            G_TIMER_TIM2.borrow(cs).borrow_mut().deref_mut(),
        ) {
            user_button.clear_interrupt_pending_bit();
            if G_TIM2_EXPIRED {
                G_TIM2_EXPIRED = false;
                PATTERN = match PATTERN {
                    LedPattern::Hammer => LedPattern::Circle,
                    LedPattern::Circle => LedPattern::Zigzag,
                    LedPattern::Zigzag => LedPattern::CrissCross,
                    LedPattern::CrissCross => LedPattern::Cross,
                    LedPattern::Cross => LedPattern::Hammer,
                };
                // hprintln!("Switching pattern to {:?}", PATTERN);
                timer.listen(Event::TimeOut);
            }
        }
    });
}
#[interrupt]
fn TIM2() {
    free(|cs| unsafe {
        if let Some(ref mut timer) = G_TIMER_TIM2.borrow(cs).borrow_mut().deref_mut() {
            // hprintln!("Entering TIM2 Interrupt!");
            timer.clear_interrupt(Event::TimeOut);
            timer.unlisten(Event::TimeOut);
            G_TIM2_EXPIRED = true;
        }
    });
}

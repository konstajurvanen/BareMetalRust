#![no_std]
#![no_main]

use embedded_graphics::mono_font::iso_8859_1::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;
use gd32vf103xx_hal::eclic::{EclicExt, Level, Priority, TriggerType};
use gd32vf103xx_hal::pac::{Interrupt, ECLIC, TIMER0};
use gd32vf103xx_hal::timer::Timer;
use longan_nano::hal::{pac, prelude::*, rcu::RcuExt};
use longan_nano::{lcd, lcd_pins};
use panic_halt as _;
use riscv_rt::entry;
static mut TIMER: Option<Timer<TIMER0>> = None;

static mut HOW_MANY_Z: u32 = 1;
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    // Configure clocks
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    // Configure lcd to display the changes caused by interrupts
    let mut afio = dp.AFIO.constrain(&mut rcu);
    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);
    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = lcd::configure(dp.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (width, height) = (lcd.size().width as i32, lcd.size().height as i32);

    // Clear screen
    Rectangle::new(Point::new(0, 0), Size::new(width as u32, height as u32))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(&mut lcd)
        .unwrap();

    // Setup timer to send TIMER0_UP interrupt twice a second
    unsafe {
        TIMER = Some(Timer::timer0(dp.TIMER0, 2.hz(), &mut rcu));
        TIMER
            .as_mut()
            .unwrap()
            .listen(gd32vf103xx_hal::timer::Event::Update);
    }

    // Reset and configure the interrupt controller
    ECLIC::reset();
    ECLIC::set_threshold_level(Level::L4);
    ECLIC::setup(
        Interrupt::TIMER0_UP,
        TriggerType::Level,
        Level::L5,
        Priority::P5,
    );
    unsafe {
        riscv::interrupt::enable();
        ECLIC::unmask(Interrupt::TIMER0_UP);
    }

    let style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(Rgb565::MAGENTA)
        .background_color(Rgb565::BLACK)
        .build();

    let style2 = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(Rgb565::RED)
        .background_color(Rgb565::BLACK)
        .build();

    let point = Point::new(20, 30);
    let point2 = Point::new(40, 60);

    Text::new(" Yo yo yo! ", point, style).draw(&mut lcd).unwrap();
    loop {
        // Display different things on the lcd according to what value has 
        // been set for HOW_MANY_Z in interrupt handler of TIMER0_UP
        if unsafe { HOW_MANY_Z == 1 } {
            Text::new("    z     ", point, style).draw(&mut lcd).unwrap();
            Text::new("         ", point2, style).draw(&mut lcd).unwrap();
        } else if unsafe { HOW_MANY_Z == 2 } {
            Text::new("    zZ ", point, style).draw(&mut lcd).unwrap();
        } else if unsafe { HOW_MANY_Z == 3 } {
            Text::new("   ZzZ ", point, style).draw(&mut lcd).unwrap();
        } else if unsafe { HOW_MANY_Z == 4 } {
            Text::new("   ZzZz", point, style).draw(&mut lcd).unwrap();
        } else if unsafe { HOW_MANY_Z == 5 } {
            Text::new("  zZzZz", point, style).draw(&mut lcd).unwrap();
        } else if unsafe { HOW_MANY_Z == 6 } {
            Text::new("         ", point, style).draw(&mut lcd).unwrap();
            Text::new("  o . o  ", point2, style2).draw(&mut lcd).unwrap();
        } else if unsafe { HOW_MANY_Z == 7 } {
            Text::new("         ", point, style).draw(&mut lcd).unwrap();
            Text::new("  - . -  ", point2, style2).draw(&mut lcd).unwrap();
        } else {
            Text::new("         ", point2, style2).draw(&mut lcd).unwrap();
        }
        unsafe {
            // Sleep until next interrupt
            riscv::asm::wfi();
        }
    }
}
// Interrupt handler for the TIMER0_UP -interrupt
#[no_mangle]
fn TIMER0_UP() {
    unsafe {
        riscv::interrupt::disable();
        if HOW_MANY_Z == 7 {
            HOW_MANY_Z = 0;
        } else {
            HOW_MANY_Z += 1;
        }
        TIMER.as_mut().unwrap().clear_update_interrupt_flag();
        riscv::interrupt::enable();
    }
}

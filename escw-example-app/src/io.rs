use escw_mcu::peripheral::io::{Io, IoState};
use escw_mcu_stm32::peripheral::io::{Io as Stm32Io, IoPin, IoPort};

static mut LED: Option<Stm32Io> = None;
static mut KEY1: Option<Stm32Io> = None;
static mut KEY2: Option<Stm32Io> = None;

pub(crate) fn io_example() {
    unsafe {
        LED = Some(Stm32Io::new(IoPort::IOH, IoPin::P10));
        KEY1 = Some(Stm32Io::new(IoPort::IOA, IoPin::P00));
        KEY2 = Some(Stm32Io::new(IoPort::IOC, IoPin::P13));

        KEY2.as_ref().unwrap().with_event(io_event_handle);
    }
}

static mut STATE: IoState = IoState::Set;

pub(crate) fn io_example_tick() {
    unsafe {
        let now = KEY1.as_ref().unwrap().state();
        if now == IoState::Set && STATE == IoState::Reset {
            LED.as_ref().unwrap().toggle();
        }
        STATE = now;
    }
}

fn io_event_handle() {
    unsafe {
        if let Some(led) = LED.as_mut() {
            led.toggle();
        }
    }
}

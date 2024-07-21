use core::ptr::addr_of_mut;

use escw_mcu::{
    common::{AsyncKind, TransmitDirection, TransmitState},
    peripheral::uart::Uart,
};
use escw_mcu_stm32::peripheral::uart::{Stm32Usart, UartNum};

static mut UART: Option<Stm32Usart> = None;
static mut DATA: [u8; 32] = [0; 32];

pub fn uart_example() {
    unsafe {
        UART = Some(Stm32Usart::new(UartNum::U1));

        UART.as_mut().unwrap().with_event(event);

        UART.as_mut()
            .unwrap()
            .async_receive(AsyncKind::Dma, &mut *addr_of_mut!(DATA))
            .unwrap();
    }
}

pub fn uart_example_tick() {
    // if let Some(uart) = UART.as_mut() {
    //     data.fill(0);
    //     let ret = uart.receive(&mut data, 600000);
    //     if ret.is_ok() {
    //         uart.send(&mut data, 1000).unwrap();
    //     }
    // }
}

fn event(direction: TransmitDirection, state: TransmitState, size: u16) {
    unsafe {
        match direction {
            TransmitDirection::Receive => {
                if state == TransmitState::Completed {
                    UART.as_mut()
                        .unwrap()
                        .send(&DATA[..size as usize], 1000)
                        .unwrap();
                    DATA.fill(0);
                    UART.as_mut()
                        .unwrap()
                        .async_receive(AsyncKind::Dma, &mut *addr_of_mut!(DATA))
                        .unwrap();
                }
            }
            TransmitDirection::Send => (),
            TransmitDirection::Any => (),
        }
    }
}

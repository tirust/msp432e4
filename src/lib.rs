#![doc = "Peripheral access API for MSP432E401Y microcontrollers (generated using svd2rust v0.15.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.15.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn GPIOA();
    fn GPIOB();
    fn GPIOC();
    fn GPIOD();
    fn GPIOE();
    fn UART0();
    fn UART1();
    fn SSI0();
    fn I2C0();
    fn PWM0_FAULT();
    fn PWM0_0();
    fn PWM0_1();
    fn PWM0_2();
    fn QEI0();
    fn ADC0SS0();
    fn ADC0SS1();
    fn ADC0SS2();
    fn ADC0SS3();
    fn WATCHDOG0();
    fn TIMER0A();
    fn TIMER0B();
    fn TIMER1A();
    fn TIMER1B();
    fn TIMER2A();
    fn TIMER2B();
    fn COMP0();
    fn COMP1();
    fn COMP2();
    fn SYSCTL();
    fn FLASH_CTRL();
    fn GPIOF();
    fn GPIOG();
    fn GPIOH();
    fn UART2();
    fn SSI1();
    fn TIMER3A();
    fn TIMER3B();
    fn I2C1();
    fn CAN0();
    fn CAN1();
    fn EMAC0();
    fn HIB();
    fn USB0();
    fn PWM0_3();
    fn UDMA();
    fn UDMAERR();
    fn ADC1SS0();
    fn ADC1SS1();
    fn ADC1SS2();
    fn ADC1SS3();
    fn EPI0();
    fn GPIOJ();
    fn GPIOK();
    fn GPIOL();
    fn SSI2();
    fn SSI3();
    fn UART3();
    fn UART4();
    fn UART5();
    fn UART6();
    fn UART7();
    fn I2C2();
    fn I2C3();
    fn TIMER4A();
    fn TIMER4B();
    fn TIMER5A();
    fn TIMER5B();
    fn SYSEXC();
    fn I2C4();
    fn I2C5();
    fn GPIOM();
    fn GPION();
    fn GPIOP0();
    fn GPIOP1();
    fn GPIOP2();
    fn GPIOP3();
    fn GPIOP4();
    fn GPIOP5();
    fn GPIOP6();
    fn GPIOP7();
    fn GPIOQ0();
    fn GPIOQ1();
    fn GPIOQ2();
    fn GPIOQ3();
    fn GPIOQ4();
    fn GPIOQ5();
    fn GPIOQ6();
    fn GPIOQ7();
    fn TIMER6A();
    fn TIMER6B();
    fn TIMER7A();
    fn TIMER7B();
    fn I2C6();
    fn I2C7();
    fn I2C8();
    fn I2C9();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 111] = [
    Vector { _handler: GPIOA },
    Vector { _handler: GPIOB },
    Vector { _handler: GPIOC },
    Vector { _handler: GPIOD },
    Vector { _handler: GPIOE },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: SSI0 },
    Vector { _handler: I2C0 },
    Vector {
        _handler: PWM0_FAULT,
    },
    Vector { _handler: PWM0_0 },
    Vector { _handler: PWM0_1 },
    Vector { _handler: PWM0_2 },
    Vector { _handler: QEI0 },
    Vector { _handler: ADC0SS0 },
    Vector { _handler: ADC0SS1 },
    Vector { _handler: ADC0SS2 },
    Vector { _handler: ADC0SS3 },
    Vector {
        _handler: WATCHDOG0,
    },
    Vector { _handler: TIMER0A },
    Vector { _handler: TIMER0B },
    Vector { _handler: TIMER1A },
    Vector { _handler: TIMER1B },
    Vector { _handler: TIMER2A },
    Vector { _handler: TIMER2B },
    Vector { _handler: COMP0 },
    Vector { _handler: COMP1 },
    Vector { _handler: COMP2 },
    Vector { _handler: SYSCTL },
    Vector {
        _handler: FLASH_CTRL,
    },
    Vector { _handler: GPIOF },
    Vector { _handler: GPIOG },
    Vector { _handler: GPIOH },
    Vector { _handler: UART2 },
    Vector { _handler: SSI1 },
    Vector { _handler: TIMER3A },
    Vector { _handler: TIMER3B },
    Vector { _handler: I2C1 },
    Vector { _handler: CAN0 },
    Vector { _handler: CAN1 },
    Vector { _handler: EMAC0 },
    Vector { _handler: HIB },
    Vector { _handler: USB0 },
    Vector { _handler: PWM0_3 },
    Vector { _handler: UDMA },
    Vector { _handler: UDMAERR },
    Vector { _handler: ADC1SS0 },
    Vector { _handler: ADC1SS1 },
    Vector { _handler: ADC1SS2 },
    Vector { _handler: ADC1SS3 },
    Vector { _handler: EPI0 },
    Vector { _handler: GPIOJ },
    Vector { _handler: GPIOK },
    Vector { _handler: GPIOL },
    Vector { _handler: SSI2 },
    Vector { _handler: SSI3 },
    Vector { _handler: UART3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector { _handler: UART6 },
    Vector { _handler: UART7 },
    Vector { _handler: I2C2 },
    Vector { _handler: I2C3 },
    Vector { _handler: TIMER4A },
    Vector { _handler: TIMER4B },
    Vector { _handler: TIMER5A },
    Vector { _handler: TIMER5B },
    Vector { _handler: SYSEXC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C4 },
    Vector { _handler: I2C5 },
    Vector { _handler: GPIOM },
    Vector { _handler: GPION },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPIOP0 },
    Vector { _handler: GPIOP1 },
    Vector { _handler: GPIOP2 },
    Vector { _handler: GPIOP3 },
    Vector { _handler: GPIOP4 },
    Vector { _handler: GPIOP5 },
    Vector { _handler: GPIOP6 },
    Vector { _handler: GPIOP7 },
    Vector { _handler: GPIOQ0 },
    Vector { _handler: GPIOQ1 },
    Vector { _handler: GPIOQ2 },
    Vector { _handler: GPIOQ3 },
    Vector { _handler: GPIOQ4 },
    Vector { _handler: GPIOQ5 },
    Vector { _handler: GPIOQ6 },
    Vector { _handler: GPIOQ7 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TIMER6A },
    Vector { _handler: TIMER6B },
    Vector { _handler: TIMER7A },
    Vector { _handler: TIMER7B },
    Vector { _handler: I2C6 },
    Vector { _handler: I2C7 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C8 },
    Vector { _handler: I2C9 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - GPIOA"]
    GPIOA,
    #[doc = "1 - GPIOB"]
    GPIOB,
    #[doc = "2 - GPIOC"]
    GPIOC,
    #[doc = "3 - GPIOD"]
    GPIOD,
    #[doc = "4 - GPIOE"]
    GPIOE,
    #[doc = "5 - UART0"]
    UART0,
    #[doc = "6 - UART1"]
    UART1,
    #[doc = "7 - SSI0"]
    SSI0,
    #[doc = "8 - I2C0"]
    I2C0,
    #[doc = "9 - PWM0_FAULT"]
    PWM0_FAULT,
    #[doc = "10 - PWM0_0"]
    PWM0_0,
    #[doc = "11 - PWM0_1"]
    PWM0_1,
    #[doc = "12 - PWM0_2"]
    PWM0_2,
    #[doc = "13 - QEI0"]
    QEI0,
    #[doc = "14 - ADC0SS0"]
    ADC0SS0,
    #[doc = "15 - ADC0SS1"]
    ADC0SS1,
    #[doc = "16 - ADC0SS2"]
    ADC0SS2,
    #[doc = "17 - ADC0SS3"]
    ADC0SS3,
    #[doc = "18 - WATCHDOG0"]
    WATCHDOG0,
    #[doc = "19 - TIMER0A"]
    TIMER0A,
    #[doc = "20 - TIMER0B"]
    TIMER0B,
    #[doc = "21 - TIMER1A"]
    TIMER1A,
    #[doc = "22 - TIMER1B"]
    TIMER1B,
    #[doc = "23 - TIMER2A"]
    TIMER2A,
    #[doc = "24 - TIMER2B"]
    TIMER2B,
    #[doc = "25 - COMP0"]
    COMP0,
    #[doc = "26 - COMP1"]
    COMP1,
    #[doc = "27 - COMP2"]
    COMP2,
    #[doc = "28 - SYSCTL"]
    SYSCTL,
    #[doc = "29 - FLASH_CTRL"]
    FLASH_CTRL,
    #[doc = "30 - GPIOF"]
    GPIOF,
    #[doc = "31 - GPIOG"]
    GPIOG,
    #[doc = "32 - GPIOH"]
    GPIOH,
    #[doc = "33 - UART2"]
    UART2,
    #[doc = "34 - SSI1"]
    SSI1,
    #[doc = "35 - TIMER3A"]
    TIMER3A,
    #[doc = "36 - TIMER3B"]
    TIMER3B,
    #[doc = "37 - I2C1"]
    I2C1,
    #[doc = "38 - CAN0"]
    CAN0,
    #[doc = "39 - CAN1"]
    CAN1,
    #[doc = "40 - EMAC0"]
    EMAC0,
    #[doc = "41 - HIB"]
    HIB,
    #[doc = "42 - USB0"]
    USB0,
    #[doc = "43 - PWM0_3"]
    PWM0_3,
    #[doc = "44 - UDMA"]
    UDMA,
    #[doc = "45 - UDMAERR"]
    UDMAERR,
    #[doc = "46 - ADC1SS0"]
    ADC1SS0,
    #[doc = "47 - ADC1SS1"]
    ADC1SS1,
    #[doc = "48 - ADC1SS2"]
    ADC1SS2,
    #[doc = "49 - ADC1SS3"]
    ADC1SS3,
    #[doc = "50 - EPI0"]
    EPI0,
    #[doc = "51 - GPIOJ"]
    GPIOJ,
    #[doc = "52 - GPIOK"]
    GPIOK,
    #[doc = "53 - GPIOL"]
    GPIOL,
    #[doc = "54 - SSI2"]
    SSI2,
    #[doc = "55 - SSI3"]
    SSI3,
    #[doc = "56 - UART3"]
    UART3,
    #[doc = "57 - UART4"]
    UART4,
    #[doc = "58 - UART5"]
    UART5,
    #[doc = "59 - UART6"]
    UART6,
    #[doc = "60 - UART7"]
    UART7,
    #[doc = "61 - I2C2"]
    I2C2,
    #[doc = "62 - I2C3"]
    I2C3,
    #[doc = "63 - TIMER4A"]
    TIMER4A,
    #[doc = "64 - TIMER4B"]
    TIMER4B,
    #[doc = "65 - TIMER5A"]
    TIMER5A,
    #[doc = "66 - TIMER5B"]
    TIMER5B,
    #[doc = "67 - SYSEXC"]
    SYSEXC,
    #[doc = "70 - I2C4"]
    I2C4,
    #[doc = "71 - I2C5"]
    I2C5,
    #[doc = "72 - GPIOM"]
    GPIOM,
    #[doc = "73 - GPION"]
    GPION,
    #[doc = "76 - GPIOP0"]
    GPIOP0,
    #[doc = "77 - GPIOP1"]
    GPIOP1,
    #[doc = "78 - GPIOP2"]
    GPIOP2,
    #[doc = "79 - GPIOP3"]
    GPIOP3,
    #[doc = "80 - GPIOP4"]
    GPIOP4,
    #[doc = "81 - GPIOP5"]
    GPIOP5,
    #[doc = "82 - GPIOP6"]
    GPIOP6,
    #[doc = "83 - GPIOP7"]
    GPIOP7,
    #[doc = "84 - GPIOQ0"]
    GPIOQ0,
    #[doc = "85 - GPIOQ1"]
    GPIOQ1,
    #[doc = "86 - GPIOQ2"]
    GPIOQ2,
    #[doc = "87 - GPIOQ3"]
    GPIOQ3,
    #[doc = "88 - GPIOQ4"]
    GPIOQ4,
    #[doc = "89 - GPIOQ5"]
    GPIOQ5,
    #[doc = "90 - GPIOQ6"]
    GPIOQ6,
    #[doc = "91 - GPIOQ7"]
    GPIOQ7,
    #[doc = "98 - TIMER6A"]
    TIMER6A,
    #[doc = "99 - TIMER6B"]
    TIMER6B,
    #[doc = "100 - TIMER7A"]
    TIMER7A,
    #[doc = "101 - TIMER7B"]
    TIMER7B,
    #[doc = "102 - I2C6"]
    I2C6,
    #[doc = "103 - I2C7"]
    I2C7,
    #[doc = "109 - I2C8"]
    I2C8,
    #[doc = "110 - I2C9"]
    I2C9,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::GPIOA => 0,
            Interrupt::GPIOB => 1,
            Interrupt::GPIOC => 2,
            Interrupt::GPIOD => 3,
            Interrupt::GPIOE => 4,
            Interrupt::UART0 => 5,
            Interrupt::UART1 => 6,
            Interrupt::SSI0 => 7,
            Interrupt::I2C0 => 8,
            Interrupt::PWM0_FAULT => 9,
            Interrupt::PWM0_0 => 10,
            Interrupt::PWM0_1 => 11,
            Interrupt::PWM0_2 => 12,
            Interrupt::QEI0 => 13,
            Interrupt::ADC0SS0 => 14,
            Interrupt::ADC0SS1 => 15,
            Interrupt::ADC0SS2 => 16,
            Interrupt::ADC0SS3 => 17,
            Interrupt::WATCHDOG0 => 18,
            Interrupt::TIMER0A => 19,
            Interrupt::TIMER0B => 20,
            Interrupt::TIMER1A => 21,
            Interrupt::TIMER1B => 22,
            Interrupt::TIMER2A => 23,
            Interrupt::TIMER2B => 24,
            Interrupt::COMP0 => 25,
            Interrupt::COMP1 => 26,
            Interrupt::COMP2 => 27,
            Interrupt::SYSCTL => 28,
            Interrupt::FLASH_CTRL => 29,
            Interrupt::GPIOF => 30,
            Interrupt::GPIOG => 31,
            Interrupt::GPIOH => 32,
            Interrupt::UART2 => 33,
            Interrupt::SSI1 => 34,
            Interrupt::TIMER3A => 35,
            Interrupt::TIMER3B => 36,
            Interrupt::I2C1 => 37,
            Interrupt::CAN0 => 38,
            Interrupt::CAN1 => 39,
            Interrupt::EMAC0 => 40,
            Interrupt::HIB => 41,
            Interrupt::USB0 => 42,
            Interrupt::PWM0_3 => 43,
            Interrupt::UDMA => 44,
            Interrupt::UDMAERR => 45,
            Interrupt::ADC1SS0 => 46,
            Interrupt::ADC1SS1 => 47,
            Interrupt::ADC1SS2 => 48,
            Interrupt::ADC1SS3 => 49,
            Interrupt::EPI0 => 50,
            Interrupt::GPIOJ => 51,
            Interrupt::GPIOK => 52,
            Interrupt::GPIOL => 53,
            Interrupt::SSI2 => 54,
            Interrupt::SSI3 => 55,
            Interrupt::UART3 => 56,
            Interrupt::UART4 => 57,
            Interrupt::UART5 => 58,
            Interrupt::UART6 => 59,
            Interrupt::UART7 => 60,
            Interrupt::I2C2 => 61,
            Interrupt::I2C3 => 62,
            Interrupt::TIMER4A => 63,
            Interrupt::TIMER4B => 64,
            Interrupt::TIMER5A => 65,
            Interrupt::TIMER5B => 66,
            Interrupt::SYSEXC => 67,
            Interrupt::I2C4 => 70,
            Interrupt::I2C5 => 71,
            Interrupt::GPIOM => 72,
            Interrupt::GPION => 73,
            Interrupt::GPIOP0 => 76,
            Interrupt::GPIOP1 => 77,
            Interrupt::GPIOP2 => 78,
            Interrupt::GPIOP3 => 79,
            Interrupt::GPIOP4 => 80,
            Interrupt::GPIOP5 => 81,
            Interrupt::GPIOP6 => 82,
            Interrupt::GPIOP7 => 83,
            Interrupt::GPIOQ0 => 84,
            Interrupt::GPIOQ1 => 85,
            Interrupt::GPIOQ2 => 86,
            Interrupt::GPIOQ3 => 87,
            Interrupt::GPIOQ4 => 88,
            Interrupt::GPIOQ5 => 89,
            Interrupt::GPIOQ6 => 90,
            Interrupt::GPIOQ7 => 91,
            Interrupt::TIMER6A => 98,
            Interrupt::TIMER6B => 99,
            Interrupt::TIMER7A => 100,
            Interrupt::TIMER7B => 101,
            Interrupt::I2C6 => 102,
            Interrupt::I2C7 => 103,
            Interrupt::I2C8 => 109,
            Interrupt::I2C9 => 110,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Register map for WATCHDOG0 peripheral"]
pub struct WATCHDOG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG0 {}
impl WATCHDOG0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog0::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for WATCHDOG0 {
    type Target = watchdog0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WATCHDOG0::ptr() }
    }
}
#[doc = "Register map for WATCHDOG0 peripheral"]
pub mod watchdog0;
#[doc = "Register map for WATCHDOG0 peripheral"]
pub struct WATCHDOG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG1 {}
impl WATCHDOG1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog0::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for WATCHDOG1 {
    type Target = watchdog0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WATCHDOG1::ptr() }
    }
}
#[doc = "Register map for SSI0 peripheral"]
pub struct SSI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI0 {}
impl SSI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi0::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for SSI0 {
    type Target = ssi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI0::ptr() }
    }
}
#[doc = "Register map for SSI0 peripheral"]
pub mod ssi0;
#[doc = "Register map for SSI0 peripheral"]
pub struct SSI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI1 {}
impl SSI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi0::RegisterBlock {
        1073778688 as *const _
    }
}
impl Deref for SSI1 {
    type Target = ssi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI1::ptr() }
    }
}
#[doc = "Register map for SSI0 peripheral"]
pub struct SSI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI2 {}
impl SSI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi0::RegisterBlock {
        1073782784 as *const _
    }
}
impl Deref for SSI2 {
    type Target = ssi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI2::ptr() }
    }
}
#[doc = "Register map for SSI0 peripheral"]
pub struct SSI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI3 {}
impl SSI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi0::RegisterBlock {
        1073786880 as *const _
    }
}
impl Deref for SSI3 {
    type Target = ssi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI3::ptr() }
    }
}
#[doc = "Register map for UART0 peripheral"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Register map for UART0 peripheral"]
pub mod uart0;
#[doc = "Register map for UART0 peripheral"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        1073795072 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Register map for UART0 peripheral"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        1073799168 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "Register map for UART0 peripheral"]
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART3 {}
impl UART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        1073803264 as *const _
    }
}
impl Deref for UART3 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART3::ptr() }
    }
}
#[doc = "Register map for UART0 peripheral"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for UART4 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "Register map for UART0 peripheral"]
pub struct UART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART5 {}
impl UART5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        1073811456 as *const _
    }
}
impl Deref for UART5 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART5::ptr() }
    }
}
#[doc = "Register map for UART0 peripheral"]
pub struct UART6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART6 {}
impl UART6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        1073815552 as *const _
    }
}
impl Deref for UART6 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART6::ptr() }
    }
}
#[doc = "Register map for UART0 peripheral"]
pub struct UART7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART7 {}
impl UART7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        1073819648 as *const _
    }
}
impl Deref for UART7 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART7::ptr() }
    }
}
#[doc = "Register map for I2C0 peripheral"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "Register map for I2C0 peripheral"]
pub mod i2c0;
#[doc = "Register map for I2C0 peripheral"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Register map for I2C0 peripheral"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        1073881088 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "Register map for I2C0 peripheral"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        1073885184 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "Register map for PWM0 peripheral"]
pub struct PWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0 {}
impl PWM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0::RegisterBlock {
        1073905664 as *const _
    }
}
impl Deref for PWM0 {
    type Target = pwm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM0::ptr() }
    }
}
#[doc = "Register map for PWM0 peripheral"]
pub mod pwm0;
#[doc = "Register map for QEI0 peripheral"]
pub struct QEI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QEI0 {}
impl QEI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qei0::RegisterBlock {
        1073922048 as *const _
    }
}
impl Deref for QEI0 {
    type Target = qei0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*QEI0::ptr() }
    }
}
#[doc = "Register map for QEI0 peripheral"]
pub mod qei0;
#[doc = "Register map for TIMER0 peripheral"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        1073938432 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "Register map for TIMER0 peripheral"]
pub mod timer0;
#[doc = "Register map for TIMER0 peripheral"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        1073942528 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "Register map for TIMER0 peripheral"]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        1073946624 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "Register map for TIMER0 peripheral"]
pub struct TIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER3 {}
impl TIMER3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        1073950720 as *const _
    }
}
impl Deref for TIMER3 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER3::ptr() }
    }
}
#[doc = "Register map for TIMER0 peripheral"]
pub struct TIMER4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER4 {}
impl TIMER4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        1073954816 as *const _
    }
}
impl Deref for TIMER4 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER4::ptr() }
    }
}
#[doc = "Register map for TIMER0 peripheral"]
pub struct TIMER5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER5 {}
impl TIMER5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        1073958912 as *const _
    }
}
impl Deref for TIMER5 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER5::ptr() }
    }
}
#[doc = "Register map for ADC0 peripheral"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        1073971200 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "Register map for ADC0 peripheral"]
pub mod adc0;
#[doc = "Register map for ADC0 peripheral"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        1073975296 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Register map for COMP peripheral"]
pub struct COMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP {}
impl COMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comp::RegisterBlock {
        1073987584 as *const _
    }
}
impl Deref for COMP {
    type Target = comp::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMP::ptr() }
    }
}
#[doc = "Register map for COMP peripheral"]
pub mod comp;
#[doc = "Register map for CAN0 peripheral"]
pub struct CAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN0 {}
impl CAN0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can0::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for CAN0 {
    type Target = can0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN0::ptr() }
    }
}
#[doc = "Register map for CAN0 peripheral"]
pub mod can0;
#[doc = "Register map for CAN0 peripheral"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can0::RegisterBlock {
        1074008064 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "Register map for USB0 peripheral"]
pub struct USB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0 {}
impl USB0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0::RegisterBlock {
        1074069504 as *const _
    }
}
impl Deref for USB0 {
    type Target = usb0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0::ptr() }
    }
}
#[doc = "Register map for USB0 peripheral"]
pub mod usb0;
#[doc = "Register map for GPIOA peripheral"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        1074102272 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "Register map for GPIOA peripheral"]
pub mod gpioa;
#[doc = "Register map for GPIOA peripheral"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        1074106368 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "Register map for GPIOA peripheral"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        1074110464 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "Register map for GPIOA peripheral"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        1074114560 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "Register map for GPIOA peripheral"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        1074118656 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "Register map for GPIOA peripheral"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        1074122752 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOF::ptr() }
    }
}
#[doc = "Register map for GPIOA peripheral"]
pub struct GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOG {}
impl GPIOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        1074126848 as *const _
    }
}
impl Deref for GPIOG {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOG::ptr() }
    }
}
#[doc = "Register map for GPIOA peripheral"]
pub struct GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOH {}
impl GPIOH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        1074130944 as *const _
    }
}
impl Deref for GPIOH {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOH::ptr() }
    }
}
#[doc = "Register map for GPIOA peripheral"]
pub struct GPIOJ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOJ {}
impl GPIOJ {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        1074135040 as *const _
    }
}
impl Deref for GPIOJ {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOJ::ptr() }
    }
}
#[doc = "Register map for GPIOA peripheral"]
pub struct GPIOK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOK {}
impl GPIOK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        1074139136 as *const _
    }
}
impl Deref for GPIOK {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOK::ptr() }
    }
}
#[doc = "Register map for GPIOA peripheral"]
pub struct GPIOL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOL {}
impl GPIOL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        1074143232 as *const _
    }
}
impl Deref for GPIOL {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOL::ptr() }
    }
}
#[doc = "Register map for GPIOA peripheral"]
pub struct GPIOM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOM {}
impl GPIOM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        1074147328 as *const _
    }
}
impl Deref for GPIOM {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOM::ptr() }
    }
}
#[doc = "Register map for GPIOA peripheral"]
pub struct GPION {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPION {}
impl GPION {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        1074151424 as *const _
    }
}
impl Deref for GPION {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPION::ptr() }
    }
}
#[doc = "Register map for GPIOA peripheral"]
pub struct GPIOP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOP {}
impl GPIOP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        1074155520 as *const _
    }
}
impl Deref for GPIOP {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOP::ptr() }
    }
}
#[doc = "Register map for GPIOA peripheral"]
pub struct GPIOQ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOQ {}
impl GPIOQ {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        1074159616 as *const _
    }
}
impl Deref for GPIOQ {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOQ::ptr() }
    }
}
#[doc = "Register map for EEPROM peripheral"]
pub struct EEPROM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EEPROM {}
impl EEPROM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eeprom::RegisterBlock {
        1074458624 as *const _
    }
}
impl Deref for EEPROM {
    type Target = eeprom::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EEPROM::ptr() }
    }
}
#[doc = "Register map for EEPROM peripheral"]
pub mod eeprom;
#[doc = "Register map for I2C0 peripheral"]
pub struct I2C8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C8 {}
impl I2C8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        1074495488 as *const _
    }
}
impl Deref for I2C8 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C8::ptr() }
    }
}
#[doc = "Register map for I2C0 peripheral"]
pub struct I2C9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C9 {}
impl I2C9 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        1074499584 as *const _
    }
}
impl Deref for I2C9 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C9::ptr() }
    }
}
#[doc = "Register map for I2C0 peripheral"]
pub struct I2C4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C4 {}
impl I2C4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        1074528256 as *const _
    }
}
impl Deref for I2C4 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C4::ptr() }
    }
}
#[doc = "Register map for I2C0 peripheral"]
pub struct I2C5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C5 {}
impl I2C5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        1074532352 as *const _
    }
}
impl Deref for I2C5 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C5::ptr() }
    }
}
#[doc = "Register map for I2C0 peripheral"]
pub struct I2C6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C6 {}
impl I2C6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        1074536448 as *const _
    }
}
impl Deref for I2C6 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C6::ptr() }
    }
}
#[doc = "Register map for I2C0 peripheral"]
pub struct I2C7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C7 {}
impl I2C7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        1074540544 as *const _
    }
}
impl Deref for I2C7 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C7::ptr() }
    }
}
#[doc = "Register map for EPI0 peripheral"]
pub struct EPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EPI0 {}
impl EPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const epi0::RegisterBlock {
        1074593792 as *const _
    }
}
impl Deref for EPI0 {
    type Target = epi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EPI0::ptr() }
    }
}
#[doc = "Register map for EPI0 peripheral"]
pub mod epi0;
#[doc = "Register map for TIMER0 peripheral"]
pub struct TIMER6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER6 {}
impl TIMER6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        1074659328 as *const _
    }
}
impl Deref for TIMER6 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER6::ptr() }
    }
}
#[doc = "Register map for TIMER0 peripheral"]
pub struct TIMER7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER7 {}
impl TIMER7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        1074663424 as *const _
    }
}
impl Deref for TIMER7 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER7::ptr() }
    }
}
#[doc = "Register map for EMAC0 peripheral"]
pub struct EMAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMAC0 {}
impl EMAC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const emac0::RegisterBlock {
        1074708480 as *const _
    }
}
impl Deref for EMAC0 {
    type Target = emac0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EMAC0::ptr() }
    }
}
#[doc = "Register map for EMAC0 peripheral"]
pub mod emac0;
#[doc = "Register map for SYSEXC peripheral"]
pub struct SYSEXC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSEXC {}
impl SYSEXC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sysexc::RegisterBlock {
        1074761728 as *const _
    }
}
impl Deref for SYSEXC {
    type Target = sysexc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSEXC::ptr() }
    }
}
#[doc = "Register map for SYSEXC peripheral"]
pub mod sysexc;
#[doc = "Register map for HIB peripheral"]
pub struct HIB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HIB {}
impl HIB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hib::RegisterBlock {
        1074774016 as *const _
    }
}
impl Deref for HIB {
    type Target = hib::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*HIB::ptr() }
    }
}
#[doc = "Register map for HIB peripheral"]
pub mod hib;
#[doc = "Register map for FLASH_CTRL peripheral"]
pub struct FLASH_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH_CTRL {}
impl FLASH_CTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash_ctrl::RegisterBlock {
        1074778112 as *const _
    }
}
impl Deref for FLASH_CTRL {
    type Target = flash_ctrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH_CTRL::ptr() }
    }
}
#[doc = "Register map for FLASH_CTRL peripheral"]
pub mod flash_ctrl;
#[doc = "Register map for SYSCTL peripheral"]
pub struct SYSCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCTL {}
impl SYSCTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sysctl::RegisterBlock {
        1074782208 as *const _
    }
}
impl Deref for SYSCTL {
    type Target = sysctl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCTL::ptr() }
    }
}
#[doc = "Register map for SYSCTL peripheral"]
pub mod sysctl;
#[doc = "Register map for UDMA peripheral"]
pub struct UDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UDMA {}
impl UDMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const udma::RegisterBlock {
        1074786304 as *const _
    }
}
impl Deref for UDMA {
    type Target = udma::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UDMA::ptr() }
    }
}
#[doc = "Register map for UDMA peripheral"]
pub mod udma;
#[doc = "Register map for CCM0 peripheral"]
pub struct CCM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCM0 {}
impl CCM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccm0::RegisterBlock {
        1141047812 as *const _
    }
}
impl Deref for CCM0 {
    type Target = ccm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCM0::ptr() }
    }
}
#[doc = "Register map for CCM0 peripheral"]
pub mod ccm0;
#[doc = "Register map for CRC peripheral"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        1141048320 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Register map for CRC peripheral"]
pub mod crc;
#[doc = "Register map for SHAMD5 peripheral"]
pub struct SHAMD5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SHAMD5 {}
impl SHAMD5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const shamd5::RegisterBlock {
        1141063680 as *const _
    }
}
impl Deref for SHAMD5 {
    type Target = shamd5::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SHAMD5::ptr() }
    }
}
#[doc = "Register map for SHAMD5 peripheral"]
pub mod shamd5;
#[doc = "Register map for SHAMD5 DMA peripheral"]
pub struct SHAMD5_DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SHAMD5_DMA {}
impl SHAMD5_DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const shamd5_dma::RegisterBlock {
        1141047312 as *const _
    }
}
impl Deref for SHAMD5_DMA {
    type Target = shamd5_dma::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SHAMD5_DMA::ptr() }
    }
}
#[doc = "Register map for SHAMD5 DMA peripheral"]
pub mod shamd5_dma;
#[doc = "Register map for AES peripheral"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes::RegisterBlock {
        1141071872 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES::ptr() }
    }
}
#[doc = "Register map for AES peripheral"]
pub mod aes;
#[doc = "Register map for AES DMA peripheral"]
pub struct AES_DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES_DMA {}
impl AES_DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes_dma::RegisterBlock {
        1141047328 as *const _
    }
}
impl Deref for AES_DMA {
    type Target = aes_dma::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES_DMA::ptr() }
    }
}
#[doc = "Register map for AES DMA peripheral"]
pub mod aes_dma;
#[doc = "Register map for DES peripheral"]
pub struct DES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DES {}
impl DES {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const des::RegisterBlock {
        1141080064 as *const _
    }
}
impl Deref for DES {
    type Target = des::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DES::ptr() }
    }
}
#[doc = "Register map for DES peripheral"]
pub mod des;
#[doc = "Register map for DES DMA peripheral"]
pub struct DES_DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DES_DMA {}
impl DES_DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const des_dma::RegisterBlock {
        1141047344 as *const _
    }
}
impl Deref for DES_DMA {
    type Target = des_dma::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DES_DMA::ptr() }
    }
}
#[doc = "Register map for DES DMA peripheral"]
pub mod des_dma;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "WATCHDOG0"]
    pub WATCHDOG0: WATCHDOG0,
    #[doc = "WATCHDOG1"]
    pub WATCHDOG1: WATCHDOG1,
    #[doc = "SSI0"]
    pub SSI0: SSI0,
    #[doc = "SSI1"]
    pub SSI1: SSI1,
    #[doc = "SSI2"]
    pub SSI2: SSI2,
    #[doc = "SSI3"]
    pub SSI3: SSI3,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "UART5"]
    pub UART5: UART5,
    #[doc = "UART6"]
    pub UART6: UART6,
    #[doc = "UART7"]
    pub UART7: UART7,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "PWM0"]
    pub PWM0: PWM0,
    #[doc = "QEI0"]
    pub QEI0: QEI0,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "TIMER3"]
    pub TIMER3: TIMER3,
    #[doc = "TIMER4"]
    pub TIMER4: TIMER4,
    #[doc = "TIMER5"]
    pub TIMER5: TIMER5,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "COMP"]
    pub COMP: COMP,
    #[doc = "CAN0"]
    pub CAN0: CAN0,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "USB0"]
    pub USB0: USB0,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "GPIOF"]
    pub GPIOF: GPIOF,
    #[doc = "GPIOG"]
    pub GPIOG: GPIOG,
    #[doc = "GPIOH"]
    pub GPIOH: GPIOH,
    #[doc = "GPIOJ"]
    pub GPIOJ: GPIOJ,
    #[doc = "GPIOK"]
    pub GPIOK: GPIOK,
    #[doc = "GPIOL"]
    pub GPIOL: GPIOL,
    #[doc = "GPIOM"]
    pub GPIOM: GPIOM,
    #[doc = "GPION"]
    pub GPION: GPION,
    #[doc = "GPIOP"]
    pub GPIOP: GPIOP,
    #[doc = "GPIOQ"]
    pub GPIOQ: GPIOQ,
    #[doc = "EEPROM"]
    pub EEPROM: EEPROM,
    #[doc = "I2C8"]
    pub I2C8: I2C8,
    #[doc = "I2C9"]
    pub I2C9: I2C9,
    #[doc = "I2C4"]
    pub I2C4: I2C4,
    #[doc = "I2C5"]
    pub I2C5: I2C5,
    #[doc = "I2C6"]
    pub I2C6: I2C6,
    #[doc = "I2C7"]
    pub I2C7: I2C7,
    #[doc = "EPI0"]
    pub EPI0: EPI0,
    #[doc = "TIMER6"]
    pub TIMER6: TIMER6,
    #[doc = "TIMER7"]
    pub TIMER7: TIMER7,
    #[doc = "EMAC0"]
    pub EMAC0: EMAC0,
    #[doc = "SYSEXC"]
    pub SYSEXC: SYSEXC,
    #[doc = "HIB"]
    pub HIB: HIB,
    #[doc = "FLASH_CTRL"]
    pub FLASH_CTRL: FLASH_CTRL,
    #[doc = "SYSCTL"]
    pub SYSCTL: SYSCTL,
    #[doc = "UDMA"]
    pub UDMA: UDMA,
    #[doc = "CCM0"]
    pub CCM0: CCM0,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "SHAMD5"]
    pub SHAMD5: SHAMD5,
    #[doc = "SHAMD5_DMA"]
    pub SHAMD5_DMA: SHAMD5_DMA,
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "AES_DMA"]
    pub AES_DMA: AES_DMA,
    #[doc = "DES"]
    pub DES: DES,
    #[doc = "DES_DMA"]
    pub DES_DMA: DES_DMA,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            WATCHDOG0: WATCHDOG0 {
                _marker: PhantomData,
            },
            WATCHDOG1: WATCHDOG1 {
                _marker: PhantomData,
            },
            SSI0: SSI0 {
                _marker: PhantomData,
            },
            SSI1: SSI1 {
                _marker: PhantomData,
            },
            SSI2: SSI2 {
                _marker: PhantomData,
            },
            SSI3: SSI3 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            UART3: UART3 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            UART5: UART5 {
                _marker: PhantomData,
            },
            UART6: UART6 {
                _marker: PhantomData,
            },
            UART7: UART7 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            PWM0: PWM0 {
                _marker: PhantomData,
            },
            QEI0: QEI0 {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            TIMER3: TIMER3 {
                _marker: PhantomData,
            },
            TIMER4: TIMER4 {
                _marker: PhantomData,
            },
            TIMER5: TIMER5 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            COMP: COMP {
                _marker: PhantomData,
            },
            CAN0: CAN0 {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            USB0: USB0 {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
            GPIOG: GPIOG {
                _marker: PhantomData,
            },
            GPIOH: GPIOH {
                _marker: PhantomData,
            },
            GPIOJ: GPIOJ {
                _marker: PhantomData,
            },
            GPIOK: GPIOK {
                _marker: PhantomData,
            },
            GPIOL: GPIOL {
                _marker: PhantomData,
            },
            GPIOM: GPIOM {
                _marker: PhantomData,
            },
            GPION: GPION {
                _marker: PhantomData,
            },
            GPIOP: GPIOP {
                _marker: PhantomData,
            },
            GPIOQ: GPIOQ {
                _marker: PhantomData,
            },
            EEPROM: EEPROM {
                _marker: PhantomData,
            },
            I2C8: I2C8 {
                _marker: PhantomData,
            },
            I2C9: I2C9 {
                _marker: PhantomData,
            },
            I2C4: I2C4 {
                _marker: PhantomData,
            },
            I2C5: I2C5 {
                _marker: PhantomData,
            },
            I2C6: I2C6 {
                _marker: PhantomData,
            },
            I2C7: I2C7 {
                _marker: PhantomData,
            },
            EPI0: EPI0 {
                _marker: PhantomData,
            },
            TIMER6: TIMER6 {
                _marker: PhantomData,
            },
            TIMER7: TIMER7 {
                _marker: PhantomData,
            },
            EMAC0: EMAC0 {
                _marker: PhantomData,
            },
            SYSEXC: SYSEXC {
                _marker: PhantomData,
            },
            HIB: HIB {
                _marker: PhantomData,
            },
            FLASH_CTRL: FLASH_CTRL {
                _marker: PhantomData,
            },
            SYSCTL: SYSCTL {
                _marker: PhantomData,
            },
            UDMA: UDMA {
                _marker: PhantomData,
            },
            CCM0: CCM0 {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            SHAMD5: SHAMD5 {
                _marker: PhantomData,
            },
            SHAMD5_DMA: SHAMD5_DMA {
                _marker: PhantomData,
            },
            AES: AES {
                _marker: PhantomData,
            },
            AES_DMA: AES_DMA {
                _marker: PhantomData,
            },
            DES: DES {
                _marker: PhantomData,
            },
            DES_DMA: DES_DMA {
                _marker: PhantomData,
            },
        }
    }
}

#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IFLS {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `UART_IFLS_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_IFLS_TXR {
    #[doc = "TX FIFO &lt;= 1/8 full"]
    UART_IFLS_TX1_8,
    #[doc = "TX FIFO &lt;= 1/4 full"]
    UART_IFLS_TX2_8,
    #[doc = "TX FIFO &lt;= 1/2 full (default)"]
    UART_IFLS_TX4_8,
    #[doc = "TX FIFO &lt;= 3/4 full"]
    UART_IFLS_TX6_8,
    #[doc = "TX FIFO &lt;= 7/8 full"]
    UART_IFLS_TX7_8,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl UART_IFLS_TXR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            UART_IFLS_TXR::UART_IFLS_TX1_8 => 0,
            UART_IFLS_TXR::UART_IFLS_TX2_8 => 1,
            UART_IFLS_TXR::UART_IFLS_TX4_8 => 2,
            UART_IFLS_TXR::UART_IFLS_TX6_8 => 3,
            UART_IFLS_TXR::UART_IFLS_TX7_8 => 4,
            UART_IFLS_TXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> UART_IFLS_TXR {
        match value {
            0 => UART_IFLS_TXR::UART_IFLS_TX1_8,
            1 => UART_IFLS_TXR::UART_IFLS_TX2_8,
            2 => UART_IFLS_TXR::UART_IFLS_TX4_8,
            3 => UART_IFLS_TXR::UART_IFLS_TX6_8,
            4 => UART_IFLS_TXR::UART_IFLS_TX7_8,
            i => UART_IFLS_TXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_TX1_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_tx1_8(&self) -> bool {
        *self == UART_IFLS_TXR::UART_IFLS_TX1_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_TX2_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_tx2_8(&self) -> bool {
        *self == UART_IFLS_TXR::UART_IFLS_TX2_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_TX4_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_tx4_8(&self) -> bool {
        *self == UART_IFLS_TXR::UART_IFLS_TX4_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_TX6_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_tx6_8(&self) -> bool {
        *self == UART_IFLS_TXR::UART_IFLS_TX6_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_TX7_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_tx7_8(&self) -> bool {
        *self == UART_IFLS_TXR::UART_IFLS_TX7_8
    }
}
#[doc = "Values that can be written to the field `UART_IFLS_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_IFLS_TXW {
    #[doc = "TX FIFO &lt;= 1/8 full"]
    UART_IFLS_TX1_8,
    #[doc = "TX FIFO &lt;= 1/4 full"]
    UART_IFLS_TX2_8,
    #[doc = "TX FIFO &lt;= 1/2 full (default)"]
    UART_IFLS_TX4_8,
    #[doc = "TX FIFO &lt;= 3/4 full"]
    UART_IFLS_TX6_8,
    #[doc = "TX FIFO &lt;= 7/8 full"]
    UART_IFLS_TX7_8,
}
impl UART_IFLS_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            UART_IFLS_TXW::UART_IFLS_TX1_8 => 0,
            UART_IFLS_TXW::UART_IFLS_TX2_8 => 1,
            UART_IFLS_TXW::UART_IFLS_TX4_8 => 2,
            UART_IFLS_TXW::UART_IFLS_TX6_8 => 3,
            UART_IFLS_TXW::UART_IFLS_TX7_8 => 4,
        }
    }
}
#[doc = r"Proxy"]
pub struct _UART_IFLS_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IFLS_TXW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_IFLS_TXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "TX FIFO &lt;= 1/8 full"]
    #[inline(always)]
    pub fn uart_ifls_tx1_8(self) -> &'a mut W {
        self.variant(UART_IFLS_TXW::UART_IFLS_TX1_8)
    }
    #[doc = "TX FIFO &lt;= 1/4 full"]
    #[inline(always)]
    pub fn uart_ifls_tx2_8(self) -> &'a mut W {
        self.variant(UART_IFLS_TXW::UART_IFLS_TX2_8)
    }
    #[doc = "TX FIFO &lt;= 1/2 full (default)"]
    #[inline(always)]
    pub fn uart_ifls_tx4_8(self) -> &'a mut W {
        self.variant(UART_IFLS_TXW::UART_IFLS_TX4_8)
    }
    #[doc = "TX FIFO &lt;= 3/4 full"]
    #[inline(always)]
    pub fn uart_ifls_tx6_8(self) -> &'a mut W {
        self.variant(UART_IFLS_TXW::UART_IFLS_TX6_8)
    }
    #[doc = "TX FIFO &lt;= 7/8 full"]
    #[inline(always)]
    pub fn uart_ifls_tx7_8(self) -> &'a mut W {
        self.variant(UART_IFLS_TXW::UART_IFLS_TX7_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 0);
        self.w.bits |= ((value as u32) & 7) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `UART_IFLS_RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_IFLS_RXR {
    #[doc = "RX FIFO >= 1/8 full"]
    UART_IFLS_RX1_8,
    #[doc = "RX FIFO >= 1/4 full"]
    UART_IFLS_RX2_8,
    #[doc = "RX FIFO >= 1/2 full (default)"]
    UART_IFLS_RX4_8,
    #[doc = "RX FIFO >= 3/4 full"]
    UART_IFLS_RX6_8,
    #[doc = "RX FIFO >= 7/8 full"]
    UART_IFLS_RX7_8,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl UART_IFLS_RXR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            UART_IFLS_RXR::UART_IFLS_RX1_8 => 0,
            UART_IFLS_RXR::UART_IFLS_RX2_8 => 1,
            UART_IFLS_RXR::UART_IFLS_RX4_8 => 2,
            UART_IFLS_RXR::UART_IFLS_RX6_8 => 3,
            UART_IFLS_RXR::UART_IFLS_RX7_8 => 4,
            UART_IFLS_RXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> UART_IFLS_RXR {
        match value {
            0 => UART_IFLS_RXR::UART_IFLS_RX1_8,
            1 => UART_IFLS_RXR::UART_IFLS_RX2_8,
            2 => UART_IFLS_RXR::UART_IFLS_RX4_8,
            3 => UART_IFLS_RXR::UART_IFLS_RX6_8,
            4 => UART_IFLS_RXR::UART_IFLS_RX7_8,
            i => UART_IFLS_RXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_RX1_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_rx1_8(&self) -> bool {
        *self == UART_IFLS_RXR::UART_IFLS_RX1_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_RX2_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_rx2_8(&self) -> bool {
        *self == UART_IFLS_RXR::UART_IFLS_RX2_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_RX4_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_rx4_8(&self) -> bool {
        *self == UART_IFLS_RXR::UART_IFLS_RX4_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_RX6_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_rx6_8(&self) -> bool {
        *self == UART_IFLS_RXR::UART_IFLS_RX6_8
    }
    #[doc = "Checks if the value of the field is `UART_IFLS_RX7_8`"]
    #[inline(always)]
    pub fn is_uart_ifls_rx7_8(&self) -> bool {
        *self == UART_IFLS_RXR::UART_IFLS_RX7_8
    }
}
#[doc = "Values that can be written to the field `UART_IFLS_RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_IFLS_RXW {
    #[doc = "RX FIFO >= 1/8 full"]
    UART_IFLS_RX1_8,
    #[doc = "RX FIFO >= 1/4 full"]
    UART_IFLS_RX2_8,
    #[doc = "RX FIFO >= 1/2 full (default)"]
    UART_IFLS_RX4_8,
    #[doc = "RX FIFO >= 3/4 full"]
    UART_IFLS_RX6_8,
    #[doc = "RX FIFO >= 7/8 full"]
    UART_IFLS_RX7_8,
}
impl UART_IFLS_RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            UART_IFLS_RXW::UART_IFLS_RX1_8 => 0,
            UART_IFLS_RXW::UART_IFLS_RX2_8 => 1,
            UART_IFLS_RXW::UART_IFLS_RX4_8 => 2,
            UART_IFLS_RXW::UART_IFLS_RX6_8 => 3,
            UART_IFLS_RXW::UART_IFLS_RX7_8 => 4,
        }
    }
}
#[doc = r"Proxy"]
pub struct _UART_IFLS_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_IFLS_RXW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_IFLS_RXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "RX FIFO >= 1/8 full"]
    #[inline(always)]
    pub fn uart_ifls_rx1_8(self) -> &'a mut W {
        self.variant(UART_IFLS_RXW::UART_IFLS_RX1_8)
    }
    #[doc = "RX FIFO >= 1/4 full"]
    #[inline(always)]
    pub fn uart_ifls_rx2_8(self) -> &'a mut W {
        self.variant(UART_IFLS_RXW::UART_IFLS_RX2_8)
    }
    #[doc = "RX FIFO >= 1/2 full (default)"]
    #[inline(always)]
    pub fn uart_ifls_rx4_8(self) -> &'a mut W {
        self.variant(UART_IFLS_RXW::UART_IFLS_RX4_8)
    }
    #[doc = "RX FIFO >= 3/4 full"]
    #[inline(always)]
    pub fn uart_ifls_rx6_8(self) -> &'a mut W {
        self.variant(UART_IFLS_RXW::UART_IFLS_RX6_8)
    }
    #[doc = "RX FIFO >= 7/8 full"]
    #[inline(always)]
    pub fn uart_ifls_rx7_8(self) -> &'a mut W {
        self.variant(UART_IFLS_RXW::UART_IFLS_RX7_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 3);
        self.w.bits |= ((value as u32) & 7) << 3;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn uart_ifls_tx(&self) -> UART_IFLS_TXR {
        UART_IFLS_TXR::_from(((self.bits >> 0) & 7) as u8)
    }
    #[doc = "Bits 3:5 - UART Receive Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn uart_ifls_rx(&self) -> UART_IFLS_RXR {
        UART_IFLS_RXR::_from(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn uart_ifls_tx(&mut self) -> _UART_IFLS_TXW {
        _UART_IFLS_TXW { w: self }
    }
    #[doc = "Bits 3:5 - UART Receive Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn uart_ifls_rx(&mut self) -> _UART_IFLS_RXW {
        _UART_IFLS_RXW { w: self }
    }
}

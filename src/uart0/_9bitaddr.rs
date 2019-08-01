#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::_9BITADDR {
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
#[doc = r"Value of the field"]
pub struct UART_9BITADDR_ADDRR {
    bits: u8,
}
impl UART_9BITADDR_ADDRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UART_9BITADDR_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_9BITADDR_ADDRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 0);
        self.w.bits |= ((value as u32) & 255) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UART_9BITADDR_9BITENR {
    bits: bool,
}
impl UART_9BITADDR_9BITENR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _UART_9BITADDR_9BITENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_9BITADDR_9BITENW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(1 << 15);
        self.w.bits |= ((value as u32) & 1) << 15;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Self Address for 9-Bit Mode"]
    #[inline(always)]
    pub fn uart_9bitaddr_addr(&self) -> UART_9BITADDR_ADDRR {
        let bits = ((self.bits >> 0) & 255) as u8;
        UART_9BITADDR_ADDRR { bits }
    }
    #[doc = "Bit 15 - Enable 9-Bit Mode"]
    #[inline(always)]
    pub fn uart_9bitaddr_9biten(&self) -> UART_9BITADDR_9BITENR {
        let bits = ((self.bits >> 15) & 1) != 0;
        UART_9BITADDR_9BITENR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Self Address for 9-Bit Mode"]
    #[inline(always)]
    pub fn uart_9bitaddr_addr(&mut self) -> _UART_9BITADDR_ADDRW {
        _UART_9BITADDR_ADDRW { w: self }
    }
    #[doc = "Bit 15 - Enable 9-Bit Mode"]
    #[inline(always)]
    pub fn uart_9bitaddr_9biten(&mut self) -> _UART_9BITADDR_9BITENW {
        _UART_9BITADDR_9BITENW { w: self }
    }
}

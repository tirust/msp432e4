#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DR {
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
pub struct UART_DR_DATAR {
    bits: u8,
}
impl UART_DR_DATAR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UART_DR_DATAW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_DR_DATAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 0);
        self.w.bits |= ((value as u32) & 255) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UART_DR_FER {
    bits: bool,
}
impl UART_DR_FER {
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
pub struct _UART_DR_FEW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_DR_FEW<'a> {
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
        self.w.bits &= !(1 << 8);
        self.w.bits |= ((value as u32) & 1) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UART_DR_PER {
    bits: bool,
}
impl UART_DR_PER {
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
pub struct _UART_DR_PEW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_DR_PEW<'a> {
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
        self.w.bits &= !(1 << 9);
        self.w.bits |= ((value as u32) & 1) << 9;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UART_DR_BER {
    bits: bool,
}
impl UART_DR_BER {
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
pub struct _UART_DR_BEW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_DR_BEW<'a> {
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
        self.w.bits &= !(1 << 10);
        self.w.bits |= ((value as u32) & 1) << 10;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UART_DR_OER {
    bits: bool,
}
impl UART_DR_OER {
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
pub struct _UART_DR_OEW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_DR_OEW<'a> {
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
        self.w.bits &= !(1 << 11);
        self.w.bits |= ((value as u32) & 1) << 11;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Data Transmitted or Received"]
    #[inline(always)]
    pub fn uart_dr_data(&self) -> UART_DR_DATAR {
        let bits = ((self.bits >> 0) & 255) as u8;
        UART_DR_DATAR { bits }
    }
    #[doc = "Bit 8 - UART Framing Error"]
    #[inline(always)]
    pub fn uart_dr_fe(&self) -> UART_DR_FER {
        let bits = ((self.bits >> 8) & 1) != 0;
        UART_DR_FER { bits }
    }
    #[doc = "Bit 9 - UART Parity Error"]
    #[inline(always)]
    pub fn uart_dr_pe(&self) -> UART_DR_PER {
        let bits = ((self.bits >> 9) & 1) != 0;
        UART_DR_PER { bits }
    }
    #[doc = "Bit 10 - UART Break Error"]
    #[inline(always)]
    pub fn uart_dr_be(&self) -> UART_DR_BER {
        let bits = ((self.bits >> 10) & 1) != 0;
        UART_DR_BER { bits }
    }
    #[doc = "Bit 11 - UART Overrun Error"]
    #[inline(always)]
    pub fn uart_dr_oe(&self) -> UART_DR_OER {
        let bits = ((self.bits >> 11) & 1) != 0;
        UART_DR_OER { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Data Transmitted or Received"]
    #[inline(always)]
    pub fn uart_dr_data(&mut self) -> _UART_DR_DATAW {
        _UART_DR_DATAW { w: self }
    }
    #[doc = "Bit 8 - UART Framing Error"]
    #[inline(always)]
    pub fn uart_dr_fe(&mut self) -> _UART_DR_FEW {
        _UART_DR_FEW { w: self }
    }
    #[doc = "Bit 9 - UART Parity Error"]
    #[inline(always)]
    pub fn uart_dr_pe(&mut self) -> _UART_DR_PEW {
        _UART_DR_PEW { w: self }
    }
    #[doc = "Bit 10 - UART Break Error"]
    #[inline(always)]
    pub fn uart_dr_be(&mut self) -> _UART_DR_BEW {
        _UART_DR_BEW { w: self }
    }
    #[doc = "Bit 11 - UART Overrun Error"]
    #[inline(always)]
    pub fn uart_dr_oe(&mut self) -> _UART_DR_OEW {
        _UART_DR_OEW { w: self }
    }
}

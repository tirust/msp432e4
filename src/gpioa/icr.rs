#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICR {
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
#[doc = r"Proxy"]
pub struct _GPIO_ICR_GPIOW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_ICR_GPIOW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 0);
        self.w.bits |= ((value as u32) & 255) << 0;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _GPIO_ICR_DMAICW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_ICR_DMAICW<'a> {
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
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - GPIO Interrupt Clear"]
    #[inline(always)]
    pub fn gpio_icr_gpio(&mut self) -> _GPIO_ICR_GPIOW {
        _GPIO_ICR_GPIOW { w: self }
    }
    #[doc = "Bit 8 - GPIO uDMA Interrupt Clear"]
    #[inline(always)]
    pub fn gpio_icr_dmaic(&mut self) -> _GPIO_ICR_DMAICW {
        _GPIO_ICR_DMAICW { w: self }
    }
}

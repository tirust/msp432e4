#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WDOGTO {
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
pub struct EMAC_WDOGTO_WTOR {
    bits: u16,
}
impl EMAC_WDOGTO_WTOR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_WDOGTO_WTOW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_WDOGTO_WTOW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(16383 << 0);
        self.w.bits |= ((value as u32) & 16383) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_WDOGTO_PWER {
    bits: bool,
}
impl EMAC_WDOGTO_PWER {
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
pub struct _EMAC_WDOGTO_PWEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_WDOGTO_PWEW<'a> {
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
        self.w.bits &= !(1 << 16);
        self.w.bits |= ((value as u32) & 1) << 16;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:13 - Watchdog Timeout"]
    #[inline(always)]
    pub fn emac_wdogto_wto(&self) -> EMAC_WDOGTO_WTOR {
        let bits = ((self.bits >> 0) & 16383) as u16;
        EMAC_WDOGTO_WTOR { bits }
    }
    #[doc = "Bit 16 - Programmable Watchdog Enable"]
    #[inline(always)]
    pub fn emac_wdogto_pwe(&self) -> EMAC_WDOGTO_PWER {
        let bits = ((self.bits >> 16) & 1) != 0;
        EMAC_WDOGTO_PWER { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:13 - Watchdog Timeout"]
    #[inline(always)]
    pub fn emac_wdogto_wto(&mut self) -> _EMAC_WDOGTO_WTOW {
        _EMAC_WDOGTO_WTOW { w: self }
    }
    #[doc = "Bit 16 - Programmable Watchdog Enable"]
    #[inline(always)]
    pub fn emac_wdogto_pwe(&mut self) -> _EMAC_WDOGTO_PWEW {
        _EMAC_WDOGTO_PWEW { w: self }
    }
}

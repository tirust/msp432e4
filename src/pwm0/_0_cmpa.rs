#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::_0_CMPA {
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
pub struct PWM_0_CMPAR {
    bits: u16,
}
impl PWM_0_CMPAR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _PWM_0_CMPAW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_0_CMPAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(65535 << 0);
        self.w.bits |= ((value as u32) & 65535) << 0;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Comparator A Value"]
    #[inline(always)]
    pub fn pwm_0_cmpa(&self) -> PWM_0_CMPAR {
        let bits = ((self.bits >> 0) & 65535) as u16;
        PWM_0_CMPAR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Comparator A Value"]
    #[inline(always)]
    pub fn pwm_0_cmpa(&mut self) -> _PWM_0_CMPAW {
        _PWM_0_CMPAW { w: self }
    }
}

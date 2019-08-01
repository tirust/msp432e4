#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWRTC {
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
pub struct SYSCTL_PWRTC_VDD_UBORR {
    bits: bool,
}
impl SYSCTL_PWRTC_VDD_UBORR {
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
pub struct _SYSCTL_PWRTC_VDD_UBORW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PWRTC_VDD_UBORW<'a> {
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
        self.w.bits &= !(1 << 0);
        self.w.bits |= ((value as u32) & 1) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_PWRTC_VDDA_UBORR {
    bits: bool,
}
impl SYSCTL_PWRTC_VDDA_UBORR {
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
pub struct _SYSCTL_PWRTC_VDDA_UBORW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PWRTC_VDDA_UBORW<'a> {
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
        self.w.bits &= !(1 << 4);
        self.w.bits |= ((value as u32) & 1) << 4;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - VDD Under BOR Status"]
    #[inline(always)]
    pub fn sysctl_pwrtc_vdd_ubor(&self) -> SYSCTL_PWRTC_VDD_UBORR {
        let bits = ((self.bits >> 0) & 1) != 0;
        SYSCTL_PWRTC_VDD_UBORR { bits }
    }
    #[doc = "Bit 4 - VDDA Under BOR Status"]
    #[inline(always)]
    pub fn sysctl_pwrtc_vdda_ubor(&self) -> SYSCTL_PWRTC_VDDA_UBORR {
        let bits = ((self.bits >> 4) & 1) != 0;
        SYSCTL_PWRTC_VDDA_UBORR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - VDD Under BOR Status"]
    #[inline(always)]
    pub fn sysctl_pwrtc_vdd_ubor(&mut self) -> _SYSCTL_PWRTC_VDD_UBORW {
        _SYSCTL_PWRTC_VDD_UBORW { w: self }
    }
    #[doc = "Bit 4 - VDDA Under BOR Status"]
    #[inline(always)]
    pub fn sysctl_pwrtc_vdda_ubor(&mut self) -> _SYSCTL_PWRTC_VDDA_UBORW {
        _SYSCTL_PWRTC_VDDA_UBORW { w: self }
    }
}
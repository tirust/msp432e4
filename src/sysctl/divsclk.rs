#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DIVSCLK {
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
pub struct SYSCTL_DIVSCLK_DIVR {
    bits: u8,
}
impl SYSCTL_DIVSCLK_DIVR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DIVSCLK_DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DIVSCLK_DIVW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 0);
        self.w.bits |= ((value as u32) & 255) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_DIVSCLK_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DIVSCLK_SRCR {
    #[doc = "System Clock"]
    SYSCTL_DIVSCLK_SRC_SYSCLK,
    #[doc = "PIOSC"]
    SYSCTL_DIVSCLK_SRC_PIOSC,
    #[doc = "MOSC"]
    SYSCTL_DIVSCLK_SRC_MOSC,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_DIVSCLK_SRCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_DIVSCLK_SRCR::SYSCTL_DIVSCLK_SRC_SYSCLK => 0,
            SYSCTL_DIVSCLK_SRCR::SYSCTL_DIVSCLK_SRC_PIOSC => 1,
            SYSCTL_DIVSCLK_SRCR::SYSCTL_DIVSCLK_SRC_MOSC => 2,
            SYSCTL_DIVSCLK_SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_DIVSCLK_SRCR {
        match value {
            0 => SYSCTL_DIVSCLK_SRCR::SYSCTL_DIVSCLK_SRC_SYSCLK,
            1 => SYSCTL_DIVSCLK_SRCR::SYSCTL_DIVSCLK_SRC_PIOSC,
            2 => SYSCTL_DIVSCLK_SRCR::SYSCTL_DIVSCLK_SRC_MOSC,
            i => SYSCTL_DIVSCLK_SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DIVSCLK_SRC_SYSCLK`"]
    #[inline(always)]
    pub fn is_sysctl_divsclk_src_sysclk(&self) -> bool {
        *self == SYSCTL_DIVSCLK_SRCR::SYSCTL_DIVSCLK_SRC_SYSCLK
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DIVSCLK_SRC_PIOSC`"]
    #[inline(always)]
    pub fn is_sysctl_divsclk_src_piosc(&self) -> bool {
        *self == SYSCTL_DIVSCLK_SRCR::SYSCTL_DIVSCLK_SRC_PIOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DIVSCLK_SRC_MOSC`"]
    #[inline(always)]
    pub fn is_sysctl_divsclk_src_mosc(&self) -> bool {
        *self == SYSCTL_DIVSCLK_SRCR::SYSCTL_DIVSCLK_SRC_MOSC
    }
}
#[doc = "Values that can be written to the field `SYSCTL_DIVSCLK_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DIVSCLK_SRCW {
    #[doc = "System Clock"]
    SYSCTL_DIVSCLK_SRC_SYSCLK,
    #[doc = "PIOSC"]
    SYSCTL_DIVSCLK_SRC_PIOSC,
    #[doc = "MOSC"]
    SYSCTL_DIVSCLK_SRC_MOSC,
}
impl SYSCTL_DIVSCLK_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_DIVSCLK_SRCW::SYSCTL_DIVSCLK_SRC_SYSCLK => 0,
            SYSCTL_DIVSCLK_SRCW::SYSCTL_DIVSCLK_SRC_PIOSC => 1,
            SYSCTL_DIVSCLK_SRCW::SYSCTL_DIVSCLK_SRC_MOSC => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DIVSCLK_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DIVSCLK_SRCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DIVSCLK_SRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "System Clock"]
    #[inline(always)]
    pub fn sysctl_divsclk_src_sysclk(self) -> &'a mut W {
        self.variant(SYSCTL_DIVSCLK_SRCW::SYSCTL_DIVSCLK_SRC_SYSCLK)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn sysctl_divsclk_src_piosc(self) -> &'a mut W {
        self.variant(SYSCTL_DIVSCLK_SRCW::SYSCTL_DIVSCLK_SRC_PIOSC)
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn sysctl_divsclk_src_mosc(self) -> &'a mut W {
        self.variant(SYSCTL_DIVSCLK_SRCW::SYSCTL_DIVSCLK_SRC_MOSC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 16);
        self.w.bits |= ((value as u32) & 3) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_DIVSCLK_ENR {
    bits: bool,
}
impl SYSCTL_DIVSCLK_ENR {
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
pub struct _SYSCTL_DIVSCLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DIVSCLK_ENW<'a> {
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
        self.w.bits &= !(1 << 31);
        self.w.bits |= ((value as u32) & 1) << 31;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Divisor Value"]
    #[inline(always)]
    pub fn sysctl_divsclk_div(&self) -> SYSCTL_DIVSCLK_DIVR {
        let bits = ((self.bits >> 0) & 255) as u8;
        SYSCTL_DIVSCLK_DIVR { bits }
    }
    #[doc = "Bits 16:17 - Clock Source"]
    #[inline(always)]
    pub fn sysctl_divsclk_src(&self) -> SYSCTL_DIVSCLK_SRCR {
        SYSCTL_DIVSCLK_SRCR::_from(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 31 - DIVSCLK Enable"]
    #[inline(always)]
    pub fn sysctl_divsclk_en(&self) -> SYSCTL_DIVSCLK_ENR {
        let bits = ((self.bits >> 31) & 1) != 0;
        SYSCTL_DIVSCLK_ENR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Divisor Value"]
    #[inline(always)]
    pub fn sysctl_divsclk_div(&mut self) -> _SYSCTL_DIVSCLK_DIVW {
        _SYSCTL_DIVSCLK_DIVW { w: self }
    }
    #[doc = "Bits 16:17 - Clock Source"]
    #[inline(always)]
    pub fn sysctl_divsclk_src(&mut self) -> _SYSCTL_DIVSCLK_SRCW {
        _SYSCTL_DIVSCLK_SRCW { w: self }
    }
    #[doc = "Bit 31 - DIVSCLK Enable"]
    #[inline(always)]
    pub fn sysctl_divsclk_en(&mut self) -> _SYSCTL_DIVSCLK_ENW {
        _SYSCTL_DIVSCLK_ENW { w: self }
    }
}

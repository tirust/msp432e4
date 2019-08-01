#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ALTCLKCFG {
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
#[doc = "Possible values of the field `SYSCTL_ALTCLKCFG_ALTCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_ALTCLKCFG_ALTCLKR {
    #[doc = "PIOSC"]
    SYSCTL_ALTCLKCFG_ALTCLK_PIOSC,
    #[doc = "Hibernation Module Real-time clock output (RTCOSC)"]
    SYSCTL_ALTCLKCFG_ALTCLK_RTCOSC,
    #[doc = "Low-frequency internal oscillator (LFIOSC)"]
    SYSCTL_ALTCLKCFG_ALTCLK_LFIOSC,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_ALTCLKCFG_ALTCLKR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_ALTCLKCFG_ALTCLKR::SYSCTL_ALTCLKCFG_ALTCLK_PIOSC => 0,
            SYSCTL_ALTCLKCFG_ALTCLKR::SYSCTL_ALTCLKCFG_ALTCLK_RTCOSC => 3,
            SYSCTL_ALTCLKCFG_ALTCLKR::SYSCTL_ALTCLKCFG_ALTCLK_LFIOSC => 4,
            SYSCTL_ALTCLKCFG_ALTCLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_ALTCLKCFG_ALTCLKR {
        match value {
            0 => SYSCTL_ALTCLKCFG_ALTCLKR::SYSCTL_ALTCLKCFG_ALTCLK_PIOSC,
            3 => SYSCTL_ALTCLKCFG_ALTCLKR::SYSCTL_ALTCLKCFG_ALTCLK_RTCOSC,
            4 => SYSCTL_ALTCLKCFG_ALTCLKR::SYSCTL_ALTCLKCFG_ALTCLK_LFIOSC,
            i => SYSCTL_ALTCLKCFG_ALTCLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_ALTCLKCFG_ALTCLK_PIOSC`"]
    #[inline(always)]
    pub fn is_sysctl_altclkcfg_altclk_piosc(&self) -> bool {
        *self == SYSCTL_ALTCLKCFG_ALTCLKR::SYSCTL_ALTCLKCFG_ALTCLK_PIOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_ALTCLKCFG_ALTCLK_RTCOSC`"]
    #[inline(always)]
    pub fn is_sysctl_altclkcfg_altclk_rtcosc(&self) -> bool {
        *self == SYSCTL_ALTCLKCFG_ALTCLKR::SYSCTL_ALTCLKCFG_ALTCLK_RTCOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_ALTCLKCFG_ALTCLK_LFIOSC`"]
    #[inline(always)]
    pub fn is_sysctl_altclkcfg_altclk_lfiosc(&self) -> bool {
        *self == SYSCTL_ALTCLKCFG_ALTCLKR::SYSCTL_ALTCLKCFG_ALTCLK_LFIOSC
    }
}
#[doc = "Values that can be written to the field `SYSCTL_ALTCLKCFG_ALTCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_ALTCLKCFG_ALTCLKW {
    #[doc = "PIOSC"]
    SYSCTL_ALTCLKCFG_ALTCLK_PIOSC,
    #[doc = "Hibernation Module Real-time clock output (RTCOSC)"]
    SYSCTL_ALTCLKCFG_ALTCLK_RTCOSC,
    #[doc = "Low-frequency internal oscillator (LFIOSC)"]
    SYSCTL_ALTCLKCFG_ALTCLK_LFIOSC,
}
impl SYSCTL_ALTCLKCFG_ALTCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_ALTCLKCFG_ALTCLKW::SYSCTL_ALTCLKCFG_ALTCLK_PIOSC => 0,
            SYSCTL_ALTCLKCFG_ALTCLKW::SYSCTL_ALTCLKCFG_ALTCLK_RTCOSC => 3,
            SYSCTL_ALTCLKCFG_ALTCLKW::SYSCTL_ALTCLKCFG_ALTCLK_LFIOSC => 4,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_ALTCLKCFG_ALTCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_ALTCLKCFG_ALTCLKW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_ALTCLKCFG_ALTCLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn sysctl_altclkcfg_altclk_piosc(self) -> &'a mut W {
        self.variant(SYSCTL_ALTCLKCFG_ALTCLKW::SYSCTL_ALTCLKCFG_ALTCLK_PIOSC)
    }
    #[doc = "Hibernation Module Real-time clock output (RTCOSC)"]
    #[inline(always)]
    pub fn sysctl_altclkcfg_altclk_rtcosc(self) -> &'a mut W {
        self.variant(SYSCTL_ALTCLKCFG_ALTCLKW::SYSCTL_ALTCLKCFG_ALTCLK_RTCOSC)
    }
    #[doc = "Low-frequency internal oscillator (LFIOSC)"]
    #[inline(always)]
    pub fn sysctl_altclkcfg_altclk_lfiosc(self) -> &'a mut W {
        self.variant(SYSCTL_ALTCLKCFG_ALTCLKW::SYSCTL_ALTCLKCFG_ALTCLK_LFIOSC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Alternate Clock Source"]
    #[inline(always)]
    pub fn sysctl_altclkcfg_altclk(&self) -> SYSCTL_ALTCLKCFG_ALTCLKR {
        SYSCTL_ALTCLKCFG_ALTCLKR::_from(((self.bits >> 0) & 15) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Alternate Clock Source"]
    #[inline(always)]
    pub fn sysctl_altclkcfg_altclk(&mut self) -> _SYSCTL_ALTCLKCFG_ALTCLKW {
        _SYSCTL_ALTCLKCFG_ALTCLKW { w: self }
    }
}

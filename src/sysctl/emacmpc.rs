#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EMACMPC {
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
#[doc = "Possible values of the field `SYSCTL_EMACMPC_PWRCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_EMACMPC_PWRCTLR {
    #[doc = "Array OFF"]
    SYSCTL_EMACMPC_PWRCTL_OFF,
    #[doc = "Array On"]
    SYSCTL_EMACMPC_PWRCTL_ON,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_EMACMPC_PWRCTLR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_EMACMPC_PWRCTLR::SYSCTL_EMACMPC_PWRCTL_OFF => 0,
            SYSCTL_EMACMPC_PWRCTLR::SYSCTL_EMACMPC_PWRCTL_ON => 3,
            SYSCTL_EMACMPC_PWRCTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_EMACMPC_PWRCTLR {
        match value {
            0 => SYSCTL_EMACMPC_PWRCTLR::SYSCTL_EMACMPC_PWRCTL_OFF,
            3 => SYSCTL_EMACMPC_PWRCTLR::SYSCTL_EMACMPC_PWRCTL_ON,
            i => SYSCTL_EMACMPC_PWRCTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_EMACMPC_PWRCTL_OFF`"]
    #[inline(always)]
    pub fn is_sysctl_emacmpc_pwrctl_off(&self) -> bool {
        *self == SYSCTL_EMACMPC_PWRCTLR::SYSCTL_EMACMPC_PWRCTL_OFF
    }
    #[doc = "Checks if the value of the field is `SYSCTL_EMACMPC_PWRCTL_ON`"]
    #[inline(always)]
    pub fn is_sysctl_emacmpc_pwrctl_on(&self) -> bool {
        *self == SYSCTL_EMACMPC_PWRCTLR::SYSCTL_EMACMPC_PWRCTL_ON
    }
}
#[doc = "Values that can be written to the field `SYSCTL_EMACMPC_PWRCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_EMACMPC_PWRCTLW {
    #[doc = "Array OFF"]
    SYSCTL_EMACMPC_PWRCTL_OFF,
    #[doc = "Array On"]
    SYSCTL_EMACMPC_PWRCTL_ON,
}
impl SYSCTL_EMACMPC_PWRCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_EMACMPC_PWRCTLW::SYSCTL_EMACMPC_PWRCTL_OFF => 0,
            SYSCTL_EMACMPC_PWRCTLW::SYSCTL_EMACMPC_PWRCTL_ON => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_EMACMPC_PWRCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_EMACMPC_PWRCTLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_EMACMPC_PWRCTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Array OFF"]
    #[inline(always)]
    pub fn sysctl_emacmpc_pwrctl_off(self) -> &'a mut W {
        self.variant(SYSCTL_EMACMPC_PWRCTLW::SYSCTL_EMACMPC_PWRCTL_OFF)
    }
    #[doc = "Array On"]
    #[inline(always)]
    pub fn sysctl_emacmpc_pwrctl_on(self) -> &'a mut W {
        self.variant(SYSCTL_EMACMPC_PWRCTLW::SYSCTL_EMACMPC_PWRCTL_ON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Memory Array Power Control"]
    #[inline(always)]
    pub fn sysctl_emacmpc_pwrctl(&self) -> SYSCTL_EMACMPC_PWRCTLR {
        SYSCTL_EMACMPC_PWRCTLR::_from(((self.bits >> 0) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Memory Array Power Control"]
    #[inline(always)]
    pub fn sysctl_emacmpc_pwrctl(&mut self) -> _SYSCTL_EMACMPC_PWRCTLW {
        _SYSCTL_EMACMPC_PWRCTLW { w: self }
    }
}

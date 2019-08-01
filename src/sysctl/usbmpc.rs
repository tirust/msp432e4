#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBMPC {
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
#[doc = "Possible values of the field `SYSCTL_USBMPC_PWRCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_USBMPC_PWRCTLR {
    #[doc = "Array OFF"]
    SYSCTL_USBMPC_PWRCTL_OFF,
    #[doc = "SRAM Retention"]
    SYSCTL_USBMPC_PWRCTL_RETAIN,
    #[doc = "Array On"]
    SYSCTL_USBMPC_PWRCTL_ON,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_USBMPC_PWRCTLR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_USBMPC_PWRCTLR::SYSCTL_USBMPC_PWRCTL_OFF => 0,
            SYSCTL_USBMPC_PWRCTLR::SYSCTL_USBMPC_PWRCTL_RETAIN => 1,
            SYSCTL_USBMPC_PWRCTLR::SYSCTL_USBMPC_PWRCTL_ON => 3,
            SYSCTL_USBMPC_PWRCTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_USBMPC_PWRCTLR {
        match value {
            0 => SYSCTL_USBMPC_PWRCTLR::SYSCTL_USBMPC_PWRCTL_OFF,
            1 => SYSCTL_USBMPC_PWRCTLR::SYSCTL_USBMPC_PWRCTL_RETAIN,
            3 => SYSCTL_USBMPC_PWRCTLR::SYSCTL_USBMPC_PWRCTL_ON,
            i => SYSCTL_USBMPC_PWRCTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_USBMPC_PWRCTL_OFF`"]
    #[inline(always)]
    pub fn is_sysctl_usbmpc_pwrctl_off(&self) -> bool {
        *self == SYSCTL_USBMPC_PWRCTLR::SYSCTL_USBMPC_PWRCTL_OFF
    }
    #[doc = "Checks if the value of the field is `SYSCTL_USBMPC_PWRCTL_RETAIN`"]
    #[inline(always)]
    pub fn is_sysctl_usbmpc_pwrctl_retain(&self) -> bool {
        *self == SYSCTL_USBMPC_PWRCTLR::SYSCTL_USBMPC_PWRCTL_RETAIN
    }
    #[doc = "Checks if the value of the field is `SYSCTL_USBMPC_PWRCTL_ON`"]
    #[inline(always)]
    pub fn is_sysctl_usbmpc_pwrctl_on(&self) -> bool {
        *self == SYSCTL_USBMPC_PWRCTLR::SYSCTL_USBMPC_PWRCTL_ON
    }
}
#[doc = "Values that can be written to the field `SYSCTL_USBMPC_PWRCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_USBMPC_PWRCTLW {
    #[doc = "Array OFF"]
    SYSCTL_USBMPC_PWRCTL_OFF,
    #[doc = "SRAM Retention"]
    SYSCTL_USBMPC_PWRCTL_RETAIN,
    #[doc = "Array On"]
    SYSCTL_USBMPC_PWRCTL_ON,
}
impl SYSCTL_USBMPC_PWRCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_USBMPC_PWRCTLW::SYSCTL_USBMPC_PWRCTL_OFF => 0,
            SYSCTL_USBMPC_PWRCTLW::SYSCTL_USBMPC_PWRCTL_RETAIN => 1,
            SYSCTL_USBMPC_PWRCTLW::SYSCTL_USBMPC_PWRCTL_ON => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_USBMPC_PWRCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_USBMPC_PWRCTLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_USBMPC_PWRCTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Array OFF"]
    #[inline(always)]
    pub fn sysctl_usbmpc_pwrctl_off(self) -> &'a mut W {
        self.variant(SYSCTL_USBMPC_PWRCTLW::SYSCTL_USBMPC_PWRCTL_OFF)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub fn sysctl_usbmpc_pwrctl_retain(self) -> &'a mut W {
        self.variant(SYSCTL_USBMPC_PWRCTLW::SYSCTL_USBMPC_PWRCTL_RETAIN)
    }
    #[doc = "Array On"]
    #[inline(always)]
    pub fn sysctl_usbmpc_pwrctl_on(self) -> &'a mut W {
        self.variant(SYSCTL_USBMPC_PWRCTLW::SYSCTL_USBMPC_PWRCTL_ON)
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
    pub fn sysctl_usbmpc_pwrctl(&self) -> SYSCTL_USBMPC_PWRCTLR {
        SYSCTL_USBMPC_PWRCTLR::_from(((self.bits >> 0) & 3) as u8)
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
    pub fn sysctl_usbmpc_pwrctl(&mut self) -> _SYSCTL_USBMPC_PWRCTLW {
        _SYSCTL_USBMPC_PWRCTLW { w: self }
    }
}

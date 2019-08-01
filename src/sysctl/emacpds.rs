#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EMACPDS {
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
#[doc = "Possible values of the field `SYSCTL_EMACPDS_PWRSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_EMACPDS_PWRSTATR {
    #[doc = "OFF"]
    SYSCTL_EMACPDS_PWRSTAT_OFF,
    #[doc = "ON"]
    SYSCTL_EMACPDS_PWRSTAT_ON,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_EMACPDS_PWRSTATR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_EMACPDS_PWRSTATR::SYSCTL_EMACPDS_PWRSTAT_OFF => 0,
            SYSCTL_EMACPDS_PWRSTATR::SYSCTL_EMACPDS_PWRSTAT_ON => 3,
            SYSCTL_EMACPDS_PWRSTATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_EMACPDS_PWRSTATR {
        match value {
            0 => SYSCTL_EMACPDS_PWRSTATR::SYSCTL_EMACPDS_PWRSTAT_OFF,
            3 => SYSCTL_EMACPDS_PWRSTATR::SYSCTL_EMACPDS_PWRSTAT_ON,
            i => SYSCTL_EMACPDS_PWRSTATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_EMACPDS_PWRSTAT_OFF`"]
    #[inline(always)]
    pub fn is_sysctl_emacpds_pwrstat_off(&self) -> bool {
        *self == SYSCTL_EMACPDS_PWRSTATR::SYSCTL_EMACPDS_PWRSTAT_OFF
    }
    #[doc = "Checks if the value of the field is `SYSCTL_EMACPDS_PWRSTAT_ON`"]
    #[inline(always)]
    pub fn is_sysctl_emacpds_pwrstat_on(&self) -> bool {
        *self == SYSCTL_EMACPDS_PWRSTATR::SYSCTL_EMACPDS_PWRSTAT_ON
    }
}
#[doc = "Values that can be written to the field `SYSCTL_EMACPDS_PWRSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_EMACPDS_PWRSTATW {
    #[doc = "OFF"]
    SYSCTL_EMACPDS_PWRSTAT_OFF,
    #[doc = "ON"]
    SYSCTL_EMACPDS_PWRSTAT_ON,
}
impl SYSCTL_EMACPDS_PWRSTATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_EMACPDS_PWRSTATW::SYSCTL_EMACPDS_PWRSTAT_OFF => 0,
            SYSCTL_EMACPDS_PWRSTATW::SYSCTL_EMACPDS_PWRSTAT_ON => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_EMACPDS_PWRSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_EMACPDS_PWRSTATW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_EMACPDS_PWRSTATW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "OFF"]
    #[inline(always)]
    pub fn sysctl_emacpds_pwrstat_off(self) -> &'a mut W {
        self.variant(SYSCTL_EMACPDS_PWRSTATW::SYSCTL_EMACPDS_PWRSTAT_OFF)
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn sysctl_emacpds_pwrstat_on(self) -> &'a mut W {
        self.variant(SYSCTL_EMACPDS_PWRSTATW::SYSCTL_EMACPDS_PWRSTAT_ON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_EMACPDS_MEMSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_EMACPDS_MEMSTATR {
    #[doc = "Array OFF"]
    SYSCTL_EMACPDS_MEMSTAT_OFF,
    #[doc = "Array On"]
    SYSCTL_EMACPDS_MEMSTAT_ON,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_EMACPDS_MEMSTATR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_EMACPDS_MEMSTATR::SYSCTL_EMACPDS_MEMSTAT_OFF => 0,
            SYSCTL_EMACPDS_MEMSTATR::SYSCTL_EMACPDS_MEMSTAT_ON => 3,
            SYSCTL_EMACPDS_MEMSTATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_EMACPDS_MEMSTATR {
        match value {
            0 => SYSCTL_EMACPDS_MEMSTATR::SYSCTL_EMACPDS_MEMSTAT_OFF,
            3 => SYSCTL_EMACPDS_MEMSTATR::SYSCTL_EMACPDS_MEMSTAT_ON,
            i => SYSCTL_EMACPDS_MEMSTATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_EMACPDS_MEMSTAT_OFF`"]
    #[inline(always)]
    pub fn is_sysctl_emacpds_memstat_off(&self) -> bool {
        *self == SYSCTL_EMACPDS_MEMSTATR::SYSCTL_EMACPDS_MEMSTAT_OFF
    }
    #[doc = "Checks if the value of the field is `SYSCTL_EMACPDS_MEMSTAT_ON`"]
    #[inline(always)]
    pub fn is_sysctl_emacpds_memstat_on(&self) -> bool {
        *self == SYSCTL_EMACPDS_MEMSTATR::SYSCTL_EMACPDS_MEMSTAT_ON
    }
}
#[doc = "Values that can be written to the field `SYSCTL_EMACPDS_MEMSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_EMACPDS_MEMSTATW {
    #[doc = "Array OFF"]
    SYSCTL_EMACPDS_MEMSTAT_OFF,
    #[doc = "Array On"]
    SYSCTL_EMACPDS_MEMSTAT_ON,
}
impl SYSCTL_EMACPDS_MEMSTATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_EMACPDS_MEMSTATW::SYSCTL_EMACPDS_MEMSTAT_OFF => 0,
            SYSCTL_EMACPDS_MEMSTATW::SYSCTL_EMACPDS_MEMSTAT_ON => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_EMACPDS_MEMSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_EMACPDS_MEMSTATW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_EMACPDS_MEMSTATW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Array OFF"]
    #[inline(always)]
    pub fn sysctl_emacpds_memstat_off(self) -> &'a mut W {
        self.variant(SYSCTL_EMACPDS_MEMSTATW::SYSCTL_EMACPDS_MEMSTAT_OFF)
    }
    #[doc = "Array On"]
    #[inline(always)]
    pub fn sysctl_emacpds_memstat_on(self) -> &'a mut W {
        self.variant(SYSCTL_EMACPDS_MEMSTATW::SYSCTL_EMACPDS_MEMSTAT_ON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 2);
        self.w.bits |= ((value as u32) & 3) << 2;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Power Domain Status"]
    #[inline(always)]
    pub fn sysctl_emacpds_pwrstat(&self) -> SYSCTL_EMACPDS_PWRSTATR {
        SYSCTL_EMACPDS_PWRSTATR::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Memory Array Power Status"]
    #[inline(always)]
    pub fn sysctl_emacpds_memstat(&self) -> SYSCTL_EMACPDS_MEMSTATR {
        SYSCTL_EMACPDS_MEMSTATR::_from(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Power Domain Status"]
    #[inline(always)]
    pub fn sysctl_emacpds_pwrstat(&mut self) -> _SYSCTL_EMACPDS_PWRSTATW {
        _SYSCTL_EMACPDS_PWRSTATW { w: self }
    }
    #[doc = "Bits 2:3 - Memory Array Power Status"]
    #[inline(always)]
    pub fn sysctl_emacpds_memstat(&mut self) -> _SYSCTL_EMACPDS_MEMSTATW {
        _SYSCTL_EMACPDS_MEMSTATW { w: self }
    }
}

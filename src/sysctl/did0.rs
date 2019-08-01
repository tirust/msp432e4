#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DID0 {
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
#[doc = "Possible values of the field `SYSCTL_DID0_MIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DID0_MINR {
    #[doc = "Initial device, or a major revision update"]
    SYSCTL_DID0_MIN_0,
    #[doc = "First metal layer change"]
    SYSCTL_DID0_MIN_1,
    #[doc = "Second metal layer change"]
    SYSCTL_DID0_MIN_2,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_DID0_MINR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_DID0_MINR::SYSCTL_DID0_MIN_0 => 0,
            SYSCTL_DID0_MINR::SYSCTL_DID0_MIN_1 => 1,
            SYSCTL_DID0_MINR::SYSCTL_DID0_MIN_2 => 2,
            SYSCTL_DID0_MINR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_DID0_MINR {
        match value {
            0 => SYSCTL_DID0_MINR::SYSCTL_DID0_MIN_0,
            1 => SYSCTL_DID0_MINR::SYSCTL_DID0_MIN_1,
            2 => SYSCTL_DID0_MINR::SYSCTL_DID0_MIN_2,
            i => SYSCTL_DID0_MINR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MIN_0`"]
    #[inline(always)]
    pub fn is_sysctl_did0_min_0(&self) -> bool {
        *self == SYSCTL_DID0_MINR::SYSCTL_DID0_MIN_0
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MIN_1`"]
    #[inline(always)]
    pub fn is_sysctl_did0_min_1(&self) -> bool {
        *self == SYSCTL_DID0_MINR::SYSCTL_DID0_MIN_1
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MIN_2`"]
    #[inline(always)]
    pub fn is_sysctl_did0_min_2(&self) -> bool {
        *self == SYSCTL_DID0_MINR::SYSCTL_DID0_MIN_2
    }
}
#[doc = "Values that can be written to the field `SYSCTL_DID0_MIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DID0_MINW {
    #[doc = "Initial device, or a major revision update"]
    SYSCTL_DID0_MIN_0,
    #[doc = "First metal layer change"]
    SYSCTL_DID0_MIN_1,
    #[doc = "Second metal layer change"]
    SYSCTL_DID0_MIN_2,
}
impl SYSCTL_DID0_MINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_DID0_MINW::SYSCTL_DID0_MIN_0 => 0,
            SYSCTL_DID0_MINW::SYSCTL_DID0_MIN_1 => 1,
            SYSCTL_DID0_MINW::SYSCTL_DID0_MIN_2 => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DID0_MINW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DID0_MINW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID0_MINW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Initial device, or a major revision update"]
    #[inline(always)]
    pub fn sysctl_did0_min_0(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MINW::SYSCTL_DID0_MIN_0)
    }
    #[doc = "First metal layer change"]
    #[inline(always)]
    pub fn sysctl_did0_min_1(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MINW::SYSCTL_DID0_MIN_1)
    }
    #[doc = "Second metal layer change"]
    #[inline(always)]
    pub fn sysctl_did0_min_2(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MINW::SYSCTL_DID0_MIN_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 0);
        self.w.bits |= ((value as u32) & 255) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_DID0_MAJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DID0_MAJR {
    #[doc = "Revision A (initial device)"]
    SYSCTL_DID0_MAJ_REVA,
    #[doc = "Revision B (first base layer revision)"]
    SYSCTL_DID0_MAJ_REVB,
    #[doc = "Revision C (second base layer revision)"]
    SYSCTL_DID0_MAJ_REVC,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_DID0_MAJR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_DID0_MAJR::SYSCTL_DID0_MAJ_REVA => 0,
            SYSCTL_DID0_MAJR::SYSCTL_DID0_MAJ_REVB => 1,
            SYSCTL_DID0_MAJR::SYSCTL_DID0_MAJ_REVC => 2,
            SYSCTL_DID0_MAJR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_DID0_MAJR {
        match value {
            0 => SYSCTL_DID0_MAJR::SYSCTL_DID0_MAJ_REVA,
            1 => SYSCTL_DID0_MAJR::SYSCTL_DID0_MAJ_REVB,
            2 => SYSCTL_DID0_MAJR::SYSCTL_DID0_MAJ_REVC,
            i => SYSCTL_DID0_MAJR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MAJ_REVA`"]
    #[inline(always)]
    pub fn is_sysctl_did0_maj_reva(&self) -> bool {
        *self == SYSCTL_DID0_MAJR::SYSCTL_DID0_MAJ_REVA
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MAJ_REVB`"]
    #[inline(always)]
    pub fn is_sysctl_did0_maj_revb(&self) -> bool {
        *self == SYSCTL_DID0_MAJR::SYSCTL_DID0_MAJ_REVB
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MAJ_REVC`"]
    #[inline(always)]
    pub fn is_sysctl_did0_maj_revc(&self) -> bool {
        *self == SYSCTL_DID0_MAJR::SYSCTL_DID0_MAJ_REVC
    }
}
#[doc = "Values that can be written to the field `SYSCTL_DID0_MAJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DID0_MAJW {
    #[doc = "Revision A (initial device)"]
    SYSCTL_DID0_MAJ_REVA,
    #[doc = "Revision B (first base layer revision)"]
    SYSCTL_DID0_MAJ_REVB,
    #[doc = "Revision C (second base layer revision)"]
    SYSCTL_DID0_MAJ_REVC,
}
impl SYSCTL_DID0_MAJW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_DID0_MAJW::SYSCTL_DID0_MAJ_REVA => 0,
            SYSCTL_DID0_MAJW::SYSCTL_DID0_MAJ_REVB => 1,
            SYSCTL_DID0_MAJW::SYSCTL_DID0_MAJ_REVC => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DID0_MAJW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DID0_MAJW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID0_MAJW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Revision A (initial device)"]
    #[inline(always)]
    pub fn sysctl_did0_maj_reva(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MAJW::SYSCTL_DID0_MAJ_REVA)
    }
    #[doc = "Revision B (first base layer revision)"]
    #[inline(always)]
    pub fn sysctl_did0_maj_revb(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MAJW::SYSCTL_DID0_MAJ_REVB)
    }
    #[doc = "Revision C (second base layer revision)"]
    #[inline(always)]
    pub fn sysctl_did0_maj_revc(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MAJW::SYSCTL_DID0_MAJ_REVC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 8);
        self.w.bits |= ((value as u32) & 255) << 8;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_DID0_CLASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DID0_CLASSR {
    #[doc = "MSP432E4 class microcontrollers"]
    SYSCTL_DID0_CLASS_MSP432E4,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_DID0_CLASSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_DID0_CLASSR::SYSCTL_DID0_CLASS_MSP432E4 => 12,
            SYSCTL_DID0_CLASSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_DID0_CLASSR {
        match value {
            12 => SYSCTL_DID0_CLASSR::SYSCTL_DID0_CLASS_MSP432E4,
            i => SYSCTL_DID0_CLASSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_CLASS_MSP432E4`"]
    #[inline(always)]
    pub fn is_sysctl_did0_class_msp432e4(&self) -> bool {
        *self == SYSCTL_DID0_CLASSR::SYSCTL_DID0_CLASS_MSP432E4
    }
}
#[doc = "Values that can be written to the field `SYSCTL_DID0_CLASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DID0_CLASSW {
    #[doc = "MSP432E4 class microcontrollers"]
    SYSCTL_DID0_CLASS_MSP432E4,
}
impl SYSCTL_DID0_CLASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_DID0_CLASSW::SYSCTL_DID0_CLASS_MSP432E4 => 12,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DID0_CLASSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DID0_CLASSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID0_CLASSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "MSP432E4 class microcontrollers"]
    #[inline(always)]
    pub fn sysctl_did0_class_msp432e4(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_CLASSW::SYSCTL_DID0_CLASS_MSP432E4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 16);
        self.w.bits |= ((value as u32) & 255) << 16;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_DID0_VER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DID0_VERR {
    #[doc = "Second version of the DID0 register format."]
    SYSCTL_DID0_VER_1,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_DID0_VERR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_DID0_VERR::SYSCTL_DID0_VER_1 => 1,
            SYSCTL_DID0_VERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_DID0_VERR {
        match value {
            1 => SYSCTL_DID0_VERR::SYSCTL_DID0_VER_1,
            i => SYSCTL_DID0_VERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_VER_1`"]
    #[inline(always)]
    pub fn is_sysctl_did0_ver_1(&self) -> bool {
        *self == SYSCTL_DID0_VERR::SYSCTL_DID0_VER_1
    }
}
#[doc = "Values that can be written to the field `SYSCTL_DID0_VER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DID0_VERW {
    #[doc = "Second version of the DID0 register format."]
    SYSCTL_DID0_VER_1,
}
impl SYSCTL_DID0_VERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_DID0_VERW::SYSCTL_DID0_VER_1 => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DID0_VERW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DID0_VERW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID0_VERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Second version of the DID0 register format."]
    #[inline(always)]
    pub fn sysctl_did0_ver_1(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_VERW::SYSCTL_DID0_VER_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 28);
        self.w.bits |= ((value as u32) & 7) << 28;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Minor Revision"]
    #[inline(always)]
    pub fn sysctl_did0_min(&self) -> SYSCTL_DID0_MINR {
        SYSCTL_DID0_MINR::_from(((self.bits >> 0) & 255) as u8)
    }
    #[doc = "Bits 8:15 - Major Revision"]
    #[inline(always)]
    pub fn sysctl_did0_maj(&self) -> SYSCTL_DID0_MAJR {
        SYSCTL_DID0_MAJR::_from(((self.bits >> 8) & 255) as u8)
    }
    #[doc = "Bits 16:23 - Device Class"]
    #[inline(always)]
    pub fn sysctl_did0_class(&self) -> SYSCTL_DID0_CLASSR {
        SYSCTL_DID0_CLASSR::_from(((self.bits >> 16) & 255) as u8)
    }
    #[doc = "Bits 28:30 - DID0 Version"]
    #[inline(always)]
    pub fn sysctl_did0_ver(&self) -> SYSCTL_DID0_VERR {
        SYSCTL_DID0_VERR::_from(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Minor Revision"]
    #[inline(always)]
    pub fn sysctl_did0_min(&mut self) -> _SYSCTL_DID0_MINW {
        _SYSCTL_DID0_MINW { w: self }
    }
    #[doc = "Bits 8:15 - Major Revision"]
    #[inline(always)]
    pub fn sysctl_did0_maj(&mut self) -> _SYSCTL_DID0_MAJW {
        _SYSCTL_DID0_MAJW { w: self }
    }
    #[doc = "Bits 16:23 - Device Class"]
    #[inline(always)]
    pub fn sysctl_did0_class(&mut self) -> _SYSCTL_DID0_CLASSW {
        _SYSCTL_DID0_CLASSW { w: self }
    }
    #[doc = "Bits 28:30 - DID0 Version"]
    #[inline(always)]
    pub fn sysctl_did0_ver(&mut self) -> _SYSCTL_DID0_VERW {
        _SYSCTL_DID0_VERW { w: self }
    }
}

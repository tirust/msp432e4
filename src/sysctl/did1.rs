#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DID1 {
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
#[doc = "Possible values of the field `SYSCTL_DID1_QUAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DID1_QUALR {
    #[doc = "Engineering Sample (unqualified)"]
    SYSCTL_DID1_QUAL_ES,
    #[doc = "Pilot Production (unqualified)"]
    SYSCTL_DID1_QUAL_PP,
    #[doc = "Fully Qualified"]
    SYSCTL_DID1_QUAL_FQ,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_DID1_QUALR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_DID1_QUALR::SYSCTL_DID1_QUAL_ES => 0,
            SYSCTL_DID1_QUALR::SYSCTL_DID1_QUAL_PP => 1,
            SYSCTL_DID1_QUALR::SYSCTL_DID1_QUAL_FQ => 2,
            SYSCTL_DID1_QUALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_DID1_QUALR {
        match value {
            0 => SYSCTL_DID1_QUALR::SYSCTL_DID1_QUAL_ES,
            1 => SYSCTL_DID1_QUALR::SYSCTL_DID1_QUAL_PP,
            2 => SYSCTL_DID1_QUALR::SYSCTL_DID1_QUAL_FQ,
            i => SYSCTL_DID1_QUALR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_QUAL_ES`"]
    #[inline(always)]
    pub fn is_sysctl_did1_qual_es(&self) -> bool {
        *self == SYSCTL_DID1_QUALR::SYSCTL_DID1_QUAL_ES
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_QUAL_PP`"]
    #[inline(always)]
    pub fn is_sysctl_did1_qual_pp(&self) -> bool {
        *self == SYSCTL_DID1_QUALR::SYSCTL_DID1_QUAL_PP
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_QUAL_FQ`"]
    #[inline(always)]
    pub fn is_sysctl_did1_qual_fq(&self) -> bool {
        *self == SYSCTL_DID1_QUALR::SYSCTL_DID1_QUAL_FQ
    }
}
#[doc = "Values that can be written to the field `SYSCTL_DID1_QUAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DID1_QUALW {
    #[doc = "Engineering Sample (unqualified)"]
    SYSCTL_DID1_QUAL_ES,
    #[doc = "Pilot Production (unqualified)"]
    SYSCTL_DID1_QUAL_PP,
    #[doc = "Fully Qualified"]
    SYSCTL_DID1_QUAL_FQ,
}
impl SYSCTL_DID1_QUALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_DID1_QUALW::SYSCTL_DID1_QUAL_ES => 0,
            SYSCTL_DID1_QUALW::SYSCTL_DID1_QUAL_PP => 1,
            SYSCTL_DID1_QUALW::SYSCTL_DID1_QUAL_FQ => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DID1_QUALW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DID1_QUALW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID1_QUALW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Engineering Sample (unqualified)"]
    #[inline(always)]
    pub fn sysctl_did1_qual_es(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_QUALW::SYSCTL_DID1_QUAL_ES)
    }
    #[doc = "Pilot Production (unqualified)"]
    #[inline(always)]
    pub fn sysctl_did1_qual_pp(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_QUALW::SYSCTL_DID1_QUAL_PP)
    }
    #[doc = "Fully Qualified"]
    #[inline(always)]
    pub fn sysctl_did1_qual_fq(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_QUALW::SYSCTL_DID1_QUAL_FQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_DID1_ROHSR {
    bits: bool,
}
impl SYSCTL_DID1_ROHSR {
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
pub struct _SYSCTL_DID1_ROHSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DID1_ROHSW<'a> {
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
        self.w.bits &= !(1 << 2);
        self.w.bits |= ((value as u32) & 1) << 2;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_DID1_PKG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DID1_PKGR {
    #[doc = "QFP package"]
    SYSCTL_DID1_PKG_QFP,
    #[doc = "BGA package"]
    SYSCTL_DID1_PKG_BGA,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_DID1_PKGR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_DID1_PKGR::SYSCTL_DID1_PKG_QFP => 1,
            SYSCTL_DID1_PKGR::SYSCTL_DID1_PKG_BGA => 2,
            SYSCTL_DID1_PKGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_DID1_PKGR {
        match value {
            1 => SYSCTL_DID1_PKGR::SYSCTL_DID1_PKG_QFP,
            2 => SYSCTL_DID1_PKGR::SYSCTL_DID1_PKG_BGA,
            i => SYSCTL_DID1_PKGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_PKG_QFP`"]
    #[inline(always)]
    pub fn is_sysctl_did1_pkg_qfp(&self) -> bool {
        *self == SYSCTL_DID1_PKGR::SYSCTL_DID1_PKG_QFP
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_PKG_BGA`"]
    #[inline(always)]
    pub fn is_sysctl_did1_pkg_bga(&self) -> bool {
        *self == SYSCTL_DID1_PKGR::SYSCTL_DID1_PKG_BGA
    }
}
#[doc = "Values that can be written to the field `SYSCTL_DID1_PKG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DID1_PKGW {
    #[doc = "QFP package"]
    SYSCTL_DID1_PKG_QFP,
    #[doc = "BGA package"]
    SYSCTL_DID1_PKG_BGA,
}
impl SYSCTL_DID1_PKGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_DID1_PKGW::SYSCTL_DID1_PKG_QFP => 1,
            SYSCTL_DID1_PKGW::SYSCTL_DID1_PKG_BGA => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DID1_PKGW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DID1_PKGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID1_PKGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "QFP package"]
    #[inline(always)]
    pub fn sysctl_did1_pkg_qfp(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_PKGW::SYSCTL_DID1_PKG_QFP)
    }
    #[doc = "BGA package"]
    #[inline(always)]
    pub fn sysctl_did1_pkg_bga(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_PKGW::SYSCTL_DID1_PKG_BGA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 3);
        self.w.bits |= ((value as u32) & 3) << 3;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_DID1_TEMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DID1_TEMPR {
    #[doc = "Commercial temperature range"]
    SYSCTL_DID1_TEMP_C,
    #[doc = "Industrial temperature range"]
    SYSCTL_DID1_TEMP_I,
    #[doc = "Extended temperature range"]
    SYSCTL_DID1_TEMP_E,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_DID1_TEMPR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_DID1_TEMPR::SYSCTL_DID1_TEMP_C => 0,
            SYSCTL_DID1_TEMPR::SYSCTL_DID1_TEMP_I => 1,
            SYSCTL_DID1_TEMPR::SYSCTL_DID1_TEMP_E => 2,
            SYSCTL_DID1_TEMPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_DID1_TEMPR {
        match value {
            0 => SYSCTL_DID1_TEMPR::SYSCTL_DID1_TEMP_C,
            1 => SYSCTL_DID1_TEMPR::SYSCTL_DID1_TEMP_I,
            2 => SYSCTL_DID1_TEMPR::SYSCTL_DID1_TEMP_E,
            i => SYSCTL_DID1_TEMPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_TEMP_C`"]
    #[inline(always)]
    pub fn is_sysctl_did1_temp_c(&self) -> bool {
        *self == SYSCTL_DID1_TEMPR::SYSCTL_DID1_TEMP_C
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_TEMP_I`"]
    #[inline(always)]
    pub fn is_sysctl_did1_temp_i(&self) -> bool {
        *self == SYSCTL_DID1_TEMPR::SYSCTL_DID1_TEMP_I
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_TEMP_E`"]
    #[inline(always)]
    pub fn is_sysctl_did1_temp_e(&self) -> bool {
        *self == SYSCTL_DID1_TEMPR::SYSCTL_DID1_TEMP_E
    }
}
#[doc = "Values that can be written to the field `SYSCTL_DID1_TEMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DID1_TEMPW {
    #[doc = "Commercial temperature range"]
    SYSCTL_DID1_TEMP_C,
    #[doc = "Industrial temperature range"]
    SYSCTL_DID1_TEMP_I,
    #[doc = "Extended temperature range"]
    SYSCTL_DID1_TEMP_E,
}
impl SYSCTL_DID1_TEMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_DID1_TEMPW::SYSCTL_DID1_TEMP_C => 0,
            SYSCTL_DID1_TEMPW::SYSCTL_DID1_TEMP_I => 1,
            SYSCTL_DID1_TEMPW::SYSCTL_DID1_TEMP_E => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DID1_TEMPW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DID1_TEMPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID1_TEMPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Commercial temperature range"]
    #[inline(always)]
    pub fn sysctl_did1_temp_c(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_TEMPW::SYSCTL_DID1_TEMP_C)
    }
    #[doc = "Industrial temperature range"]
    #[inline(always)]
    pub fn sysctl_did1_temp_i(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_TEMPW::SYSCTL_DID1_TEMP_I)
    }
    #[doc = "Extended temperature range"]
    #[inline(always)]
    pub fn sysctl_did1_temp_e(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_TEMPW::SYSCTL_DID1_TEMP_E)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 5);
        self.w.bits |= ((value as u32) & 7) << 5;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_DID1_PINCNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DID1_PINCNTR {
    #[doc = "128-pin TQFP package"]
    SYSCTL_DID1_PINCNT_128,
    #[doc = "212-pin BGA package"]
    SYSCTL_DID1_PINCNT_212,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_DID1_PINCNTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_DID1_PINCNTR::SYSCTL_DID1_PINCNT_128 => 6,
            SYSCTL_DID1_PINCNTR::SYSCTL_DID1_PINCNT_212 => 7,
            SYSCTL_DID1_PINCNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_DID1_PINCNTR {
        match value {
            6 => SYSCTL_DID1_PINCNTR::SYSCTL_DID1_PINCNT_128,
            7 => SYSCTL_DID1_PINCNTR::SYSCTL_DID1_PINCNT_212,
            i => SYSCTL_DID1_PINCNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_PINCNT_128`"]
    #[inline(always)]
    pub fn is_sysctl_did1_pincnt_128(&self) -> bool {
        *self == SYSCTL_DID1_PINCNTR::SYSCTL_DID1_PINCNT_128
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_PINCNT_212`"]
    #[inline(always)]
    pub fn is_sysctl_did1_pincnt_212(&self) -> bool {
        *self == SYSCTL_DID1_PINCNTR::SYSCTL_DID1_PINCNT_212
    }
}
#[doc = "Values that can be written to the field `SYSCTL_DID1_PINCNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DID1_PINCNTW {
    #[doc = "128-pin TQFP package"]
    SYSCTL_DID1_PINCNT_128,
    #[doc = "212-pin BGA package"]
    SYSCTL_DID1_PINCNT_212,
}
impl SYSCTL_DID1_PINCNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_DID1_PINCNTW::SYSCTL_DID1_PINCNT_128 => 6,
            SYSCTL_DID1_PINCNTW::SYSCTL_DID1_PINCNT_212 => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DID1_PINCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DID1_PINCNTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DID1_PINCNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "128-pin TQFP package"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt_128(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_PINCNTW::SYSCTL_DID1_PINCNT_128)
    }
    #[doc = "212-pin BGA package"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt_212(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_PINCNTW::SYSCTL_DID1_PINCNT_212)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 13);
        self.w.bits |= ((value as u32) & 7) << 13;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_DID1_PRTNOR {
    bits: u8,
}
impl SYSCTL_DID1_PRTNOR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DID1_PRTNOW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DID1_PRTNOW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 16);
        self.w.bits |= ((value as u32) & 255) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_DID1_FAMR {
    bits: u8,
}
impl SYSCTL_DID1_FAMR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DID1_FAMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DID1_FAMW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 24);
        self.w.bits |= ((value as u32) & 15) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_DID1_VERR {
    bits: u8,
}
impl SYSCTL_DID1_VERR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DID1_VERW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DID1_VERW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 28);
        self.w.bits |= ((value as u32) & 15) << 28;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Qualification Status"]
    #[inline(always)]
    pub fn sysctl_did1_qual(&self) -> SYSCTL_DID1_QUALR {
        SYSCTL_DID1_QUALR::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bit 2 - RoHS-Compliance"]
    #[inline(always)]
    pub fn sysctl_did1_rohs(&self) -> SYSCTL_DID1_ROHSR {
        let bits = ((self.bits >> 2) & 1) != 0;
        SYSCTL_DID1_ROHSR { bits }
    }
    #[doc = "Bits 3:4 - Package Type"]
    #[inline(always)]
    pub fn sysctl_did1_pkg(&self) -> SYSCTL_DID1_PKGR {
        SYSCTL_DID1_PKGR::_from(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Temperature Range"]
    #[inline(always)]
    pub fn sysctl_did1_temp(&self) -> SYSCTL_DID1_TEMPR {
        SYSCTL_DID1_TEMPR::_from(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Package Pin Count"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt(&self) -> SYSCTL_DID1_PINCNTR {
        SYSCTL_DID1_PINCNTR::_from(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Part Number"]
    #[inline(always)]
    pub fn sysctl_did1_prtno(&self) -> SYSCTL_DID1_PRTNOR {
        let bits = ((self.bits >> 16) & 255) as u8;
        SYSCTL_DID1_PRTNOR { bits }
    }
    #[doc = "Bits 24:27 - Family"]
    #[inline(always)]
    pub fn sysctl_did1_fam(&self) -> SYSCTL_DID1_FAMR {
        let bits = ((self.bits >> 24) & 15) as u8;
        SYSCTL_DID1_FAMR { bits }
    }
    #[doc = "Bits 28:31 - DID1 Version"]
    #[inline(always)]
    pub fn sysctl_did1_ver(&self) -> SYSCTL_DID1_VERR {
        let bits = ((self.bits >> 28) & 15) as u8;
        SYSCTL_DID1_VERR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Qualification Status"]
    #[inline(always)]
    pub fn sysctl_did1_qual(&mut self) -> _SYSCTL_DID1_QUALW {
        _SYSCTL_DID1_QUALW { w: self }
    }
    #[doc = "Bit 2 - RoHS-Compliance"]
    #[inline(always)]
    pub fn sysctl_did1_rohs(&mut self) -> _SYSCTL_DID1_ROHSW {
        _SYSCTL_DID1_ROHSW { w: self }
    }
    #[doc = "Bits 3:4 - Package Type"]
    #[inline(always)]
    pub fn sysctl_did1_pkg(&mut self) -> _SYSCTL_DID1_PKGW {
        _SYSCTL_DID1_PKGW { w: self }
    }
    #[doc = "Bits 5:7 - Temperature Range"]
    #[inline(always)]
    pub fn sysctl_did1_temp(&mut self) -> _SYSCTL_DID1_TEMPW {
        _SYSCTL_DID1_TEMPW { w: self }
    }
    #[doc = "Bits 13:15 - Package Pin Count"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt(&mut self) -> _SYSCTL_DID1_PINCNTW {
        _SYSCTL_DID1_PINCNTW { w: self }
    }
    #[doc = "Bits 16:23 - Part Number"]
    #[inline(always)]
    pub fn sysctl_did1_prtno(&mut self) -> _SYSCTL_DID1_PRTNOW {
        _SYSCTL_DID1_PRTNOW { w: self }
    }
    #[doc = "Bits 24:27 - Family"]
    #[inline(always)]
    pub fn sysctl_did1_fam(&mut self) -> _SYSCTL_DID1_FAMW {
        _SYSCTL_DID1_FAMW { w: self }
    }
    #[doc = "Bits 28:31 - DID1 Version"]
    #[inline(always)]
    pub fn sysctl_did1_ver(&mut self) -> _SYSCTL_DID1_VERW {
        _SYSCTL_DID1_VERW { w: self }
    }
}

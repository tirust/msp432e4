#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR0 {
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
#[doc = "Possible values of the field `SSI_CR0_DSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSI_CR0_DSSR {
    #[doc = "4-bit data"]
    SSI_CR0_DSS_4,
    #[doc = "5-bit data"]
    SSI_CR0_DSS_5,
    #[doc = "6-bit data"]
    SSI_CR0_DSS_6,
    #[doc = "7-bit data"]
    SSI_CR0_DSS_7,
    #[doc = "8-bit data"]
    SSI_CR0_DSS_8,
    #[doc = "9-bit data"]
    SSI_CR0_DSS_9,
    #[doc = "10-bit data"]
    SSI_CR0_DSS_10,
    #[doc = "11-bit data"]
    SSI_CR0_DSS_11,
    #[doc = "12-bit data"]
    SSI_CR0_DSS_12,
    #[doc = "13-bit data"]
    SSI_CR0_DSS_13,
    #[doc = "14-bit data"]
    SSI_CR0_DSS_14,
    #[doc = "15-bit data"]
    SSI_CR0_DSS_15,
    #[doc = "16-bit data"]
    SSI_CR0_DSS_16,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SSI_CR0_DSSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SSI_CR0_DSSR::SSI_CR0_DSS_4 => 3,
            SSI_CR0_DSSR::SSI_CR0_DSS_5 => 4,
            SSI_CR0_DSSR::SSI_CR0_DSS_6 => 5,
            SSI_CR0_DSSR::SSI_CR0_DSS_7 => 6,
            SSI_CR0_DSSR::SSI_CR0_DSS_8 => 7,
            SSI_CR0_DSSR::SSI_CR0_DSS_9 => 8,
            SSI_CR0_DSSR::SSI_CR0_DSS_10 => 9,
            SSI_CR0_DSSR::SSI_CR0_DSS_11 => 10,
            SSI_CR0_DSSR::SSI_CR0_DSS_12 => 11,
            SSI_CR0_DSSR::SSI_CR0_DSS_13 => 12,
            SSI_CR0_DSSR::SSI_CR0_DSS_14 => 13,
            SSI_CR0_DSSR::SSI_CR0_DSS_15 => 14,
            SSI_CR0_DSSR::SSI_CR0_DSS_16 => 15,
            SSI_CR0_DSSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SSI_CR0_DSSR {
        match value {
            3 => SSI_CR0_DSSR::SSI_CR0_DSS_4,
            4 => SSI_CR0_DSSR::SSI_CR0_DSS_5,
            5 => SSI_CR0_DSSR::SSI_CR0_DSS_6,
            6 => SSI_CR0_DSSR::SSI_CR0_DSS_7,
            7 => SSI_CR0_DSSR::SSI_CR0_DSS_8,
            8 => SSI_CR0_DSSR::SSI_CR0_DSS_9,
            9 => SSI_CR0_DSSR::SSI_CR0_DSS_10,
            10 => SSI_CR0_DSSR::SSI_CR0_DSS_11,
            11 => SSI_CR0_DSSR::SSI_CR0_DSS_12,
            12 => SSI_CR0_DSSR::SSI_CR0_DSS_13,
            13 => SSI_CR0_DSSR::SSI_CR0_DSS_14,
            14 => SSI_CR0_DSSR::SSI_CR0_DSS_15,
            15 => SSI_CR0_DSSR::SSI_CR0_DSS_16,
            i => SSI_CR0_DSSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_4`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_4(&self) -> bool {
        *self == SSI_CR0_DSSR::SSI_CR0_DSS_4
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_5`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_5(&self) -> bool {
        *self == SSI_CR0_DSSR::SSI_CR0_DSS_5
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_6`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_6(&self) -> bool {
        *self == SSI_CR0_DSSR::SSI_CR0_DSS_6
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_7`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_7(&self) -> bool {
        *self == SSI_CR0_DSSR::SSI_CR0_DSS_7
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_8`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_8(&self) -> bool {
        *self == SSI_CR0_DSSR::SSI_CR0_DSS_8
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_9`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_9(&self) -> bool {
        *self == SSI_CR0_DSSR::SSI_CR0_DSS_9
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_10`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_10(&self) -> bool {
        *self == SSI_CR0_DSSR::SSI_CR0_DSS_10
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_11`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_11(&self) -> bool {
        *self == SSI_CR0_DSSR::SSI_CR0_DSS_11
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_12`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_12(&self) -> bool {
        *self == SSI_CR0_DSSR::SSI_CR0_DSS_12
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_13`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_13(&self) -> bool {
        *self == SSI_CR0_DSSR::SSI_CR0_DSS_13
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_14`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_14(&self) -> bool {
        *self == SSI_CR0_DSSR::SSI_CR0_DSS_14
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_15`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_15(&self) -> bool {
        *self == SSI_CR0_DSSR::SSI_CR0_DSS_15
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_DSS_16`"]
    #[inline(always)]
    pub fn is_ssi_cr0_dss_16(&self) -> bool {
        *self == SSI_CR0_DSSR::SSI_CR0_DSS_16
    }
}
#[doc = "Values that can be written to the field `SSI_CR0_DSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSI_CR0_DSSW {
    #[doc = "4-bit data"]
    SSI_CR0_DSS_4,
    #[doc = "5-bit data"]
    SSI_CR0_DSS_5,
    #[doc = "6-bit data"]
    SSI_CR0_DSS_6,
    #[doc = "7-bit data"]
    SSI_CR0_DSS_7,
    #[doc = "8-bit data"]
    SSI_CR0_DSS_8,
    #[doc = "9-bit data"]
    SSI_CR0_DSS_9,
    #[doc = "10-bit data"]
    SSI_CR0_DSS_10,
    #[doc = "11-bit data"]
    SSI_CR0_DSS_11,
    #[doc = "12-bit data"]
    SSI_CR0_DSS_12,
    #[doc = "13-bit data"]
    SSI_CR0_DSS_13,
    #[doc = "14-bit data"]
    SSI_CR0_DSS_14,
    #[doc = "15-bit data"]
    SSI_CR0_DSS_15,
    #[doc = "16-bit data"]
    SSI_CR0_DSS_16,
}
impl SSI_CR0_DSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSI_CR0_DSSW::SSI_CR0_DSS_4 => 3,
            SSI_CR0_DSSW::SSI_CR0_DSS_5 => 4,
            SSI_CR0_DSSW::SSI_CR0_DSS_6 => 5,
            SSI_CR0_DSSW::SSI_CR0_DSS_7 => 6,
            SSI_CR0_DSSW::SSI_CR0_DSS_8 => 7,
            SSI_CR0_DSSW::SSI_CR0_DSS_9 => 8,
            SSI_CR0_DSSW::SSI_CR0_DSS_10 => 9,
            SSI_CR0_DSSW::SSI_CR0_DSS_11 => 10,
            SSI_CR0_DSSW::SSI_CR0_DSS_12 => 11,
            SSI_CR0_DSSW::SSI_CR0_DSS_13 => 12,
            SSI_CR0_DSSW::SSI_CR0_DSS_14 => 13,
            SSI_CR0_DSSW::SSI_CR0_DSS_15 => 14,
            SSI_CR0_DSSW::SSI_CR0_DSS_16 => 15,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SSI_CR0_DSSW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_CR0_DSSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSI_CR0_DSSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "4-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_4(self) -> &'a mut W {
        self.variant(SSI_CR0_DSSW::SSI_CR0_DSS_4)
    }
    #[doc = "5-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_5(self) -> &'a mut W {
        self.variant(SSI_CR0_DSSW::SSI_CR0_DSS_5)
    }
    #[doc = "6-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_6(self) -> &'a mut W {
        self.variant(SSI_CR0_DSSW::SSI_CR0_DSS_6)
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_7(self) -> &'a mut W {
        self.variant(SSI_CR0_DSSW::SSI_CR0_DSS_7)
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_8(self) -> &'a mut W {
        self.variant(SSI_CR0_DSSW::SSI_CR0_DSS_8)
    }
    #[doc = "9-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_9(self) -> &'a mut W {
        self.variant(SSI_CR0_DSSW::SSI_CR0_DSS_9)
    }
    #[doc = "10-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_10(self) -> &'a mut W {
        self.variant(SSI_CR0_DSSW::SSI_CR0_DSS_10)
    }
    #[doc = "11-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_11(self) -> &'a mut W {
        self.variant(SSI_CR0_DSSW::SSI_CR0_DSS_11)
    }
    #[doc = "12-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_12(self) -> &'a mut W {
        self.variant(SSI_CR0_DSSW::SSI_CR0_DSS_12)
    }
    #[doc = "13-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_13(self) -> &'a mut W {
        self.variant(SSI_CR0_DSSW::SSI_CR0_DSS_13)
    }
    #[doc = "14-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_14(self) -> &'a mut W {
        self.variant(SSI_CR0_DSSW::SSI_CR0_DSS_14)
    }
    #[doc = "15-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_15(self) -> &'a mut W {
        self.variant(SSI_CR0_DSSW::SSI_CR0_DSS_15)
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn ssi_cr0_dss_16(self) -> &'a mut W {
        self.variant(SSI_CR0_DSSW::SSI_CR0_DSS_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `SSI_CR0_FRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSI_CR0_FRFR {
    #[doc = "Freescale SPI Frame Format"]
    SSI_CR0_FRF_MOTO,
    #[doc = "Synchronous Serial Frame Format"]
    SSI_CR0_FRF_TI,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SSI_CR0_FRFR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SSI_CR0_FRFR::SSI_CR0_FRF_MOTO => 0,
            SSI_CR0_FRFR::SSI_CR0_FRF_TI => 1,
            SSI_CR0_FRFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SSI_CR0_FRFR {
        match value {
            0 => SSI_CR0_FRFR::SSI_CR0_FRF_MOTO,
            1 => SSI_CR0_FRFR::SSI_CR0_FRF_TI,
            i => SSI_CR0_FRFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_FRF_MOTO`"]
    #[inline(always)]
    pub fn is_ssi_cr0_frf_moto(&self) -> bool {
        *self == SSI_CR0_FRFR::SSI_CR0_FRF_MOTO
    }
    #[doc = "Checks if the value of the field is `SSI_CR0_FRF_TI`"]
    #[inline(always)]
    pub fn is_ssi_cr0_frf_ti(&self) -> bool {
        *self == SSI_CR0_FRFR::SSI_CR0_FRF_TI
    }
}
#[doc = "Values that can be written to the field `SSI_CR0_FRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSI_CR0_FRFW {
    #[doc = "Freescale SPI Frame Format"]
    SSI_CR0_FRF_MOTO,
    #[doc = "Synchronous Serial Frame Format"]
    SSI_CR0_FRF_TI,
}
impl SSI_CR0_FRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSI_CR0_FRFW::SSI_CR0_FRF_MOTO => 0,
            SSI_CR0_FRFW::SSI_CR0_FRF_TI => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SSI_CR0_FRFW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_CR0_FRFW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSI_CR0_FRFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Freescale SPI Frame Format"]
    #[inline(always)]
    pub fn ssi_cr0_frf_moto(self) -> &'a mut W {
        self.variant(SSI_CR0_FRFW::SSI_CR0_FRF_MOTO)
    }
    #[doc = "Synchronous Serial Frame Format"]
    #[inline(always)]
    pub fn ssi_cr0_frf_ti(self) -> &'a mut W {
        self.variant(SSI_CR0_FRFW::SSI_CR0_FRF_TI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SSI_CR0_SPOR {
    bits: bool,
}
impl SSI_CR0_SPOR {
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
pub struct _SSI_CR0_SPOW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_CR0_SPOW<'a> {
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
        self.w.bits &= !(1 << 6);
        self.w.bits |= ((value as u32) & 1) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SSI_CR0_SPHR {
    bits: bool,
}
impl SSI_CR0_SPHR {
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
pub struct _SSI_CR0_SPHW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_CR0_SPHW<'a> {
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
        self.w.bits &= !(1 << 7);
        self.w.bits |= ((value as u32) & 1) << 7;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SSI_CR0_SCRR {
    bits: u8,
}
impl SSI_CR0_SCRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SSI_CR0_SCRW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_CR0_SCRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 8);
        self.w.bits |= ((value as u32) & 255) << 8;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - SSI Data Size Select"]
    #[inline(always)]
    pub fn ssi_cr0_dss(&self) -> SSI_CR0_DSSR {
        SSI_CR0_DSSR::_from(((self.bits >> 0) & 15) as u8)
    }
    #[doc = "Bits 4:5 - SSI Frame Format Select"]
    #[inline(always)]
    pub fn ssi_cr0_frf(&self) -> SSI_CR0_FRFR {
        SSI_CR0_FRFR::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - SSI Serial Clock Polarity"]
    #[inline(always)]
    pub fn ssi_cr0_spo(&self) -> SSI_CR0_SPOR {
        let bits = ((self.bits >> 6) & 1) != 0;
        SSI_CR0_SPOR { bits }
    }
    #[doc = "Bit 7 - SSI Serial Clock Phase"]
    #[inline(always)]
    pub fn ssi_cr0_sph(&self) -> SSI_CR0_SPHR {
        let bits = ((self.bits >> 7) & 1) != 0;
        SSI_CR0_SPHR { bits }
    }
    #[doc = "Bits 8:15 - SSI Serial Clock Rate"]
    #[inline(always)]
    pub fn ssi_cr0_scr(&self) -> SSI_CR0_SCRR {
        let bits = ((self.bits >> 8) & 255) as u8;
        SSI_CR0_SCRR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - SSI Data Size Select"]
    #[inline(always)]
    pub fn ssi_cr0_dss(&mut self) -> _SSI_CR0_DSSW {
        _SSI_CR0_DSSW { w: self }
    }
    #[doc = "Bits 4:5 - SSI Frame Format Select"]
    #[inline(always)]
    pub fn ssi_cr0_frf(&mut self) -> _SSI_CR0_FRFW {
        _SSI_CR0_FRFW { w: self }
    }
    #[doc = "Bit 6 - SSI Serial Clock Polarity"]
    #[inline(always)]
    pub fn ssi_cr0_spo(&mut self) -> _SSI_CR0_SPOW {
        _SSI_CR0_SPOW { w: self }
    }
    #[doc = "Bit 7 - SSI Serial Clock Phase"]
    #[inline(always)]
    pub fn ssi_cr0_sph(&mut self) -> _SSI_CR0_SPHW {
        _SSI_CR0_SPHW { w: self }
    }
    #[doc = "Bits 8:15 - SSI Serial Clock Rate"]
    #[inline(always)]
    pub fn ssi_cr0_scr(&mut self) -> _SSI_CR0_SCRW {
        _SSI_CR0_SCRW { w: self }
    }
}

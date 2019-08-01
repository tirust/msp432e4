#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPCFG {
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
#[doc = "Possible values of the field `EPI_GPCFG_DSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_GPCFG_DSIZER {
    #[doc = "8 Bits Wide (EPI0S0 to EPI0S7)"]
    EPI_GPCFG_DSIZE_4BIT,
    #[doc = "16 Bits Wide (EPI0S0 to EPI0S15)"]
    EPI_GPCFG_DSIZE_16BIT,
    #[doc = "24 Bits Wide (EPI0S0 to EPI0S23)"]
    EPI_GPCFG_DSIZE_24BIT,
    #[doc = "32 Bits Wide (EPI0S0 to EPI0S31)"]
    EPI_GPCFG_DSIZE_32BIT,
}
impl EPI_GPCFG_DSIZER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_GPCFG_DSIZER::EPI_GPCFG_DSIZE_4BIT => 0,
            EPI_GPCFG_DSIZER::EPI_GPCFG_DSIZE_16BIT => 1,
            EPI_GPCFG_DSIZER::EPI_GPCFG_DSIZE_24BIT => 2,
            EPI_GPCFG_DSIZER::EPI_GPCFG_DSIZE_32BIT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_GPCFG_DSIZER {
        match value {
            0 => EPI_GPCFG_DSIZER::EPI_GPCFG_DSIZE_4BIT,
            1 => EPI_GPCFG_DSIZER::EPI_GPCFG_DSIZE_16BIT,
            2 => EPI_GPCFG_DSIZER::EPI_GPCFG_DSIZE_24BIT,
            3 => EPI_GPCFG_DSIZER::EPI_GPCFG_DSIZE_32BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_GPCFG_DSIZE_4BIT`"]
    #[inline(always)]
    pub fn is_epi_gpcfg_dsize_4bit(&self) -> bool {
        *self == EPI_GPCFG_DSIZER::EPI_GPCFG_DSIZE_4BIT
    }
    #[doc = "Checks if the value of the field is `EPI_GPCFG_DSIZE_16BIT`"]
    #[inline(always)]
    pub fn is_epi_gpcfg_dsize_16bit(&self) -> bool {
        *self == EPI_GPCFG_DSIZER::EPI_GPCFG_DSIZE_16BIT
    }
    #[doc = "Checks if the value of the field is `EPI_GPCFG_DSIZE_24BIT`"]
    #[inline(always)]
    pub fn is_epi_gpcfg_dsize_24bit(&self) -> bool {
        *self == EPI_GPCFG_DSIZER::EPI_GPCFG_DSIZE_24BIT
    }
    #[doc = "Checks if the value of the field is `EPI_GPCFG_DSIZE_32BIT`"]
    #[inline(always)]
    pub fn is_epi_gpcfg_dsize_32bit(&self) -> bool {
        *self == EPI_GPCFG_DSIZER::EPI_GPCFG_DSIZE_32BIT
    }
}
#[doc = "Values that can be written to the field `EPI_GPCFG_DSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_GPCFG_DSIZEW {
    #[doc = "8 Bits Wide (EPI0S0 to EPI0S7)"]
    EPI_GPCFG_DSIZE_4BIT,
    #[doc = "16 Bits Wide (EPI0S0 to EPI0S15)"]
    EPI_GPCFG_DSIZE_16BIT,
    #[doc = "24 Bits Wide (EPI0S0 to EPI0S23)"]
    EPI_GPCFG_DSIZE_24BIT,
    #[doc = "32 Bits Wide (EPI0S0 to EPI0S31)"]
    EPI_GPCFG_DSIZE_32BIT,
}
impl EPI_GPCFG_DSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_GPCFG_DSIZEW::EPI_GPCFG_DSIZE_4BIT => 0,
            EPI_GPCFG_DSIZEW::EPI_GPCFG_DSIZE_16BIT => 1,
            EPI_GPCFG_DSIZEW::EPI_GPCFG_DSIZE_24BIT => 2,
            EPI_GPCFG_DSIZEW::EPI_GPCFG_DSIZE_32BIT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_GPCFG_DSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_GPCFG_DSIZEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_GPCFG_DSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "8 Bits Wide (EPI0S0 to EPI0S7)"]
    #[inline(always)]
    pub fn epi_gpcfg_dsize_4bit(self) -> &'a mut W {
        self.variant(EPI_GPCFG_DSIZEW::EPI_GPCFG_DSIZE_4BIT)
    }
    #[doc = "16 Bits Wide (EPI0S0 to EPI0S15)"]
    #[inline(always)]
    pub fn epi_gpcfg_dsize_16bit(self) -> &'a mut W {
        self.variant(EPI_GPCFG_DSIZEW::EPI_GPCFG_DSIZE_16BIT)
    }
    #[doc = "24 Bits Wide (EPI0S0 to EPI0S23)"]
    #[inline(always)]
    pub fn epi_gpcfg_dsize_24bit(self) -> &'a mut W {
        self.variant(EPI_GPCFG_DSIZEW::EPI_GPCFG_DSIZE_24BIT)
    }
    #[doc = "32 Bits Wide (EPI0S0 to EPI0S31)"]
    #[inline(always)]
    pub fn epi_gpcfg_dsize_32bit(self) -> &'a mut W {
        self.variant(EPI_GPCFG_DSIZEW::EPI_GPCFG_DSIZE_32BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `EPI_GPCFG_ASIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_GPCFG_ASIZER {
    #[doc = "No address"]
    EPI_GPCFG_ASIZE_NONE,
    #[doc = "Up to 4 bits wide"]
    EPI_GPCFG_ASIZE_4BIT,
    #[doc = "Up to 12 bits wide. This size cannot be used with 24-bit data"]
    EPI_GPCFG_ASIZE_12BIT,
    #[doc = "Up to 20 bits wide. This size cannot be used with data sizes other than 8"]
    EPI_GPCFG_ASIZE_20BIT,
}
impl EPI_GPCFG_ASIZER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_GPCFG_ASIZER::EPI_GPCFG_ASIZE_NONE => 0,
            EPI_GPCFG_ASIZER::EPI_GPCFG_ASIZE_4BIT => 1,
            EPI_GPCFG_ASIZER::EPI_GPCFG_ASIZE_12BIT => 2,
            EPI_GPCFG_ASIZER::EPI_GPCFG_ASIZE_20BIT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_GPCFG_ASIZER {
        match value {
            0 => EPI_GPCFG_ASIZER::EPI_GPCFG_ASIZE_NONE,
            1 => EPI_GPCFG_ASIZER::EPI_GPCFG_ASIZE_4BIT,
            2 => EPI_GPCFG_ASIZER::EPI_GPCFG_ASIZE_12BIT,
            3 => EPI_GPCFG_ASIZER::EPI_GPCFG_ASIZE_20BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_GPCFG_ASIZE_NONE`"]
    #[inline(always)]
    pub fn is_epi_gpcfg_asize_none(&self) -> bool {
        *self == EPI_GPCFG_ASIZER::EPI_GPCFG_ASIZE_NONE
    }
    #[doc = "Checks if the value of the field is `EPI_GPCFG_ASIZE_4BIT`"]
    #[inline(always)]
    pub fn is_epi_gpcfg_asize_4bit(&self) -> bool {
        *self == EPI_GPCFG_ASIZER::EPI_GPCFG_ASIZE_4BIT
    }
    #[doc = "Checks if the value of the field is `EPI_GPCFG_ASIZE_12BIT`"]
    #[inline(always)]
    pub fn is_epi_gpcfg_asize_12bit(&self) -> bool {
        *self == EPI_GPCFG_ASIZER::EPI_GPCFG_ASIZE_12BIT
    }
    #[doc = "Checks if the value of the field is `EPI_GPCFG_ASIZE_20BIT`"]
    #[inline(always)]
    pub fn is_epi_gpcfg_asize_20bit(&self) -> bool {
        *self == EPI_GPCFG_ASIZER::EPI_GPCFG_ASIZE_20BIT
    }
}
#[doc = "Values that can be written to the field `EPI_GPCFG_ASIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_GPCFG_ASIZEW {
    #[doc = "No address"]
    EPI_GPCFG_ASIZE_NONE,
    #[doc = "Up to 4 bits wide"]
    EPI_GPCFG_ASIZE_4BIT,
    #[doc = "Up to 12 bits wide. This size cannot be used with 24-bit data"]
    EPI_GPCFG_ASIZE_12BIT,
    #[doc = "Up to 20 bits wide. This size cannot be used with data sizes other than 8"]
    EPI_GPCFG_ASIZE_20BIT,
}
impl EPI_GPCFG_ASIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_GPCFG_ASIZEW::EPI_GPCFG_ASIZE_NONE => 0,
            EPI_GPCFG_ASIZEW::EPI_GPCFG_ASIZE_4BIT => 1,
            EPI_GPCFG_ASIZEW::EPI_GPCFG_ASIZE_12BIT => 2,
            EPI_GPCFG_ASIZEW::EPI_GPCFG_ASIZE_20BIT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_GPCFG_ASIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_GPCFG_ASIZEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_GPCFG_ASIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No address"]
    #[inline(always)]
    pub fn epi_gpcfg_asize_none(self) -> &'a mut W {
        self.variant(EPI_GPCFG_ASIZEW::EPI_GPCFG_ASIZE_NONE)
    }
    #[doc = "Up to 4 bits wide"]
    #[inline(always)]
    pub fn epi_gpcfg_asize_4bit(self) -> &'a mut W {
        self.variant(EPI_GPCFG_ASIZEW::EPI_GPCFG_ASIZE_4BIT)
    }
    #[doc = "Up to 12 bits wide. This size cannot be used with 24-bit data"]
    #[inline(always)]
    pub fn epi_gpcfg_asize_12bit(self) -> &'a mut W {
        self.variant(EPI_GPCFG_ASIZEW::EPI_GPCFG_ASIZE_12BIT)
    }
    #[doc = "Up to 20 bits wide. This size cannot be used with data sizes other than 8"]
    #[inline(always)]
    pub fn epi_gpcfg_asize_20bit(self) -> &'a mut W {
        self.variant(EPI_GPCFG_ASIZEW::EPI_GPCFG_ASIZE_20BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_GPCFG_WR2CYCR {
    bits: bool,
}
impl EPI_GPCFG_WR2CYCR {
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
pub struct _EPI_GPCFG_WR2CYCW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_GPCFG_WR2CYCW<'a> {
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
        self.w.bits &= !(1 << 19);
        self.w.bits |= ((value as u32) & 1) << 19;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_GPCFG_FRMCNTR {
    bits: u8,
}
impl EPI_GPCFG_FRMCNTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EPI_GPCFG_FRMCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_GPCFG_FRMCNTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 22);
        self.w.bits |= ((value as u32) & 15) << 22;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_GPCFG_FRM50R {
    bits: bool,
}
impl EPI_GPCFG_FRM50R {
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
pub struct _EPI_GPCFG_FRM50W<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_GPCFG_FRM50W<'a> {
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
        self.w.bits &= !(1 << 26);
        self.w.bits |= ((value as u32) & 1) << 26;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_GPCFG_CLKGATER {
    bits: bool,
}
impl EPI_GPCFG_CLKGATER {
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
pub struct _EPI_GPCFG_CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_GPCFG_CLKGATEW<'a> {
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
        self.w.bits &= !(1 << 30);
        self.w.bits |= ((value as u32) & 1) << 30;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_GPCFG_CLKPINR {
    bits: bool,
}
impl EPI_GPCFG_CLKPINR {
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
pub struct _EPI_GPCFG_CLKPINW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_GPCFG_CLKPINW<'a> {
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
    #[doc = "Bits 0:1 - Size of Data Bus"]
    #[inline(always)]
    pub fn epi_gpcfg_dsize(&self) -> EPI_GPCFG_DSIZER {
        EPI_GPCFG_DSIZER::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Address Bus Size"]
    #[inline(always)]
    pub fn epi_gpcfg_asize(&self) -> EPI_GPCFG_ASIZER {
        EPI_GPCFG_ASIZER::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 19 - 2-Cycle Writes"]
    #[inline(always)]
    pub fn epi_gpcfg_wr2cyc(&self) -> EPI_GPCFG_WR2CYCR {
        let bits = ((self.bits >> 19) & 1) != 0;
        EPI_GPCFG_WR2CYCR { bits }
    }
    #[doc = "Bits 22:25 - Frame Count"]
    #[inline(always)]
    pub fn epi_gpcfg_frmcnt(&self) -> EPI_GPCFG_FRMCNTR {
        let bits = ((self.bits >> 22) & 15) as u8;
        EPI_GPCFG_FRMCNTR { bits }
    }
    #[doc = "Bit 26 - 50/50 Frame"]
    #[inline(always)]
    pub fn epi_gpcfg_frm50(&self) -> EPI_GPCFG_FRM50R {
        let bits = ((self.bits >> 26) & 1) != 0;
        EPI_GPCFG_FRM50R { bits }
    }
    #[doc = "Bit 30 - Clock Gated"]
    #[inline(always)]
    pub fn epi_gpcfg_clkgate(&self) -> EPI_GPCFG_CLKGATER {
        let bits = ((self.bits >> 30) & 1) != 0;
        EPI_GPCFG_CLKGATER { bits }
    }
    #[doc = "Bit 31 - Clock Pin"]
    #[inline(always)]
    pub fn epi_gpcfg_clkpin(&self) -> EPI_GPCFG_CLKPINR {
        let bits = ((self.bits >> 31) & 1) != 0;
        EPI_GPCFG_CLKPINR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Size of Data Bus"]
    #[inline(always)]
    pub fn epi_gpcfg_dsize(&mut self) -> _EPI_GPCFG_DSIZEW {
        _EPI_GPCFG_DSIZEW { w: self }
    }
    #[doc = "Bits 4:5 - Address Bus Size"]
    #[inline(always)]
    pub fn epi_gpcfg_asize(&mut self) -> _EPI_GPCFG_ASIZEW {
        _EPI_GPCFG_ASIZEW { w: self }
    }
    #[doc = "Bit 19 - 2-Cycle Writes"]
    #[inline(always)]
    pub fn epi_gpcfg_wr2cyc(&mut self) -> _EPI_GPCFG_WR2CYCW {
        _EPI_GPCFG_WR2CYCW { w: self }
    }
    #[doc = "Bits 22:25 - Frame Count"]
    #[inline(always)]
    pub fn epi_gpcfg_frmcnt(&mut self) -> _EPI_GPCFG_FRMCNTW {
        _EPI_GPCFG_FRMCNTW { w: self }
    }
    #[doc = "Bit 26 - 50/50 Frame"]
    #[inline(always)]
    pub fn epi_gpcfg_frm50(&mut self) -> _EPI_GPCFG_FRM50W {
        _EPI_GPCFG_FRM50W { w: self }
    }
    #[doc = "Bit 30 - Clock Gated"]
    #[inline(always)]
    pub fn epi_gpcfg_clkgate(&mut self) -> _EPI_GPCFG_CLKGATEW {
        _EPI_GPCFG_CLKGATEW { w: self }
    }
    #[doc = "Bit 31 - Clock Pin"]
    #[inline(always)]
    pub fn epi_gpcfg_clkpin(&mut self) -> _EPI_GPCFG_CLKPINW {
        _EPI_GPCFG_CLKPINW { w: self }
    }
}

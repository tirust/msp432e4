#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HB16CFG3 {
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
#[doc = "Possible values of the field `EPI_HB16CFG3_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG3_MODER {
    #[doc = "ADMUX - AD\\[15:0\\]"]
    EPI_HB16CFG3_MODE_ADMUX,
    #[doc = "ADNONMUX - D\\[15:0\\]"]
    EPI_HB16CFG3_MODE_AD,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EPI_HB16CFG3_MODER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG3_MODER::EPI_HB16CFG3_MODE_ADMUX => 0,
            EPI_HB16CFG3_MODER::EPI_HB16CFG3_MODE_AD => 1,
            EPI_HB16CFG3_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_HB16CFG3_MODER {
        match value {
            0 => EPI_HB16CFG3_MODER::EPI_HB16CFG3_MODE_ADMUX,
            1 => EPI_HB16CFG3_MODER::EPI_HB16CFG3_MODE_AD,
            i => EPI_HB16CFG3_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG3_MODE_ADMUX`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg3_mode_admux(&self) -> bool {
        *self == EPI_HB16CFG3_MODER::EPI_HB16CFG3_MODE_ADMUX
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG3_MODE_AD`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg3_mode_ad(&self) -> bool {
        *self == EPI_HB16CFG3_MODER::EPI_HB16CFG3_MODE_AD
    }
}
#[doc = "Values that can be written to the field `EPI_HB16CFG3_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG3_MODEW {
    #[doc = "ADMUX - AD\\[15:0\\]"]
    EPI_HB16CFG3_MODE_ADMUX,
    #[doc = "ADNONMUX - D\\[15:0\\]"]
    EPI_HB16CFG3_MODE_AD,
}
impl EPI_HB16CFG3_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG3_MODEW::EPI_HB16CFG3_MODE_ADMUX => 0,
            EPI_HB16CFG3_MODEW::EPI_HB16CFG3_MODE_AD => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB16CFG3_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG3_MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_HB16CFG3_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ADMUX - AD\\[15:0\\]"]
    #[inline(always)]
    pub fn epi_hb16cfg3_mode_admux(self) -> &'a mut W {
        self.variant(EPI_HB16CFG3_MODEW::EPI_HB16CFG3_MODE_ADMUX)
    }
    #[doc = "ADNONMUX - D\\[15:0\\]"]
    #[inline(always)]
    pub fn epi_hb16cfg3_mode_ad(self) -> &'a mut W {
        self.variant(EPI_HB16CFG3_MODEW::EPI_HB16CFG3_MODE_AD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `EPI_HB16CFG3_RDWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG3_RDWSR {
    #[doc = "Active RDn is 2 EPI clocks"]
    EPI_HB16CFG3_RDWS_2,
    #[doc = "Active RDn is 4 EPI clocks"]
    EPI_HB16CFG3_RDWS_4,
    #[doc = "Active RDn is 6 EPI clocks"]
    EPI_HB16CFG3_RDWS_6,
    #[doc = "Active RDn is 8 EPI clocks"]
    EPI_HB16CFG3_RDWS_8,
}
impl EPI_HB16CFG3_RDWSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG3_RDWSR::EPI_HB16CFG3_RDWS_2 => 0,
            EPI_HB16CFG3_RDWSR::EPI_HB16CFG3_RDWS_4 => 1,
            EPI_HB16CFG3_RDWSR::EPI_HB16CFG3_RDWS_6 => 2,
            EPI_HB16CFG3_RDWSR::EPI_HB16CFG3_RDWS_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_HB16CFG3_RDWSR {
        match value {
            0 => EPI_HB16CFG3_RDWSR::EPI_HB16CFG3_RDWS_2,
            1 => EPI_HB16CFG3_RDWSR::EPI_HB16CFG3_RDWS_4,
            2 => EPI_HB16CFG3_RDWSR::EPI_HB16CFG3_RDWS_6,
            3 => EPI_HB16CFG3_RDWSR::EPI_HB16CFG3_RDWS_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG3_RDWS_2`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg3_rdws_2(&self) -> bool {
        *self == EPI_HB16CFG3_RDWSR::EPI_HB16CFG3_RDWS_2
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG3_RDWS_4`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg3_rdws_4(&self) -> bool {
        *self == EPI_HB16CFG3_RDWSR::EPI_HB16CFG3_RDWS_4
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG3_RDWS_6`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg3_rdws_6(&self) -> bool {
        *self == EPI_HB16CFG3_RDWSR::EPI_HB16CFG3_RDWS_6
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG3_RDWS_8`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg3_rdws_8(&self) -> bool {
        *self == EPI_HB16CFG3_RDWSR::EPI_HB16CFG3_RDWS_8
    }
}
#[doc = "Values that can be written to the field `EPI_HB16CFG3_RDWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG3_RDWSW {
    #[doc = "Active RDn is 2 EPI clocks"]
    EPI_HB16CFG3_RDWS_2,
    #[doc = "Active RDn is 4 EPI clocks"]
    EPI_HB16CFG3_RDWS_4,
    #[doc = "Active RDn is 6 EPI clocks"]
    EPI_HB16CFG3_RDWS_6,
    #[doc = "Active RDn is 8 EPI clocks"]
    EPI_HB16CFG3_RDWS_8,
}
impl EPI_HB16CFG3_RDWSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG3_RDWSW::EPI_HB16CFG3_RDWS_2 => 0,
            EPI_HB16CFG3_RDWSW::EPI_HB16CFG3_RDWS_4 => 1,
            EPI_HB16CFG3_RDWSW::EPI_HB16CFG3_RDWS_6 => 2,
            EPI_HB16CFG3_RDWSW::EPI_HB16CFG3_RDWS_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB16CFG3_RDWSW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG3_RDWSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_HB16CFG3_RDWSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Active RDn is 2 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg3_rdws_2(self) -> &'a mut W {
        self.variant(EPI_HB16CFG3_RDWSW::EPI_HB16CFG3_RDWS_2)
    }
    #[doc = "Active RDn is 4 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg3_rdws_4(self) -> &'a mut W {
        self.variant(EPI_HB16CFG3_RDWSW::EPI_HB16CFG3_RDWS_4)
    }
    #[doc = "Active RDn is 6 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg3_rdws_6(self) -> &'a mut W {
        self.variant(EPI_HB16CFG3_RDWSW::EPI_HB16CFG3_RDWS_6)
    }
    #[doc = "Active RDn is 8 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg3_rdws_8(self) -> &'a mut W {
        self.variant(EPI_HB16CFG3_RDWSW::EPI_HB16CFG3_RDWS_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = "Possible values of the field `EPI_HB16CFG3_WRWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG3_WRWSR {
    #[doc = "Active WRn is 2 EPI clocks"]
    EPI_HB16CFG3_WRWS_2,
    #[doc = "Active WRn is 4 EPI clocks"]
    EPI_HB16CFG3_WRWS_4,
    #[doc = "Active WRn is 6 EPI clocks"]
    EPI_HB16CFG3_WRWS_6,
    #[doc = "Active WRn is 8 EPI clocks"]
    EPI_HB16CFG3_WRWS_8,
}
impl EPI_HB16CFG3_WRWSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG3_WRWSR::EPI_HB16CFG3_WRWS_2 => 0,
            EPI_HB16CFG3_WRWSR::EPI_HB16CFG3_WRWS_4 => 1,
            EPI_HB16CFG3_WRWSR::EPI_HB16CFG3_WRWS_6 => 2,
            EPI_HB16CFG3_WRWSR::EPI_HB16CFG3_WRWS_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_HB16CFG3_WRWSR {
        match value {
            0 => EPI_HB16CFG3_WRWSR::EPI_HB16CFG3_WRWS_2,
            1 => EPI_HB16CFG3_WRWSR::EPI_HB16CFG3_WRWS_4,
            2 => EPI_HB16CFG3_WRWSR::EPI_HB16CFG3_WRWS_6,
            3 => EPI_HB16CFG3_WRWSR::EPI_HB16CFG3_WRWS_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG3_WRWS_2`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg3_wrws_2(&self) -> bool {
        *self == EPI_HB16CFG3_WRWSR::EPI_HB16CFG3_WRWS_2
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG3_WRWS_4`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg3_wrws_4(&self) -> bool {
        *self == EPI_HB16CFG3_WRWSR::EPI_HB16CFG3_WRWS_4
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG3_WRWS_6`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg3_wrws_6(&self) -> bool {
        *self == EPI_HB16CFG3_WRWSR::EPI_HB16CFG3_WRWS_6
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG3_WRWS_8`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg3_wrws_8(&self) -> bool {
        *self == EPI_HB16CFG3_WRWSR::EPI_HB16CFG3_WRWS_8
    }
}
#[doc = "Values that can be written to the field `EPI_HB16CFG3_WRWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG3_WRWSW {
    #[doc = "Active WRn is 2 EPI clocks"]
    EPI_HB16CFG3_WRWS_2,
    #[doc = "Active WRn is 4 EPI clocks"]
    EPI_HB16CFG3_WRWS_4,
    #[doc = "Active WRn is 6 EPI clocks"]
    EPI_HB16CFG3_WRWS_6,
    #[doc = "Active WRn is 8 EPI clocks"]
    EPI_HB16CFG3_WRWS_8,
}
impl EPI_HB16CFG3_WRWSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG3_WRWSW::EPI_HB16CFG3_WRWS_2 => 0,
            EPI_HB16CFG3_WRWSW::EPI_HB16CFG3_WRWS_4 => 1,
            EPI_HB16CFG3_WRWSW::EPI_HB16CFG3_WRWS_6 => 2,
            EPI_HB16CFG3_WRWSW::EPI_HB16CFG3_WRWS_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB16CFG3_WRWSW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG3_WRWSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_HB16CFG3_WRWSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Active WRn is 2 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg3_wrws_2(self) -> &'a mut W {
        self.variant(EPI_HB16CFG3_WRWSW::EPI_HB16CFG3_WRWS_2)
    }
    #[doc = "Active WRn is 4 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg3_wrws_4(self) -> &'a mut W {
        self.variant(EPI_HB16CFG3_WRWSW::EPI_HB16CFG3_WRWS_4)
    }
    #[doc = "Active WRn is 6 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg3_wrws_6(self) -> &'a mut W {
        self.variant(EPI_HB16CFG3_WRWSW::EPI_HB16CFG3_WRWS_6)
    }
    #[doc = "Active WRn is 8 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg3_wrws_8(self) -> &'a mut W {
        self.variant(EPI_HB16CFG3_WRWSW::EPI_HB16CFG3_WRWS_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 6);
        self.w.bits |= ((value as u32) & 3) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB16CFG3_BURSTR {
    bits: bool,
}
impl EPI_HB16CFG3_BURSTR {
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
pub struct _EPI_HB16CFG3_BURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG3_BURSTW<'a> {
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
        self.w.bits &= !(1 << 16);
        self.w.bits |= ((value as u32) & 1) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB16CFG3_RDCRER {
    bits: bool,
}
impl EPI_HB16CFG3_RDCRER {
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
pub struct _EPI_HB16CFG3_RDCREW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG3_RDCREW<'a> {
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
        self.w.bits &= !(1 << 17);
        self.w.bits |= ((value as u32) & 1) << 17;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB16CFG3_WRCRER {
    bits: bool,
}
impl EPI_HB16CFG3_WRCRER {
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
pub struct _EPI_HB16CFG3_WRCREW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG3_WRCREW<'a> {
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
        self.w.bits &= !(1 << 18);
        self.w.bits |= ((value as u32) & 1) << 18;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB16CFG3_ALEHIGHR {
    bits: bool,
}
impl EPI_HB16CFG3_ALEHIGHR {
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
pub struct _EPI_HB16CFG3_ALEHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG3_ALEHIGHW<'a> {
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
pub struct EPI_HB16CFG3_RDHIGHR {
    bits: bool,
}
impl EPI_HB16CFG3_RDHIGHR {
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
pub struct _EPI_HB16CFG3_RDHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG3_RDHIGHW<'a> {
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
        self.w.bits &= !(1 << 20);
        self.w.bits |= ((value as u32) & 1) << 20;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB16CFG3_WRHIGHR {
    bits: bool,
}
impl EPI_HB16CFG3_WRHIGHR {
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
pub struct _EPI_HB16CFG3_WRHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG3_WRHIGHW<'a> {
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
        self.w.bits &= !(1 << 21);
        self.w.bits |= ((value as u32) & 1) << 21;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - CS2n Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn epi_hb16cfg3_mode(&self) -> EPI_HB16CFG3_MODER {
        EPI_HB16CFG3_MODER::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CS2n Read Wait States"]
    #[inline(always)]
    pub fn epi_hb16cfg3_rdws(&self) -> EPI_HB16CFG3_RDWSR {
        EPI_HB16CFG3_RDWSR::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CS2n Write Wait States"]
    #[inline(always)]
    pub fn epi_hb16cfg3_wrws(&self) -> EPI_HB16CFG3_WRWSR {
        EPI_HB16CFG3_WRWSR::_from(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 16 - CS2n Burst Mode"]
    #[inline(always)]
    pub fn epi_hb16cfg3_burst(&self) -> EPI_HB16CFG3_BURSTR {
        let bits = ((self.bits >> 16) & 1) != 0;
        EPI_HB16CFG3_BURSTR { bits }
    }
    #[doc = "Bit 17 - CS2n PSRAM Configuration Register Read"]
    #[inline(always)]
    pub fn epi_hb16cfg3_rdcre(&self) -> EPI_HB16CFG3_RDCRER {
        let bits = ((self.bits >> 17) & 1) != 0;
        EPI_HB16CFG3_RDCRER { bits }
    }
    #[doc = "Bit 18 - CS2n PSRAM Configuration Register Write"]
    #[inline(always)]
    pub fn epi_hb16cfg3_wrcre(&self) -> EPI_HB16CFG3_WRCRER {
        let bits = ((self.bits >> 18) & 1) != 0;
        EPI_HB16CFG3_WRCRER { bits }
    }
    #[doc = "Bit 19 - CS2n ALE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg3_alehigh(&self) -> EPI_HB16CFG3_ALEHIGHR {
        let bits = ((self.bits >> 19) & 1) != 0;
        EPI_HB16CFG3_ALEHIGHR { bits }
    }
    #[doc = "Bit 20 - CS2n READ Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg3_rdhigh(&self) -> EPI_HB16CFG3_RDHIGHR {
        let bits = ((self.bits >> 20) & 1) != 0;
        EPI_HB16CFG3_RDHIGHR { bits }
    }
    #[doc = "Bit 21 - CS2n WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg3_wrhigh(&self) -> EPI_HB16CFG3_WRHIGHR {
        let bits = ((self.bits >> 21) & 1) != 0;
        EPI_HB16CFG3_WRHIGHR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - CS2n Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn epi_hb16cfg3_mode(&mut self) -> _EPI_HB16CFG3_MODEW {
        _EPI_HB16CFG3_MODEW { w: self }
    }
    #[doc = "Bits 4:5 - CS2n Read Wait States"]
    #[inline(always)]
    pub fn epi_hb16cfg3_rdws(&mut self) -> _EPI_HB16CFG3_RDWSW {
        _EPI_HB16CFG3_RDWSW { w: self }
    }
    #[doc = "Bits 6:7 - CS2n Write Wait States"]
    #[inline(always)]
    pub fn epi_hb16cfg3_wrws(&mut self) -> _EPI_HB16CFG3_WRWSW {
        _EPI_HB16CFG3_WRWSW { w: self }
    }
    #[doc = "Bit 16 - CS2n Burst Mode"]
    #[inline(always)]
    pub fn epi_hb16cfg3_burst(&mut self) -> _EPI_HB16CFG3_BURSTW {
        _EPI_HB16CFG3_BURSTW { w: self }
    }
    #[doc = "Bit 17 - CS2n PSRAM Configuration Register Read"]
    #[inline(always)]
    pub fn epi_hb16cfg3_rdcre(&mut self) -> _EPI_HB16CFG3_RDCREW {
        _EPI_HB16CFG3_RDCREW { w: self }
    }
    #[doc = "Bit 18 - CS2n PSRAM Configuration Register Write"]
    #[inline(always)]
    pub fn epi_hb16cfg3_wrcre(&mut self) -> _EPI_HB16CFG3_WRCREW {
        _EPI_HB16CFG3_WRCREW { w: self }
    }
    #[doc = "Bit 19 - CS2n ALE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg3_alehigh(&mut self) -> _EPI_HB16CFG3_ALEHIGHW {
        _EPI_HB16CFG3_ALEHIGHW { w: self }
    }
    #[doc = "Bit 20 - CS2n READ Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg3_rdhigh(&mut self) -> _EPI_HB16CFG3_RDHIGHW {
        _EPI_HB16CFG3_RDHIGHW { w: self }
    }
    #[doc = "Bit 21 - CS2n WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg3_wrhigh(&mut self) -> _EPI_HB16CFG3_WRHIGHW {
        _EPI_HB16CFG3_WRHIGHW { w: self }
    }
}

#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HB16CFG2 {
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
#[doc = "Possible values of the field `EPI_HB16CFG2_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG2_MODER {
    #[doc = "ADMUX - AD\\[15:0\\]"]
    EPI_HB16CFG2_MODE_ADMUX,
    #[doc = "ADNONMUX - D\\[15:0\\]"]
    EPI_HB16CFG2_MODE_AD,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EPI_HB16CFG2_MODER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG2_MODER::EPI_HB16CFG2_MODE_ADMUX => 0,
            EPI_HB16CFG2_MODER::EPI_HB16CFG2_MODE_AD => 1,
            EPI_HB16CFG2_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_HB16CFG2_MODER {
        match value {
            0 => EPI_HB16CFG2_MODER::EPI_HB16CFG2_MODE_ADMUX,
            1 => EPI_HB16CFG2_MODER::EPI_HB16CFG2_MODE_AD,
            i => EPI_HB16CFG2_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG2_MODE_ADMUX`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg2_mode_admux(&self) -> bool {
        *self == EPI_HB16CFG2_MODER::EPI_HB16CFG2_MODE_ADMUX
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG2_MODE_AD`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg2_mode_ad(&self) -> bool {
        *self == EPI_HB16CFG2_MODER::EPI_HB16CFG2_MODE_AD
    }
}
#[doc = "Values that can be written to the field `EPI_HB16CFG2_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG2_MODEW {
    #[doc = "ADMUX - AD\\[15:0\\]"]
    EPI_HB16CFG2_MODE_ADMUX,
    #[doc = "ADNONMUX - D\\[15:0\\]"]
    EPI_HB16CFG2_MODE_AD,
}
impl EPI_HB16CFG2_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG2_MODEW::EPI_HB16CFG2_MODE_ADMUX => 0,
            EPI_HB16CFG2_MODEW::EPI_HB16CFG2_MODE_AD => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB16CFG2_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG2_MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_HB16CFG2_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ADMUX - AD\\[15:0\\]"]
    #[inline(always)]
    pub fn epi_hb16cfg2_mode_admux(self) -> &'a mut W {
        self.variant(EPI_HB16CFG2_MODEW::EPI_HB16CFG2_MODE_ADMUX)
    }
    #[doc = "ADNONMUX - D\\[15:0\\]"]
    #[inline(always)]
    pub fn epi_hb16cfg2_mode_ad(self) -> &'a mut W {
        self.variant(EPI_HB16CFG2_MODEW::EPI_HB16CFG2_MODE_AD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `EPI_HB16CFG2_RDWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG2_RDWSR {
    #[doc = "Active RDn is 2 EPI clocks"]
    EPI_HB16CFG2_RDWS_2,
    #[doc = "Active RDn is 4 EPI clocks"]
    EPI_HB16CFG2_RDWS_4,
    #[doc = "Active RDn is 6 EPI clocks"]
    EPI_HB16CFG2_RDWS_6,
    #[doc = "Active RDn is 8 EPI clocks"]
    EPI_HB16CFG2_RDWS_8,
}
impl EPI_HB16CFG2_RDWSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG2_RDWSR::EPI_HB16CFG2_RDWS_2 => 0,
            EPI_HB16CFG2_RDWSR::EPI_HB16CFG2_RDWS_4 => 1,
            EPI_HB16CFG2_RDWSR::EPI_HB16CFG2_RDWS_6 => 2,
            EPI_HB16CFG2_RDWSR::EPI_HB16CFG2_RDWS_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_HB16CFG2_RDWSR {
        match value {
            0 => EPI_HB16CFG2_RDWSR::EPI_HB16CFG2_RDWS_2,
            1 => EPI_HB16CFG2_RDWSR::EPI_HB16CFG2_RDWS_4,
            2 => EPI_HB16CFG2_RDWSR::EPI_HB16CFG2_RDWS_6,
            3 => EPI_HB16CFG2_RDWSR::EPI_HB16CFG2_RDWS_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG2_RDWS_2`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg2_rdws_2(&self) -> bool {
        *self == EPI_HB16CFG2_RDWSR::EPI_HB16CFG2_RDWS_2
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG2_RDWS_4`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg2_rdws_4(&self) -> bool {
        *self == EPI_HB16CFG2_RDWSR::EPI_HB16CFG2_RDWS_4
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG2_RDWS_6`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg2_rdws_6(&self) -> bool {
        *self == EPI_HB16CFG2_RDWSR::EPI_HB16CFG2_RDWS_6
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG2_RDWS_8`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg2_rdws_8(&self) -> bool {
        *self == EPI_HB16CFG2_RDWSR::EPI_HB16CFG2_RDWS_8
    }
}
#[doc = "Values that can be written to the field `EPI_HB16CFG2_RDWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG2_RDWSW {
    #[doc = "Active RDn is 2 EPI clocks"]
    EPI_HB16CFG2_RDWS_2,
    #[doc = "Active RDn is 4 EPI clocks"]
    EPI_HB16CFG2_RDWS_4,
    #[doc = "Active RDn is 6 EPI clocks"]
    EPI_HB16CFG2_RDWS_6,
    #[doc = "Active RDn is 8 EPI clocks"]
    EPI_HB16CFG2_RDWS_8,
}
impl EPI_HB16CFG2_RDWSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG2_RDWSW::EPI_HB16CFG2_RDWS_2 => 0,
            EPI_HB16CFG2_RDWSW::EPI_HB16CFG2_RDWS_4 => 1,
            EPI_HB16CFG2_RDWSW::EPI_HB16CFG2_RDWS_6 => 2,
            EPI_HB16CFG2_RDWSW::EPI_HB16CFG2_RDWS_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB16CFG2_RDWSW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG2_RDWSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_HB16CFG2_RDWSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Active RDn is 2 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg2_rdws_2(self) -> &'a mut W {
        self.variant(EPI_HB16CFG2_RDWSW::EPI_HB16CFG2_RDWS_2)
    }
    #[doc = "Active RDn is 4 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg2_rdws_4(self) -> &'a mut W {
        self.variant(EPI_HB16CFG2_RDWSW::EPI_HB16CFG2_RDWS_4)
    }
    #[doc = "Active RDn is 6 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg2_rdws_6(self) -> &'a mut W {
        self.variant(EPI_HB16CFG2_RDWSW::EPI_HB16CFG2_RDWS_6)
    }
    #[doc = "Active RDn is 8 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg2_rdws_8(self) -> &'a mut W {
        self.variant(EPI_HB16CFG2_RDWSW::EPI_HB16CFG2_RDWS_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = "Possible values of the field `EPI_HB16CFG2_WRWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG2_WRWSR {
    #[doc = "Active WRn is 2 EPI clocks"]
    EPI_HB16CFG2_WRWS_2,
    #[doc = "Active WRn is 4 EPI clocks"]
    EPI_HB16CFG2_WRWS_4,
    #[doc = "Active WRn is 6 EPI clocks"]
    EPI_HB16CFG2_WRWS_6,
    #[doc = "Active WRn is 8 EPI clocks"]
    EPI_HB16CFG2_WRWS_8,
}
impl EPI_HB16CFG2_WRWSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG2_WRWSR::EPI_HB16CFG2_WRWS_2 => 0,
            EPI_HB16CFG2_WRWSR::EPI_HB16CFG2_WRWS_4 => 1,
            EPI_HB16CFG2_WRWSR::EPI_HB16CFG2_WRWS_6 => 2,
            EPI_HB16CFG2_WRWSR::EPI_HB16CFG2_WRWS_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_HB16CFG2_WRWSR {
        match value {
            0 => EPI_HB16CFG2_WRWSR::EPI_HB16CFG2_WRWS_2,
            1 => EPI_HB16CFG2_WRWSR::EPI_HB16CFG2_WRWS_4,
            2 => EPI_HB16CFG2_WRWSR::EPI_HB16CFG2_WRWS_6,
            3 => EPI_HB16CFG2_WRWSR::EPI_HB16CFG2_WRWS_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG2_WRWS_2`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg2_wrws_2(&self) -> bool {
        *self == EPI_HB16CFG2_WRWSR::EPI_HB16CFG2_WRWS_2
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG2_WRWS_4`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg2_wrws_4(&self) -> bool {
        *self == EPI_HB16CFG2_WRWSR::EPI_HB16CFG2_WRWS_4
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG2_WRWS_6`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg2_wrws_6(&self) -> bool {
        *self == EPI_HB16CFG2_WRWSR::EPI_HB16CFG2_WRWS_6
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG2_WRWS_8`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg2_wrws_8(&self) -> bool {
        *self == EPI_HB16CFG2_WRWSR::EPI_HB16CFG2_WRWS_8
    }
}
#[doc = "Values that can be written to the field `EPI_HB16CFG2_WRWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG2_WRWSW {
    #[doc = "Active WRn is 2 EPI clocks"]
    EPI_HB16CFG2_WRWS_2,
    #[doc = "Active WRn is 4 EPI clocks"]
    EPI_HB16CFG2_WRWS_4,
    #[doc = "Active WRn is 6 EPI clocks"]
    EPI_HB16CFG2_WRWS_6,
    #[doc = "Active WRn is 8 EPI clocks"]
    EPI_HB16CFG2_WRWS_8,
}
impl EPI_HB16CFG2_WRWSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG2_WRWSW::EPI_HB16CFG2_WRWS_2 => 0,
            EPI_HB16CFG2_WRWSW::EPI_HB16CFG2_WRWS_4 => 1,
            EPI_HB16CFG2_WRWSW::EPI_HB16CFG2_WRWS_6 => 2,
            EPI_HB16CFG2_WRWSW::EPI_HB16CFG2_WRWS_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB16CFG2_WRWSW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG2_WRWSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_HB16CFG2_WRWSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Active WRn is 2 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg2_wrws_2(self) -> &'a mut W {
        self.variant(EPI_HB16CFG2_WRWSW::EPI_HB16CFG2_WRWS_2)
    }
    #[doc = "Active WRn is 4 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg2_wrws_4(self) -> &'a mut W {
        self.variant(EPI_HB16CFG2_WRWSW::EPI_HB16CFG2_WRWS_4)
    }
    #[doc = "Active WRn is 6 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg2_wrws_6(self) -> &'a mut W {
        self.variant(EPI_HB16CFG2_WRWSW::EPI_HB16CFG2_WRWS_6)
    }
    #[doc = "Active WRn is 8 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg2_wrws_8(self) -> &'a mut W {
        self.variant(EPI_HB16CFG2_WRWSW::EPI_HB16CFG2_WRWS_8)
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
pub struct EPI_HB16CFG2_BURSTR {
    bits: bool,
}
impl EPI_HB16CFG2_BURSTR {
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
pub struct _EPI_HB16CFG2_BURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG2_BURSTW<'a> {
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
pub struct EPI_HB16CFG2_RDCRER {
    bits: bool,
}
impl EPI_HB16CFG2_RDCRER {
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
pub struct _EPI_HB16CFG2_RDCREW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG2_RDCREW<'a> {
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
pub struct EPI_HB16CFG2_WRCRER {
    bits: bool,
}
impl EPI_HB16CFG2_WRCRER {
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
pub struct _EPI_HB16CFG2_WRCREW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG2_WRCREW<'a> {
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
pub struct EPI_HB16CFG2_ALEHIGHR {
    bits: bool,
}
impl EPI_HB16CFG2_ALEHIGHR {
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
pub struct _EPI_HB16CFG2_ALEHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG2_ALEHIGHW<'a> {
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
pub struct EPI_HB16CFG2_RDHIGHR {
    bits: bool,
}
impl EPI_HB16CFG2_RDHIGHR {
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
pub struct _EPI_HB16CFG2_RDHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG2_RDHIGHW<'a> {
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
pub struct EPI_HB16CFG2_WRHIGHR {
    bits: bool,
}
impl EPI_HB16CFG2_WRHIGHR {
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
pub struct _EPI_HB16CFG2_WRHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG2_WRHIGHW<'a> {
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
#[doc = "Possible values of the field `EPI_HB16CFG2_CSCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG2_CSCFGR {
    #[doc = "ALE Configuration"]
    EPI_HB16CFG2_CSCFG_ALE,
    #[doc = "CSn Configuration"]
    EPI_HB16CFG2_CSCFG_CS,
    #[doc = "Dual CSn Configuration"]
    EPI_HB16CFG2_CSCFG_DCS,
    #[doc = "ALE with Dual CSn Configuration"]
    EPI_HB16CFG2_CSCFG_ADCS,
}
impl EPI_HB16CFG2_CSCFGR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG2_CSCFGR::EPI_HB16CFG2_CSCFG_ALE => 0,
            EPI_HB16CFG2_CSCFGR::EPI_HB16CFG2_CSCFG_CS => 1,
            EPI_HB16CFG2_CSCFGR::EPI_HB16CFG2_CSCFG_DCS => 2,
            EPI_HB16CFG2_CSCFGR::EPI_HB16CFG2_CSCFG_ADCS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_HB16CFG2_CSCFGR {
        match value {
            0 => EPI_HB16CFG2_CSCFGR::EPI_HB16CFG2_CSCFG_ALE,
            1 => EPI_HB16CFG2_CSCFGR::EPI_HB16CFG2_CSCFG_CS,
            2 => EPI_HB16CFG2_CSCFGR::EPI_HB16CFG2_CSCFG_DCS,
            3 => EPI_HB16CFG2_CSCFGR::EPI_HB16CFG2_CSCFG_ADCS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG2_CSCFG_ALE`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg2_cscfg_ale(&self) -> bool {
        *self == EPI_HB16CFG2_CSCFGR::EPI_HB16CFG2_CSCFG_ALE
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG2_CSCFG_CS`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg2_cscfg_cs(&self) -> bool {
        *self == EPI_HB16CFG2_CSCFGR::EPI_HB16CFG2_CSCFG_CS
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG2_CSCFG_DCS`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg2_cscfg_dcs(&self) -> bool {
        *self == EPI_HB16CFG2_CSCFGR::EPI_HB16CFG2_CSCFG_DCS
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG2_CSCFG_ADCS`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg2_cscfg_adcs(&self) -> bool {
        *self == EPI_HB16CFG2_CSCFGR::EPI_HB16CFG2_CSCFG_ADCS
    }
}
#[doc = "Values that can be written to the field `EPI_HB16CFG2_CSCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG2_CSCFGW {
    #[doc = "ALE Configuration"]
    EPI_HB16CFG2_CSCFG_ALE,
    #[doc = "CSn Configuration"]
    EPI_HB16CFG2_CSCFG_CS,
    #[doc = "Dual CSn Configuration"]
    EPI_HB16CFG2_CSCFG_DCS,
    #[doc = "ALE with Dual CSn Configuration"]
    EPI_HB16CFG2_CSCFG_ADCS,
}
impl EPI_HB16CFG2_CSCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG2_CSCFGW::EPI_HB16CFG2_CSCFG_ALE => 0,
            EPI_HB16CFG2_CSCFGW::EPI_HB16CFG2_CSCFG_CS => 1,
            EPI_HB16CFG2_CSCFGW::EPI_HB16CFG2_CSCFG_DCS => 2,
            EPI_HB16CFG2_CSCFGW::EPI_HB16CFG2_CSCFG_ADCS => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB16CFG2_CSCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG2_CSCFGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_HB16CFG2_CSCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ALE Configuration"]
    #[inline(always)]
    pub fn epi_hb16cfg2_cscfg_ale(self) -> &'a mut W {
        self.variant(EPI_HB16CFG2_CSCFGW::EPI_HB16CFG2_CSCFG_ALE)
    }
    #[doc = "CSn Configuration"]
    #[inline(always)]
    pub fn epi_hb16cfg2_cscfg_cs(self) -> &'a mut W {
        self.variant(EPI_HB16CFG2_CSCFGW::EPI_HB16CFG2_CSCFG_CS)
    }
    #[doc = "Dual CSn Configuration"]
    #[inline(always)]
    pub fn epi_hb16cfg2_cscfg_dcs(self) -> &'a mut W {
        self.variant(EPI_HB16CFG2_CSCFGW::EPI_HB16CFG2_CSCFG_DCS)
    }
    #[doc = "ALE with Dual CSn Configuration"]
    #[inline(always)]
    pub fn epi_hb16cfg2_cscfg_adcs(self) -> &'a mut W {
        self.variant(EPI_HB16CFG2_CSCFGW::EPI_HB16CFG2_CSCFG_ADCS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 24);
        self.w.bits |= ((value as u32) & 3) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB16CFG2_CSBAUDR {
    bits: bool,
}
impl EPI_HB16CFG2_CSBAUDR {
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
pub struct _EPI_HB16CFG2_CSBAUDW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG2_CSBAUDW<'a> {
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
pub struct EPI_HB16CFG2_CSCFGEXTR {
    bits: bool,
}
impl EPI_HB16CFG2_CSCFGEXTR {
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
pub struct _EPI_HB16CFG2_CSCFGEXTW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG2_CSCFGEXTW<'a> {
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
        self.w.bits &= !(1 << 27);
        self.w.bits |= ((value as u32) & 1) << 27;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - CS1n Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn epi_hb16cfg2_mode(&self) -> EPI_HB16CFG2_MODER {
        EPI_HB16CFG2_MODER::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CS1n Read Wait States"]
    #[inline(always)]
    pub fn epi_hb16cfg2_rdws(&self) -> EPI_HB16CFG2_RDWSR {
        EPI_HB16CFG2_RDWSR::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CS1n Write Wait States"]
    #[inline(always)]
    pub fn epi_hb16cfg2_wrws(&self) -> EPI_HB16CFG2_WRWSR {
        EPI_HB16CFG2_WRWSR::_from(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 16 - CS1n Burst Mode"]
    #[inline(always)]
    pub fn epi_hb16cfg2_burst(&self) -> EPI_HB16CFG2_BURSTR {
        let bits = ((self.bits >> 16) & 1) != 0;
        EPI_HB16CFG2_BURSTR { bits }
    }
    #[doc = "Bit 17 - CS1n PSRAM Configuration Register Read"]
    #[inline(always)]
    pub fn epi_hb16cfg2_rdcre(&self) -> EPI_HB16CFG2_RDCRER {
        let bits = ((self.bits >> 17) & 1) != 0;
        EPI_HB16CFG2_RDCRER { bits }
    }
    #[doc = "Bit 18 - CS1n PSRAM Configuration Register Write"]
    #[inline(always)]
    pub fn epi_hb16cfg2_wrcre(&self) -> EPI_HB16CFG2_WRCRER {
        let bits = ((self.bits >> 18) & 1) != 0;
        EPI_HB16CFG2_WRCRER { bits }
    }
    #[doc = "Bit 19 - CS1n ALE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg2_alehigh(&self) -> EPI_HB16CFG2_ALEHIGHR {
        let bits = ((self.bits >> 19) & 1) != 0;
        EPI_HB16CFG2_ALEHIGHR { bits }
    }
    #[doc = "Bit 20 - CS1n READ Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg2_rdhigh(&self) -> EPI_HB16CFG2_RDHIGHR {
        let bits = ((self.bits >> 20) & 1) != 0;
        EPI_HB16CFG2_RDHIGHR { bits }
    }
    #[doc = "Bit 21 - CS1n WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg2_wrhigh(&self) -> EPI_HB16CFG2_WRHIGHR {
        let bits = ((self.bits >> 21) & 1) != 0;
        EPI_HB16CFG2_WRHIGHR { bits }
    }
    #[doc = "Bits 24:25 - Chip Select Configuration"]
    #[inline(always)]
    pub fn epi_hb16cfg2_cscfg(&self) -> EPI_HB16CFG2_CSCFGR {
        EPI_HB16CFG2_CSCFGR::_from(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Chip Select Baud Rate and Multiple Sub-Mode Configuration enable"]
    #[inline(always)]
    pub fn epi_hb16cfg2_csbaud(&self) -> EPI_HB16CFG2_CSBAUDR {
        let bits = ((self.bits >> 26) & 1) != 0;
        EPI_HB16CFG2_CSBAUDR { bits }
    }
    #[doc = "Bit 27 - Chip Select Extended Configuration"]
    #[inline(always)]
    pub fn epi_hb16cfg2_cscfgext(&self) -> EPI_HB16CFG2_CSCFGEXTR {
        let bits = ((self.bits >> 27) & 1) != 0;
        EPI_HB16CFG2_CSCFGEXTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - CS1n Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn epi_hb16cfg2_mode(&mut self) -> _EPI_HB16CFG2_MODEW {
        _EPI_HB16CFG2_MODEW { w: self }
    }
    #[doc = "Bits 4:5 - CS1n Read Wait States"]
    #[inline(always)]
    pub fn epi_hb16cfg2_rdws(&mut self) -> _EPI_HB16CFG2_RDWSW {
        _EPI_HB16CFG2_RDWSW { w: self }
    }
    #[doc = "Bits 6:7 - CS1n Write Wait States"]
    #[inline(always)]
    pub fn epi_hb16cfg2_wrws(&mut self) -> _EPI_HB16CFG2_WRWSW {
        _EPI_HB16CFG2_WRWSW { w: self }
    }
    #[doc = "Bit 16 - CS1n Burst Mode"]
    #[inline(always)]
    pub fn epi_hb16cfg2_burst(&mut self) -> _EPI_HB16CFG2_BURSTW {
        _EPI_HB16CFG2_BURSTW { w: self }
    }
    #[doc = "Bit 17 - CS1n PSRAM Configuration Register Read"]
    #[inline(always)]
    pub fn epi_hb16cfg2_rdcre(&mut self) -> _EPI_HB16CFG2_RDCREW {
        _EPI_HB16CFG2_RDCREW { w: self }
    }
    #[doc = "Bit 18 - CS1n PSRAM Configuration Register Write"]
    #[inline(always)]
    pub fn epi_hb16cfg2_wrcre(&mut self) -> _EPI_HB16CFG2_WRCREW {
        _EPI_HB16CFG2_WRCREW { w: self }
    }
    #[doc = "Bit 19 - CS1n ALE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg2_alehigh(&mut self) -> _EPI_HB16CFG2_ALEHIGHW {
        _EPI_HB16CFG2_ALEHIGHW { w: self }
    }
    #[doc = "Bit 20 - CS1n READ Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg2_rdhigh(&mut self) -> _EPI_HB16CFG2_RDHIGHW {
        _EPI_HB16CFG2_RDHIGHW { w: self }
    }
    #[doc = "Bit 21 - CS1n WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg2_wrhigh(&mut self) -> _EPI_HB16CFG2_WRHIGHW {
        _EPI_HB16CFG2_WRHIGHW { w: self }
    }
    #[doc = "Bits 24:25 - Chip Select Configuration"]
    #[inline(always)]
    pub fn epi_hb16cfg2_cscfg(&mut self) -> _EPI_HB16CFG2_CSCFGW {
        _EPI_HB16CFG2_CSCFGW { w: self }
    }
    #[doc = "Bit 26 - Chip Select Baud Rate and Multiple Sub-Mode Configuration enable"]
    #[inline(always)]
    pub fn epi_hb16cfg2_csbaud(&mut self) -> _EPI_HB16CFG2_CSBAUDW {
        _EPI_HB16CFG2_CSBAUDW { w: self }
    }
    #[doc = "Bit 27 - Chip Select Extended Configuration"]
    #[inline(always)]
    pub fn epi_hb16cfg2_cscfgext(&mut self) -> _EPI_HB16CFG2_CSCFGEXTW {
        _EPI_HB16CFG2_CSCFGEXTW { w: self }
    }
}

#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HB16CFG {
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
#[doc = "Possible values of the field `EPI_HB16CFG_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG_MODER {
    #[doc = "ADMUX - AD\\[15:0\\]"]
    EPI_HB16CFG_MODE_ADMUX,
    #[doc = "ADNONMUX - D\\[15:0\\]"]
    EPI_HB16CFG_MODE_ADNMUX,
    #[doc = "Continuous Read - D\\[15:0\\]"]
    EPI_HB16CFG_MODE_SRAM,
    #[doc = "XFIFO - D\\[15:0\\]"]
    EPI_HB16CFG_MODE_XFIFO,
}
impl EPI_HB16CFG_MODER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG_MODER::EPI_HB16CFG_MODE_ADMUX => 0,
            EPI_HB16CFG_MODER::EPI_HB16CFG_MODE_ADNMUX => 1,
            EPI_HB16CFG_MODER::EPI_HB16CFG_MODE_SRAM => 2,
            EPI_HB16CFG_MODER::EPI_HB16CFG_MODE_XFIFO => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_HB16CFG_MODER {
        match value {
            0 => EPI_HB16CFG_MODER::EPI_HB16CFG_MODE_ADMUX,
            1 => EPI_HB16CFG_MODER::EPI_HB16CFG_MODE_ADNMUX,
            2 => EPI_HB16CFG_MODER::EPI_HB16CFG_MODE_SRAM,
            3 => EPI_HB16CFG_MODER::EPI_HB16CFG_MODE_XFIFO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG_MODE_ADMUX`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg_mode_admux(&self) -> bool {
        *self == EPI_HB16CFG_MODER::EPI_HB16CFG_MODE_ADMUX
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG_MODE_ADNMUX`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg_mode_adnmux(&self) -> bool {
        *self == EPI_HB16CFG_MODER::EPI_HB16CFG_MODE_ADNMUX
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG_MODE_SRAM`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg_mode_sram(&self) -> bool {
        *self == EPI_HB16CFG_MODER::EPI_HB16CFG_MODE_SRAM
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG_MODE_XFIFO`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg_mode_xfifo(&self) -> bool {
        *self == EPI_HB16CFG_MODER::EPI_HB16CFG_MODE_XFIFO
    }
}
#[doc = "Values that can be written to the field `EPI_HB16CFG_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG_MODEW {
    #[doc = "ADMUX - AD\\[15:0\\]"]
    EPI_HB16CFG_MODE_ADMUX,
    #[doc = "ADNONMUX - D\\[15:0\\]"]
    EPI_HB16CFG_MODE_ADNMUX,
    #[doc = "Continuous Read - D\\[15:0\\]"]
    EPI_HB16CFG_MODE_SRAM,
    #[doc = "XFIFO - D\\[15:0\\]"]
    EPI_HB16CFG_MODE_XFIFO,
}
impl EPI_HB16CFG_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG_MODEW::EPI_HB16CFG_MODE_ADMUX => 0,
            EPI_HB16CFG_MODEW::EPI_HB16CFG_MODE_ADNMUX => 1,
            EPI_HB16CFG_MODEW::EPI_HB16CFG_MODE_SRAM => 2,
            EPI_HB16CFG_MODEW::EPI_HB16CFG_MODE_XFIFO => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB16CFG_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_HB16CFG_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADMUX - AD\\[15:0\\]"]
    #[inline(always)]
    pub fn epi_hb16cfg_mode_admux(self) -> &'a mut W {
        self.variant(EPI_HB16CFG_MODEW::EPI_HB16CFG_MODE_ADMUX)
    }
    #[doc = "ADNONMUX - D\\[15:0\\]"]
    #[inline(always)]
    pub fn epi_hb16cfg_mode_adnmux(self) -> &'a mut W {
        self.variant(EPI_HB16CFG_MODEW::EPI_HB16CFG_MODE_ADNMUX)
    }
    #[doc = "Continuous Read - D\\[15:0\\]"]
    #[inline(always)]
    pub fn epi_hb16cfg_mode_sram(self) -> &'a mut W {
        self.variant(EPI_HB16CFG_MODEW::EPI_HB16CFG_MODE_SRAM)
    }
    #[doc = "XFIFO - D\\[15:0\\]"]
    #[inline(always)]
    pub fn epi_hb16cfg_mode_xfifo(self) -> &'a mut W {
        self.variant(EPI_HB16CFG_MODEW::EPI_HB16CFG_MODE_XFIFO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB16CFG_BSELR {
    bits: bool,
}
impl EPI_HB16CFG_BSELR {
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
pub struct _EPI_HB16CFG_BSELW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_BSELW<'a> {
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
#[doc = "Possible values of the field `EPI_HB16CFG_RDWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG_RDWSR {
    #[doc = "Active RDn is 2 EPI clocks"]
    EPI_HB16CFG_RDWS_2,
    #[doc = "Active RDn is 4 EPI clocks"]
    EPI_HB16CFG_RDWS_4,
    #[doc = "Active RDn is 6 EPI clocks"]
    EPI_HB16CFG_RDWS_6,
    #[doc = "Active RDn is 8 EPI clocks"]
    EPI_HB16CFG_RDWS_8,
}
impl EPI_HB16CFG_RDWSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG_RDWSR::EPI_HB16CFG_RDWS_2 => 0,
            EPI_HB16CFG_RDWSR::EPI_HB16CFG_RDWS_4 => 1,
            EPI_HB16CFG_RDWSR::EPI_HB16CFG_RDWS_6 => 2,
            EPI_HB16CFG_RDWSR::EPI_HB16CFG_RDWS_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_HB16CFG_RDWSR {
        match value {
            0 => EPI_HB16CFG_RDWSR::EPI_HB16CFG_RDWS_2,
            1 => EPI_HB16CFG_RDWSR::EPI_HB16CFG_RDWS_4,
            2 => EPI_HB16CFG_RDWSR::EPI_HB16CFG_RDWS_6,
            3 => EPI_HB16CFG_RDWSR::EPI_HB16CFG_RDWS_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG_RDWS_2`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg_rdws_2(&self) -> bool {
        *self == EPI_HB16CFG_RDWSR::EPI_HB16CFG_RDWS_2
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG_RDWS_4`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg_rdws_4(&self) -> bool {
        *self == EPI_HB16CFG_RDWSR::EPI_HB16CFG_RDWS_4
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG_RDWS_6`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg_rdws_6(&self) -> bool {
        *self == EPI_HB16CFG_RDWSR::EPI_HB16CFG_RDWS_6
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG_RDWS_8`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg_rdws_8(&self) -> bool {
        *self == EPI_HB16CFG_RDWSR::EPI_HB16CFG_RDWS_8
    }
}
#[doc = "Values that can be written to the field `EPI_HB16CFG_RDWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG_RDWSW {
    #[doc = "Active RDn is 2 EPI clocks"]
    EPI_HB16CFG_RDWS_2,
    #[doc = "Active RDn is 4 EPI clocks"]
    EPI_HB16CFG_RDWS_4,
    #[doc = "Active RDn is 6 EPI clocks"]
    EPI_HB16CFG_RDWS_6,
    #[doc = "Active RDn is 8 EPI clocks"]
    EPI_HB16CFG_RDWS_8,
}
impl EPI_HB16CFG_RDWSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG_RDWSW::EPI_HB16CFG_RDWS_2 => 0,
            EPI_HB16CFG_RDWSW::EPI_HB16CFG_RDWS_4 => 1,
            EPI_HB16CFG_RDWSW::EPI_HB16CFG_RDWS_6 => 2,
            EPI_HB16CFG_RDWSW::EPI_HB16CFG_RDWS_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB16CFG_RDWSW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_RDWSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_HB16CFG_RDWSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Active RDn is 2 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg_rdws_2(self) -> &'a mut W {
        self.variant(EPI_HB16CFG_RDWSW::EPI_HB16CFG_RDWS_2)
    }
    #[doc = "Active RDn is 4 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg_rdws_4(self) -> &'a mut W {
        self.variant(EPI_HB16CFG_RDWSW::EPI_HB16CFG_RDWS_4)
    }
    #[doc = "Active RDn is 6 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg_rdws_6(self) -> &'a mut W {
        self.variant(EPI_HB16CFG_RDWSW::EPI_HB16CFG_RDWS_6)
    }
    #[doc = "Active RDn is 8 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg_rdws_8(self) -> &'a mut W {
        self.variant(EPI_HB16CFG_RDWSW::EPI_HB16CFG_RDWS_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = "Possible values of the field `EPI_HB16CFG_WRWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG_WRWSR {
    #[doc = "Active WRn is 2 EPI clocks"]
    EPI_HB16CFG_WRWS_2,
    #[doc = "Active WRn is 4 EPI clocks"]
    EPI_HB16CFG_WRWS_4,
    #[doc = "Active WRn is 6 EPI clocks"]
    EPI_HB16CFG_WRWS_6,
    #[doc = "Active WRn is 8 EPI clocks"]
    EPI_HB16CFG_WRWS_8,
}
impl EPI_HB16CFG_WRWSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG_WRWSR::EPI_HB16CFG_WRWS_2 => 0,
            EPI_HB16CFG_WRWSR::EPI_HB16CFG_WRWS_4 => 1,
            EPI_HB16CFG_WRWSR::EPI_HB16CFG_WRWS_6 => 2,
            EPI_HB16CFG_WRWSR::EPI_HB16CFG_WRWS_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_HB16CFG_WRWSR {
        match value {
            0 => EPI_HB16CFG_WRWSR::EPI_HB16CFG_WRWS_2,
            1 => EPI_HB16CFG_WRWSR::EPI_HB16CFG_WRWS_4,
            2 => EPI_HB16CFG_WRWSR::EPI_HB16CFG_WRWS_6,
            3 => EPI_HB16CFG_WRWSR::EPI_HB16CFG_WRWS_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG_WRWS_2`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg_wrws_2(&self) -> bool {
        *self == EPI_HB16CFG_WRWSR::EPI_HB16CFG_WRWS_2
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG_WRWS_4`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg_wrws_4(&self) -> bool {
        *self == EPI_HB16CFG_WRWSR::EPI_HB16CFG_WRWS_4
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG_WRWS_6`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg_wrws_6(&self) -> bool {
        *self == EPI_HB16CFG_WRWSR::EPI_HB16CFG_WRWS_6
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG_WRWS_8`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg_wrws_8(&self) -> bool {
        *self == EPI_HB16CFG_WRWSR::EPI_HB16CFG_WRWS_8
    }
}
#[doc = "Values that can be written to the field `EPI_HB16CFG_WRWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16CFG_WRWSW {
    #[doc = "Active WRn is 2 EPI clocks"]
    EPI_HB16CFG_WRWS_2,
    #[doc = "Active WRn is 4 EPI clocks"]
    EPI_HB16CFG_WRWS_4,
    #[doc = "Active WRn is 6 EPI clocks"]
    EPI_HB16CFG_WRWS_6,
    #[doc = "Active WRn is 8 EPI clocks"]
    EPI_HB16CFG_WRWS_8,
}
impl EPI_HB16CFG_WRWSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_HB16CFG_WRWSW::EPI_HB16CFG_WRWS_2 => 0,
            EPI_HB16CFG_WRWSW::EPI_HB16CFG_WRWS_4 => 1,
            EPI_HB16CFG_WRWSW::EPI_HB16CFG_WRWS_6 => 2,
            EPI_HB16CFG_WRWSW::EPI_HB16CFG_WRWS_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB16CFG_WRWSW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_WRWSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_HB16CFG_WRWSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Active WRn is 2 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg_wrws_2(self) -> &'a mut W {
        self.variant(EPI_HB16CFG_WRWSW::EPI_HB16CFG_WRWS_2)
    }
    #[doc = "Active WRn is 4 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg_wrws_4(self) -> &'a mut W {
        self.variant(EPI_HB16CFG_WRWSW::EPI_HB16CFG_WRWS_4)
    }
    #[doc = "Active WRn is 6 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg_wrws_6(self) -> &'a mut W {
        self.variant(EPI_HB16CFG_WRWSW::EPI_HB16CFG_WRWS_6)
    }
    #[doc = "Active WRn is 8 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg_wrws_8(self) -> &'a mut W {
        self.variant(EPI_HB16CFG_WRWSW::EPI_HB16CFG_WRWS_8)
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
pub struct EPI_HB16CFG_MAXWAITR {
    bits: u8,
}
impl EPI_HB16CFG_MAXWAITR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB16CFG_MAXWAITW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_MAXWAITW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 8);
        self.w.bits |= ((value as u32) & 255) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB16CFG_BURSTR {
    bits: bool,
}
impl EPI_HB16CFG_BURSTR {
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
pub struct _EPI_HB16CFG_BURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_BURSTW<'a> {
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
pub struct EPI_HB16CFG_RDCRER {
    bits: bool,
}
impl EPI_HB16CFG_RDCRER {
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
pub struct _EPI_HB16CFG_RDCREW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_RDCREW<'a> {
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
pub struct EPI_HB16CFG_WRCRER {
    bits: bool,
}
impl EPI_HB16CFG_WRCRER {
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
pub struct _EPI_HB16CFG_WRCREW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_WRCREW<'a> {
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
pub struct EPI_HB16CFG_ALEHIGHR {
    bits: bool,
}
impl EPI_HB16CFG_ALEHIGHR {
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
pub struct _EPI_HB16CFG_ALEHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_ALEHIGHW<'a> {
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
pub struct EPI_HB16CFG_RDHIGHR {
    bits: bool,
}
impl EPI_HB16CFG_RDHIGHR {
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
pub struct _EPI_HB16CFG_RDHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_RDHIGHW<'a> {
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
pub struct EPI_HB16CFG_WRHIGHR {
    bits: bool,
}
impl EPI_HB16CFG_WRHIGHR {
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
pub struct _EPI_HB16CFG_WRHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_WRHIGHW<'a> {
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
#[doc = r"Value of the field"]
pub struct EPI_HB16CFG_XFEENR {
    bits: bool,
}
impl EPI_HB16CFG_XFEENR {
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
pub struct _EPI_HB16CFG_XFEENW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_XFEENW<'a> {
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
        self.w.bits &= !(1 << 22);
        self.w.bits |= ((value as u32) & 1) << 22;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB16CFG_XFFENR {
    bits: bool,
}
impl EPI_HB16CFG_XFFENR {
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
pub struct _EPI_HB16CFG_XFFENW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_XFFENW<'a> {
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
        self.w.bits &= !(1 << 23);
        self.w.bits |= ((value as u32) & 1) << 23;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB16CFG_IRDYINVR {
    bits: bool,
}
impl EPI_HB16CFG_IRDYINVR {
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
pub struct _EPI_HB16CFG_IRDYINVW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_IRDYINVW<'a> {
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
#[doc = r"Value of the field"]
pub struct EPI_HB16CFG_RDYENR {
    bits: bool,
}
impl EPI_HB16CFG_RDYENR {
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
pub struct _EPI_HB16CFG_RDYENW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_RDYENW<'a> {
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
        self.w.bits &= !(1 << 28);
        self.w.bits |= ((value as u32) & 1) << 28;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB16CFG_CLKINVR {
    bits: bool,
}
impl EPI_HB16CFG_CLKINVR {
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
pub struct _EPI_HB16CFG_CLKINVW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_CLKINVW<'a> {
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
        self.w.bits &= !(1 << 29);
        self.w.bits |= ((value as u32) & 1) << 29;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB16CFG_CLKGATEIR {
    bits: bool,
}
impl EPI_HB16CFG_CLKGATEIR {
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
pub struct _EPI_HB16CFG_CLKGATEIW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_CLKGATEIW<'a> {
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
pub struct EPI_HB16CFG_CLKGATER {
    bits: bool,
}
impl EPI_HB16CFG_CLKGATER {
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
pub struct _EPI_HB16CFG_CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16CFG_CLKGATEW<'a> {
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
    #[doc = "Bits 0:1 - Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn epi_hb16cfg_mode(&self) -> EPI_HB16CFG_MODER {
        EPI_HB16CFG_MODER::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bit 2 - Byte Select Configuration"]
    #[inline(always)]
    pub fn epi_hb16cfg_bsel(&self) -> EPI_HB16CFG_BSELR {
        let bits = ((self.bits >> 2) & 1) != 0;
        EPI_HB16CFG_BSELR { bits }
    }
    #[doc = "Bits 4:5 - Read Wait States"]
    #[inline(always)]
    pub fn epi_hb16cfg_rdws(&self) -> EPI_HB16CFG_RDWSR {
        EPI_HB16CFG_RDWSR::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Write Wait States"]
    #[inline(always)]
    pub fn epi_hb16cfg_wrws(&self) -> EPI_HB16CFG_WRWSR {
        EPI_HB16CFG_WRWSR::_from(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Wait"]
    #[inline(always)]
    pub fn epi_hb16cfg_maxwait(&self) -> EPI_HB16CFG_MAXWAITR {
        let bits = ((self.bits >> 8) & 255) as u8;
        EPI_HB16CFG_MAXWAITR { bits }
    }
    #[doc = "Bit 16 - Burst Mode"]
    #[inline(always)]
    pub fn epi_hb16cfg_burst(&self) -> EPI_HB16CFG_BURSTR {
        let bits = ((self.bits >> 16) & 1) != 0;
        EPI_HB16CFG_BURSTR { bits }
    }
    #[doc = "Bit 17 - PSRAM Configuration Register Read"]
    #[inline(always)]
    pub fn epi_hb16cfg_rdcre(&self) -> EPI_HB16CFG_RDCRER {
        let bits = ((self.bits >> 17) & 1) != 0;
        EPI_HB16CFG_RDCRER { bits }
    }
    #[doc = "Bit 18 - PSRAM Configuration Register Write"]
    #[inline(always)]
    pub fn epi_hb16cfg_wrcre(&self) -> EPI_HB16CFG_WRCRER {
        let bits = ((self.bits >> 18) & 1) != 0;
        EPI_HB16CFG_WRCRER { bits }
    }
    #[doc = "Bit 19 - ALE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg_alehigh(&self) -> EPI_HB16CFG_ALEHIGHR {
        let bits = ((self.bits >> 19) & 1) != 0;
        EPI_HB16CFG_ALEHIGHR { bits }
    }
    #[doc = "Bit 20 - READ Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg_rdhigh(&self) -> EPI_HB16CFG_RDHIGHR {
        let bits = ((self.bits >> 20) & 1) != 0;
        EPI_HB16CFG_RDHIGHR { bits }
    }
    #[doc = "Bit 21 - WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg_wrhigh(&self) -> EPI_HB16CFG_WRHIGHR {
        let bits = ((self.bits >> 21) & 1) != 0;
        EPI_HB16CFG_WRHIGHR { bits }
    }
    #[doc = "Bit 22 - External FIFO EMPTY Enable"]
    #[inline(always)]
    pub fn epi_hb16cfg_xfeen(&self) -> EPI_HB16CFG_XFEENR {
        let bits = ((self.bits >> 22) & 1) != 0;
        EPI_HB16CFG_XFEENR { bits }
    }
    #[doc = "Bit 23 - External FIFO FULL Enable"]
    #[inline(always)]
    pub fn epi_hb16cfg_xffen(&self) -> EPI_HB16CFG_XFFENR {
        let bits = ((self.bits >> 23) & 1) != 0;
        EPI_HB16CFG_XFFENR { bits }
    }
    #[doc = "Bit 27 - Input Ready Invert"]
    #[inline(always)]
    pub fn epi_hb16cfg_irdyinv(&self) -> EPI_HB16CFG_IRDYINVR {
        let bits = ((self.bits >> 27) & 1) != 0;
        EPI_HB16CFG_IRDYINVR { bits }
    }
    #[doc = "Bit 28 - Input Ready Enable"]
    #[inline(always)]
    pub fn epi_hb16cfg_rdyen(&self) -> EPI_HB16CFG_RDYENR {
        let bits = ((self.bits >> 28) & 1) != 0;
        EPI_HB16CFG_RDYENR { bits }
    }
    #[doc = "Bit 29 - Invert Output Clock Enable"]
    #[inline(always)]
    pub fn epi_hb16cfg_clkinv(&self) -> EPI_HB16CFG_CLKINVR {
        let bits = ((self.bits >> 29) & 1) != 0;
        EPI_HB16CFG_CLKINVR { bits }
    }
    #[doc = "Bit 30 - Clock Gated Idle"]
    #[inline(always)]
    pub fn epi_hb16cfg_clkgatei(&self) -> EPI_HB16CFG_CLKGATEIR {
        let bits = ((self.bits >> 30) & 1) != 0;
        EPI_HB16CFG_CLKGATEIR { bits }
    }
    #[doc = "Bit 31 - Clock Gated"]
    #[inline(always)]
    pub fn epi_hb16cfg_clkgate(&self) -> EPI_HB16CFG_CLKGATER {
        let bits = ((self.bits >> 31) & 1) != 0;
        EPI_HB16CFG_CLKGATER { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn epi_hb16cfg_mode(&mut self) -> _EPI_HB16CFG_MODEW {
        _EPI_HB16CFG_MODEW { w: self }
    }
    #[doc = "Bit 2 - Byte Select Configuration"]
    #[inline(always)]
    pub fn epi_hb16cfg_bsel(&mut self) -> _EPI_HB16CFG_BSELW {
        _EPI_HB16CFG_BSELW { w: self }
    }
    #[doc = "Bits 4:5 - Read Wait States"]
    #[inline(always)]
    pub fn epi_hb16cfg_rdws(&mut self) -> _EPI_HB16CFG_RDWSW {
        _EPI_HB16CFG_RDWSW { w: self }
    }
    #[doc = "Bits 6:7 - Write Wait States"]
    #[inline(always)]
    pub fn epi_hb16cfg_wrws(&mut self) -> _EPI_HB16CFG_WRWSW {
        _EPI_HB16CFG_WRWSW { w: self }
    }
    #[doc = "Bits 8:15 - Maximum Wait"]
    #[inline(always)]
    pub fn epi_hb16cfg_maxwait(&mut self) -> _EPI_HB16CFG_MAXWAITW {
        _EPI_HB16CFG_MAXWAITW { w: self }
    }
    #[doc = "Bit 16 - Burst Mode"]
    #[inline(always)]
    pub fn epi_hb16cfg_burst(&mut self) -> _EPI_HB16CFG_BURSTW {
        _EPI_HB16CFG_BURSTW { w: self }
    }
    #[doc = "Bit 17 - PSRAM Configuration Register Read"]
    #[inline(always)]
    pub fn epi_hb16cfg_rdcre(&mut self) -> _EPI_HB16CFG_RDCREW {
        _EPI_HB16CFG_RDCREW { w: self }
    }
    #[doc = "Bit 18 - PSRAM Configuration Register Write"]
    #[inline(always)]
    pub fn epi_hb16cfg_wrcre(&mut self) -> _EPI_HB16CFG_WRCREW {
        _EPI_HB16CFG_WRCREW { w: self }
    }
    #[doc = "Bit 19 - ALE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg_alehigh(&mut self) -> _EPI_HB16CFG_ALEHIGHW {
        _EPI_HB16CFG_ALEHIGHW { w: self }
    }
    #[doc = "Bit 20 - READ Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg_rdhigh(&mut self) -> _EPI_HB16CFG_RDHIGHW {
        _EPI_HB16CFG_RDHIGHW { w: self }
    }
    #[doc = "Bit 21 - WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg_wrhigh(&mut self) -> _EPI_HB16CFG_WRHIGHW {
        _EPI_HB16CFG_WRHIGHW { w: self }
    }
    #[doc = "Bit 22 - External FIFO EMPTY Enable"]
    #[inline(always)]
    pub fn epi_hb16cfg_xfeen(&mut self) -> _EPI_HB16CFG_XFEENW {
        _EPI_HB16CFG_XFEENW { w: self }
    }
    #[doc = "Bit 23 - External FIFO FULL Enable"]
    #[inline(always)]
    pub fn epi_hb16cfg_xffen(&mut self) -> _EPI_HB16CFG_XFFENW {
        _EPI_HB16CFG_XFFENW { w: self }
    }
    #[doc = "Bit 27 - Input Ready Invert"]
    #[inline(always)]
    pub fn epi_hb16cfg_irdyinv(&mut self) -> _EPI_HB16CFG_IRDYINVW {
        _EPI_HB16CFG_IRDYINVW { w: self }
    }
    #[doc = "Bit 28 - Input Ready Enable"]
    #[inline(always)]
    pub fn epi_hb16cfg_rdyen(&mut self) -> _EPI_HB16CFG_RDYENW {
        _EPI_HB16CFG_RDYENW { w: self }
    }
    #[doc = "Bit 29 - Invert Output Clock Enable"]
    #[inline(always)]
    pub fn epi_hb16cfg_clkinv(&mut self) -> _EPI_HB16CFG_CLKINVW {
        _EPI_HB16CFG_CLKINVW { w: self }
    }
    #[doc = "Bit 30 - Clock Gated Idle"]
    #[inline(always)]
    pub fn epi_hb16cfg_clkgatei(&mut self) -> _EPI_HB16CFG_CLKGATEIW {
        _EPI_HB16CFG_CLKGATEIW { w: self }
    }
    #[doc = "Bit 31 - Clock Gated"]
    #[inline(always)]
    pub fn epi_hb16cfg_clkgate(&mut self) -> _EPI_HB16CFG_CLKGATEW {
        _EPI_HB16CFG_CLKGATEW { w: self }
    }
}

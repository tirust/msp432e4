#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HB8CFG {
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
#[doc = "Possible values of the field `EPI_HB8CFG_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB8CFG_MODER {
    #[doc = "ADMUX - AD\\[7:0\\]"]
    EPI_HB8CFG_MODE_MUX,
    #[doc = "ADNONMUX - D\\[7:0\\]"]
    EPI_HB8CFG_MODE_NMUX,
    #[doc = "Continuous Read - D\\[7:0\\]"]
    EPI_HB8CFG_MODE_SRAM,
    #[doc = "XFIFO - D\\[7:0\\]"]
    EPI_HB8CFG_MODE_FIFO,
}
impl EPI_HB8CFG_MODER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_HB8CFG_MODER::EPI_HB8CFG_MODE_MUX => 0,
            EPI_HB8CFG_MODER::EPI_HB8CFG_MODE_NMUX => 1,
            EPI_HB8CFG_MODER::EPI_HB8CFG_MODE_SRAM => 2,
            EPI_HB8CFG_MODER::EPI_HB8CFG_MODE_FIFO => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_HB8CFG_MODER {
        match value {
            0 => EPI_HB8CFG_MODER::EPI_HB8CFG_MODE_MUX,
            1 => EPI_HB8CFG_MODER::EPI_HB8CFG_MODE_NMUX,
            2 => EPI_HB8CFG_MODER::EPI_HB8CFG_MODE_SRAM,
            3 => EPI_HB8CFG_MODER::EPI_HB8CFG_MODE_FIFO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_MODE_MUX`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_mode_mux(&self) -> bool {
        *self == EPI_HB8CFG_MODER::EPI_HB8CFG_MODE_MUX
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_MODE_NMUX`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_mode_nmux(&self) -> bool {
        *self == EPI_HB8CFG_MODER::EPI_HB8CFG_MODE_NMUX
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_MODE_SRAM`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_mode_sram(&self) -> bool {
        *self == EPI_HB8CFG_MODER::EPI_HB8CFG_MODE_SRAM
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_MODE_FIFO`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_mode_fifo(&self) -> bool {
        *self == EPI_HB8CFG_MODER::EPI_HB8CFG_MODE_FIFO
    }
}
#[doc = "Values that can be written to the field `EPI_HB8CFG_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB8CFG_MODEW {
    #[doc = "ADMUX - AD\\[7:0\\]"]
    EPI_HB8CFG_MODE_MUX,
    #[doc = "ADNONMUX - D\\[7:0\\]"]
    EPI_HB8CFG_MODE_NMUX,
    #[doc = "Continuous Read - D\\[7:0\\]"]
    EPI_HB8CFG_MODE_SRAM,
    #[doc = "XFIFO - D\\[7:0\\]"]
    EPI_HB8CFG_MODE_FIFO,
}
impl EPI_HB8CFG_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_HB8CFG_MODEW::EPI_HB8CFG_MODE_MUX => 0,
            EPI_HB8CFG_MODEW::EPI_HB8CFG_MODE_NMUX => 1,
            EPI_HB8CFG_MODEW::EPI_HB8CFG_MODE_SRAM => 2,
            EPI_HB8CFG_MODEW::EPI_HB8CFG_MODE_FIFO => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB8CFG_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8CFG_MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_HB8CFG_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADMUX - AD\\[7:0\\]"]
    #[inline(always)]
    pub fn epi_hb8cfg_mode_mux(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_MODEW::EPI_HB8CFG_MODE_MUX)
    }
    #[doc = "ADNONMUX - D\\[7:0\\]"]
    #[inline(always)]
    pub fn epi_hb8cfg_mode_nmux(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_MODEW::EPI_HB8CFG_MODE_NMUX)
    }
    #[doc = "Continuous Read - D\\[7:0\\]"]
    #[inline(always)]
    pub fn epi_hb8cfg_mode_sram(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_MODEW::EPI_HB8CFG_MODE_SRAM)
    }
    #[doc = "XFIFO - D\\[7:0\\]"]
    #[inline(always)]
    pub fn epi_hb8cfg_mode_fifo(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_MODEW::EPI_HB8CFG_MODE_FIFO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `EPI_HB8CFG_RDWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB8CFG_RDWSR {
    #[doc = "Active RDn is 2 EPI clocks"]
    EPI_HB8CFG_RDWS_2,
    #[doc = "Active RDn is 4 EPI clocks"]
    EPI_HB8CFG_RDWS_4,
    #[doc = "Active RDn is 6 EPI clocks"]
    EPI_HB8CFG_RDWS_6,
    #[doc = "Active RDn is 8 EPI clocks"]
    EPI_HB8CFG_RDWS_8,
}
impl EPI_HB8CFG_RDWSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_HB8CFG_RDWSR::EPI_HB8CFG_RDWS_2 => 0,
            EPI_HB8CFG_RDWSR::EPI_HB8CFG_RDWS_4 => 1,
            EPI_HB8CFG_RDWSR::EPI_HB8CFG_RDWS_6 => 2,
            EPI_HB8CFG_RDWSR::EPI_HB8CFG_RDWS_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_HB8CFG_RDWSR {
        match value {
            0 => EPI_HB8CFG_RDWSR::EPI_HB8CFG_RDWS_2,
            1 => EPI_HB8CFG_RDWSR::EPI_HB8CFG_RDWS_4,
            2 => EPI_HB8CFG_RDWSR::EPI_HB8CFG_RDWS_6,
            3 => EPI_HB8CFG_RDWSR::EPI_HB8CFG_RDWS_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_RDWS_2`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_rdws_2(&self) -> bool {
        *self == EPI_HB8CFG_RDWSR::EPI_HB8CFG_RDWS_2
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_RDWS_4`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_rdws_4(&self) -> bool {
        *self == EPI_HB8CFG_RDWSR::EPI_HB8CFG_RDWS_4
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_RDWS_6`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_rdws_6(&self) -> bool {
        *self == EPI_HB8CFG_RDWSR::EPI_HB8CFG_RDWS_6
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_RDWS_8`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_rdws_8(&self) -> bool {
        *self == EPI_HB8CFG_RDWSR::EPI_HB8CFG_RDWS_8
    }
}
#[doc = "Values that can be written to the field `EPI_HB8CFG_RDWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB8CFG_RDWSW {
    #[doc = "Active RDn is 2 EPI clocks"]
    EPI_HB8CFG_RDWS_2,
    #[doc = "Active RDn is 4 EPI clocks"]
    EPI_HB8CFG_RDWS_4,
    #[doc = "Active RDn is 6 EPI clocks"]
    EPI_HB8CFG_RDWS_6,
    #[doc = "Active RDn is 8 EPI clocks"]
    EPI_HB8CFG_RDWS_8,
}
impl EPI_HB8CFG_RDWSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_HB8CFG_RDWSW::EPI_HB8CFG_RDWS_2 => 0,
            EPI_HB8CFG_RDWSW::EPI_HB8CFG_RDWS_4 => 1,
            EPI_HB8CFG_RDWSW::EPI_HB8CFG_RDWS_6 => 2,
            EPI_HB8CFG_RDWSW::EPI_HB8CFG_RDWS_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB8CFG_RDWSW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8CFG_RDWSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_HB8CFG_RDWSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Active RDn is 2 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdws_2(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_RDWSW::EPI_HB8CFG_RDWS_2)
    }
    #[doc = "Active RDn is 4 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdws_4(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_RDWSW::EPI_HB8CFG_RDWS_4)
    }
    #[doc = "Active RDn is 6 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdws_6(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_RDWSW::EPI_HB8CFG_RDWS_6)
    }
    #[doc = "Active RDn is 8 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdws_8(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_RDWSW::EPI_HB8CFG_RDWS_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = "Possible values of the field `EPI_HB8CFG_WRWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB8CFG_WRWSR {
    #[doc = "Active WRn is 2 EPI clocks"]
    EPI_HB8CFG_WRWS_2,
    #[doc = "Active WRn is 4 EPI clocks"]
    EPI_HB8CFG_WRWS_4,
    #[doc = "Active WRn is 6 EPI clocks"]
    EPI_HB8CFG_WRWS_6,
    #[doc = "Active WRn is 8 EPI clocks"]
    EPI_HB8CFG_WRWS_8,
}
impl EPI_HB8CFG_WRWSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_HB8CFG_WRWSR::EPI_HB8CFG_WRWS_2 => 0,
            EPI_HB8CFG_WRWSR::EPI_HB8CFG_WRWS_4 => 1,
            EPI_HB8CFG_WRWSR::EPI_HB8CFG_WRWS_6 => 2,
            EPI_HB8CFG_WRWSR::EPI_HB8CFG_WRWS_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_HB8CFG_WRWSR {
        match value {
            0 => EPI_HB8CFG_WRWSR::EPI_HB8CFG_WRWS_2,
            1 => EPI_HB8CFG_WRWSR::EPI_HB8CFG_WRWS_4,
            2 => EPI_HB8CFG_WRWSR::EPI_HB8CFG_WRWS_6,
            3 => EPI_HB8CFG_WRWSR::EPI_HB8CFG_WRWS_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_WRWS_2`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_wrws_2(&self) -> bool {
        *self == EPI_HB8CFG_WRWSR::EPI_HB8CFG_WRWS_2
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_WRWS_4`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_wrws_4(&self) -> bool {
        *self == EPI_HB8CFG_WRWSR::EPI_HB8CFG_WRWS_4
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_WRWS_6`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_wrws_6(&self) -> bool {
        *self == EPI_HB8CFG_WRWSR::EPI_HB8CFG_WRWS_6
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_WRWS_8`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_wrws_8(&self) -> bool {
        *self == EPI_HB8CFG_WRWSR::EPI_HB8CFG_WRWS_8
    }
}
#[doc = "Values that can be written to the field `EPI_HB8CFG_WRWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB8CFG_WRWSW {
    #[doc = "Active WRn is 2 EPI clocks"]
    EPI_HB8CFG_WRWS_2,
    #[doc = "Active WRn is 4 EPI clocks"]
    EPI_HB8CFG_WRWS_4,
    #[doc = "Active WRn is 6 EPI clocks"]
    EPI_HB8CFG_WRWS_6,
    #[doc = "Active WRn is 8 EPI clocks"]
    EPI_HB8CFG_WRWS_8,
}
impl EPI_HB8CFG_WRWSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_HB8CFG_WRWSW::EPI_HB8CFG_WRWS_2 => 0,
            EPI_HB8CFG_WRWSW::EPI_HB8CFG_WRWS_4 => 1,
            EPI_HB8CFG_WRWSW::EPI_HB8CFG_WRWS_6 => 2,
            EPI_HB8CFG_WRWSW::EPI_HB8CFG_WRWS_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB8CFG_WRWSW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8CFG_WRWSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_HB8CFG_WRWSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Active WRn is 2 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg_wrws_2(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_WRWSW::EPI_HB8CFG_WRWS_2)
    }
    #[doc = "Active WRn is 4 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg_wrws_4(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_WRWSW::EPI_HB8CFG_WRWS_4)
    }
    #[doc = "Active WRn is 6 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg_wrws_6(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_WRWSW::EPI_HB8CFG_WRWS_6)
    }
    #[doc = "Active WRn is 8 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg_wrws_8(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_WRWSW::EPI_HB8CFG_WRWS_8)
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
pub struct EPI_HB8CFG_MAXWAITR {
    bits: u8,
}
impl EPI_HB8CFG_MAXWAITR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB8CFG_MAXWAITW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8CFG_MAXWAITW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 8);
        self.w.bits |= ((value as u32) & 255) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB8CFG_ALEHIGHR {
    bits: bool,
}
impl EPI_HB8CFG_ALEHIGHR {
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
pub struct _EPI_HB8CFG_ALEHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8CFG_ALEHIGHW<'a> {
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
pub struct EPI_HB8CFG_RDHIGHR {
    bits: bool,
}
impl EPI_HB8CFG_RDHIGHR {
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
pub struct _EPI_HB8CFG_RDHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8CFG_RDHIGHW<'a> {
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
pub struct EPI_HB8CFG_WRHIGHR {
    bits: bool,
}
impl EPI_HB8CFG_WRHIGHR {
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
pub struct _EPI_HB8CFG_WRHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8CFG_WRHIGHW<'a> {
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
pub struct EPI_HB8CFG_XFEENR {
    bits: bool,
}
impl EPI_HB8CFG_XFEENR {
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
pub struct _EPI_HB8CFG_XFEENW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8CFG_XFEENW<'a> {
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
pub struct EPI_HB8CFG_XFFENR {
    bits: bool,
}
impl EPI_HB8CFG_XFFENR {
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
pub struct _EPI_HB8CFG_XFFENW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8CFG_XFFENW<'a> {
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
pub struct EPI_HB8CFG_IRDYINVR {
    bits: bool,
}
impl EPI_HB8CFG_IRDYINVR {
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
pub struct _EPI_HB8CFG_IRDYINVW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8CFG_IRDYINVW<'a> {
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
pub struct EPI_HB8CFG_RDYENR {
    bits: bool,
}
impl EPI_HB8CFG_RDYENR {
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
pub struct _EPI_HB8CFG_RDYENW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8CFG_RDYENW<'a> {
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
pub struct EPI_HB8CFG_CLKINVR {
    bits: bool,
}
impl EPI_HB8CFG_CLKINVR {
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
pub struct _EPI_HB8CFG_CLKINVW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8CFG_CLKINVW<'a> {
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
pub struct EPI_HB8CFG_CLKGATEIR {
    bits: bool,
}
impl EPI_HB8CFG_CLKGATEIR {
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
pub struct _EPI_HB8CFG_CLKGATEIW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8CFG_CLKGATEIW<'a> {
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
pub struct EPI_HB8CFG_CLKGATER {
    bits: bool,
}
impl EPI_HB8CFG_CLKGATER {
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
pub struct _EPI_HB8CFG_CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8CFG_CLKGATEW<'a> {
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
    pub fn epi_hb8cfg_mode(&self) -> EPI_HB8CFG_MODER {
        EPI_HB8CFG_MODER::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Read Wait States"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdws(&self) -> EPI_HB8CFG_RDWSR {
        EPI_HB8CFG_RDWSR::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Write Wait States"]
    #[inline(always)]
    pub fn epi_hb8cfg_wrws(&self) -> EPI_HB8CFG_WRWSR {
        EPI_HB8CFG_WRWSR::_from(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Wait"]
    #[inline(always)]
    pub fn epi_hb8cfg_maxwait(&self) -> EPI_HB8CFG_MAXWAITR {
        let bits = ((self.bits >> 8) & 255) as u8;
        EPI_HB8CFG_MAXWAITR { bits }
    }
    #[doc = "Bit 19 - ALE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg_alehigh(&self) -> EPI_HB8CFG_ALEHIGHR {
        let bits = ((self.bits >> 19) & 1) != 0;
        EPI_HB8CFG_ALEHIGHR { bits }
    }
    #[doc = "Bit 20 - READ Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdhigh(&self) -> EPI_HB8CFG_RDHIGHR {
        let bits = ((self.bits >> 20) & 1) != 0;
        EPI_HB8CFG_RDHIGHR { bits }
    }
    #[doc = "Bit 21 - WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg_wrhigh(&self) -> EPI_HB8CFG_WRHIGHR {
        let bits = ((self.bits >> 21) & 1) != 0;
        EPI_HB8CFG_WRHIGHR { bits }
    }
    #[doc = "Bit 22 - External FIFO EMPTY Enable"]
    #[inline(always)]
    pub fn epi_hb8cfg_xfeen(&self) -> EPI_HB8CFG_XFEENR {
        let bits = ((self.bits >> 22) & 1) != 0;
        EPI_HB8CFG_XFEENR { bits }
    }
    #[doc = "Bit 23 - External FIFO FULL Enable"]
    #[inline(always)]
    pub fn epi_hb8cfg_xffen(&self) -> EPI_HB8CFG_XFFENR {
        let bits = ((self.bits >> 23) & 1) != 0;
        EPI_HB8CFG_XFFENR { bits }
    }
    #[doc = "Bit 27 - Input Ready Invert"]
    #[inline(always)]
    pub fn epi_hb8cfg_irdyinv(&self) -> EPI_HB8CFG_IRDYINVR {
        let bits = ((self.bits >> 27) & 1) != 0;
        EPI_HB8CFG_IRDYINVR { bits }
    }
    #[doc = "Bit 28 - Input Ready Enable"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdyen(&self) -> EPI_HB8CFG_RDYENR {
        let bits = ((self.bits >> 28) & 1) != 0;
        EPI_HB8CFG_RDYENR { bits }
    }
    #[doc = "Bit 29 - Invert Output Clock Enable"]
    #[inline(always)]
    pub fn epi_hb8cfg_clkinv(&self) -> EPI_HB8CFG_CLKINVR {
        let bits = ((self.bits >> 29) & 1) != 0;
        EPI_HB8CFG_CLKINVR { bits }
    }
    #[doc = "Bit 30 - Clock Gated when Idle"]
    #[inline(always)]
    pub fn epi_hb8cfg_clkgatei(&self) -> EPI_HB8CFG_CLKGATEIR {
        let bits = ((self.bits >> 30) & 1) != 0;
        EPI_HB8CFG_CLKGATEIR { bits }
    }
    #[doc = "Bit 31 - Clock Gated"]
    #[inline(always)]
    pub fn epi_hb8cfg_clkgate(&self) -> EPI_HB8CFG_CLKGATER {
        let bits = ((self.bits >> 31) & 1) != 0;
        EPI_HB8CFG_CLKGATER { bits }
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
    pub fn epi_hb8cfg_mode(&mut self) -> _EPI_HB8CFG_MODEW {
        _EPI_HB8CFG_MODEW { w: self }
    }
    #[doc = "Bits 4:5 - Read Wait States"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdws(&mut self) -> _EPI_HB8CFG_RDWSW {
        _EPI_HB8CFG_RDWSW { w: self }
    }
    #[doc = "Bits 6:7 - Write Wait States"]
    #[inline(always)]
    pub fn epi_hb8cfg_wrws(&mut self) -> _EPI_HB8CFG_WRWSW {
        _EPI_HB8CFG_WRWSW { w: self }
    }
    #[doc = "Bits 8:15 - Maximum Wait"]
    #[inline(always)]
    pub fn epi_hb8cfg_maxwait(&mut self) -> _EPI_HB8CFG_MAXWAITW {
        _EPI_HB8CFG_MAXWAITW { w: self }
    }
    #[doc = "Bit 19 - ALE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg_alehigh(&mut self) -> _EPI_HB8CFG_ALEHIGHW {
        _EPI_HB8CFG_ALEHIGHW { w: self }
    }
    #[doc = "Bit 20 - READ Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdhigh(&mut self) -> _EPI_HB8CFG_RDHIGHW {
        _EPI_HB8CFG_RDHIGHW { w: self }
    }
    #[doc = "Bit 21 - WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg_wrhigh(&mut self) -> _EPI_HB8CFG_WRHIGHW {
        _EPI_HB8CFG_WRHIGHW { w: self }
    }
    #[doc = "Bit 22 - External FIFO EMPTY Enable"]
    #[inline(always)]
    pub fn epi_hb8cfg_xfeen(&mut self) -> _EPI_HB8CFG_XFEENW {
        _EPI_HB8CFG_XFEENW { w: self }
    }
    #[doc = "Bit 23 - External FIFO FULL Enable"]
    #[inline(always)]
    pub fn epi_hb8cfg_xffen(&mut self) -> _EPI_HB8CFG_XFFENW {
        _EPI_HB8CFG_XFFENW { w: self }
    }
    #[doc = "Bit 27 - Input Ready Invert"]
    #[inline(always)]
    pub fn epi_hb8cfg_irdyinv(&mut self) -> _EPI_HB8CFG_IRDYINVW {
        _EPI_HB8CFG_IRDYINVW { w: self }
    }
    #[doc = "Bit 28 - Input Ready Enable"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdyen(&mut self) -> _EPI_HB8CFG_RDYENW {
        _EPI_HB8CFG_RDYENW { w: self }
    }
    #[doc = "Bit 29 - Invert Output Clock Enable"]
    #[inline(always)]
    pub fn epi_hb8cfg_clkinv(&mut self) -> _EPI_HB8CFG_CLKINVW {
        _EPI_HB8CFG_CLKINVW { w: self }
    }
    #[doc = "Bit 30 - Clock Gated when Idle"]
    #[inline(always)]
    pub fn epi_hb8cfg_clkgatei(&mut self) -> _EPI_HB8CFG_CLKGATEIW {
        _EPI_HB8CFG_CLKGATEIW { w: self }
    }
    #[doc = "Bit 31 - Clock Gated"]
    #[inline(always)]
    pub fn epi_hb8cfg_clkgate(&mut self) -> _EPI_HB8CFG_CLKGATEW {
        _EPI_HB8CFG_CLKGATEW { w: self }
    }
}

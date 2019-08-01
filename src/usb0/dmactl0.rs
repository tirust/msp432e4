#[doc = r"Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::DMACTL0 {
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
    pub const fn reset_value() -> u16 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Value of the field"]
pub struct USB_DMACTL0_ENABLER {
    bits: bool,
}
impl USB_DMACTL0_ENABLER {
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
pub struct _USB_DMACTL0_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DMACTL0_ENABLEW<'a> {
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
        self.w.bits &= !(1 << 0);
        self.w.bits |= ((value as u16) & 1) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_DMACTL0_DIRR {
    bits: bool,
}
impl USB_DMACTL0_DIRR {
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
pub struct _USB_DMACTL0_DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DMACTL0_DIRW<'a> {
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
        self.w.bits &= !(1 << 1);
        self.w.bits |= ((value as u16) & 1) << 1;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_DMACTL0_MODER {
    bits: bool,
}
impl USB_DMACTL0_MODER {
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
pub struct _USB_DMACTL0_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DMACTL0_MODEW<'a> {
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
        self.w.bits |= ((value as u16) & 1) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_DMACTL0_IER {
    bits: bool,
}
impl USB_DMACTL0_IER {
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
pub struct _USB_DMACTL0_IEW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DMACTL0_IEW<'a> {
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
        self.w.bits &= !(1 << 3);
        self.w.bits |= ((value as u16) & 1) << 3;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_DMACTL0_EPR {
    bits: u8,
}
impl USB_DMACTL0_EPR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _USB_DMACTL0_EPW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DMACTL0_EPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 4);
        self.w.bits |= ((value as u16) & 15) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_DMACTL0_ERRR {
    bits: bool,
}
impl USB_DMACTL0_ERRR {
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
pub struct _USB_DMACTL0_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DMACTL0_ERRW<'a> {
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
        self.w.bits &= !(1 << 8);
        self.w.bits |= ((value as u16) & 1) << 8;
        self.w
    }
}
#[doc = "Possible values of the field `USB_DMACTL0_BRSTM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_DMACTL0_BRSTMR {
    #[doc = "Bursts of unspecified length"]
    USB_DMACTL0_BRSTM_ANY,
    #[doc = "INCR4 or unspecified length"]
    USB_DMACTL0_BRSTM_INC4,
    #[doc = "INCR8, INCR4 or unspecified length"]
    USB_DMACTL0_BRSTM_INC8,
    #[doc = "INCR16, INCR8, INCR4 or unspecified length"]
    USB_DMACTL0_BRSTM_INC16,
}
impl USB_DMACTL0_BRSTMR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            USB_DMACTL0_BRSTMR::USB_DMACTL0_BRSTM_ANY => 0,
            USB_DMACTL0_BRSTMR::USB_DMACTL0_BRSTM_INC4 => 1,
            USB_DMACTL0_BRSTMR::USB_DMACTL0_BRSTM_INC8 => 2,
            USB_DMACTL0_BRSTMR::USB_DMACTL0_BRSTM_INC16 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> USB_DMACTL0_BRSTMR {
        match value {
            0 => USB_DMACTL0_BRSTMR::USB_DMACTL0_BRSTM_ANY,
            1 => USB_DMACTL0_BRSTMR::USB_DMACTL0_BRSTM_INC4,
            2 => USB_DMACTL0_BRSTMR::USB_DMACTL0_BRSTM_INC8,
            3 => USB_DMACTL0_BRSTMR::USB_DMACTL0_BRSTM_INC16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USB_DMACTL0_BRSTM_ANY`"]
    #[inline(always)]
    pub fn is_usb_dmactl0_brstm_any(&self) -> bool {
        *self == USB_DMACTL0_BRSTMR::USB_DMACTL0_BRSTM_ANY
    }
    #[doc = "Checks if the value of the field is `USB_DMACTL0_BRSTM_INC4`"]
    #[inline(always)]
    pub fn is_usb_dmactl0_brstm_inc4(&self) -> bool {
        *self == USB_DMACTL0_BRSTMR::USB_DMACTL0_BRSTM_INC4
    }
    #[doc = "Checks if the value of the field is `USB_DMACTL0_BRSTM_INC8`"]
    #[inline(always)]
    pub fn is_usb_dmactl0_brstm_inc8(&self) -> bool {
        *self == USB_DMACTL0_BRSTMR::USB_DMACTL0_BRSTM_INC8
    }
    #[doc = "Checks if the value of the field is `USB_DMACTL0_BRSTM_INC16`"]
    #[inline(always)]
    pub fn is_usb_dmactl0_brstm_inc16(&self) -> bool {
        *self == USB_DMACTL0_BRSTMR::USB_DMACTL0_BRSTM_INC16
    }
}
#[doc = "Values that can be written to the field `USB_DMACTL0_BRSTM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_DMACTL0_BRSTMW {
    #[doc = "Bursts of unspecified length"]
    USB_DMACTL0_BRSTM_ANY,
    #[doc = "INCR4 or unspecified length"]
    USB_DMACTL0_BRSTM_INC4,
    #[doc = "INCR8, INCR4 or unspecified length"]
    USB_DMACTL0_BRSTM_INC8,
    #[doc = "INCR16, INCR8, INCR4 or unspecified length"]
    USB_DMACTL0_BRSTM_INC16,
}
impl USB_DMACTL0_BRSTMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USB_DMACTL0_BRSTMW::USB_DMACTL0_BRSTM_ANY => 0,
            USB_DMACTL0_BRSTMW::USB_DMACTL0_BRSTM_INC4 => 1,
            USB_DMACTL0_BRSTMW::USB_DMACTL0_BRSTM_INC8 => 2,
            USB_DMACTL0_BRSTMW::USB_DMACTL0_BRSTM_INC16 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USB_DMACTL0_BRSTMW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DMACTL0_BRSTMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_DMACTL0_BRSTMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bursts of unspecified length"]
    #[inline(always)]
    pub fn usb_dmactl0_brstm_any(self) -> &'a mut W {
        self.variant(USB_DMACTL0_BRSTMW::USB_DMACTL0_BRSTM_ANY)
    }
    #[doc = "INCR4 or unspecified length"]
    #[inline(always)]
    pub fn usb_dmactl0_brstm_inc4(self) -> &'a mut W {
        self.variant(USB_DMACTL0_BRSTMW::USB_DMACTL0_BRSTM_INC4)
    }
    #[doc = "INCR8, INCR4 or unspecified length"]
    #[inline(always)]
    pub fn usb_dmactl0_brstm_inc8(self) -> &'a mut W {
        self.variant(USB_DMACTL0_BRSTMW::USB_DMACTL0_BRSTM_INC8)
    }
    #[doc = "INCR16, INCR8, INCR4 or unspecified length"]
    #[inline(always)]
    pub fn usb_dmactl0_brstm_inc16(self) -> &'a mut W {
        self.variant(USB_DMACTL0_BRSTMW::USB_DMACTL0_BRSTM_INC16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 9);
        self.w.bits |= ((value as u16) & 3) << 9;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - DMA Transfer Enable"]
    #[inline(always)]
    pub fn usb_dmactl0_enable(&self) -> USB_DMACTL0_ENABLER {
        let bits = ((self.bits >> 0) & 1) != 0;
        USB_DMACTL0_ENABLER { bits }
    }
    #[doc = "Bit 1 - DMA Direction"]
    #[inline(always)]
    pub fn usb_dmactl0_dir(&self) -> USB_DMACTL0_DIRR {
        let bits = ((self.bits >> 1) & 1) != 0;
        USB_DMACTL0_DIRR { bits }
    }
    #[doc = "Bit 2 - DMA Transfer Mode"]
    #[inline(always)]
    pub fn usb_dmactl0_mode(&self) -> USB_DMACTL0_MODER {
        let bits = ((self.bits >> 2) & 1) != 0;
        USB_DMACTL0_MODER { bits }
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline(always)]
    pub fn usb_dmactl0_ie(&self) -> USB_DMACTL0_IER {
        let bits = ((self.bits >> 3) & 1) != 0;
        USB_DMACTL0_IER { bits }
    }
    #[doc = "Bits 4:7 - Endpoint number"]
    #[inline(always)]
    pub fn usb_dmactl0_ep(&self) -> USB_DMACTL0_EPR {
        let bits = ((self.bits >> 4) & 15) as u8;
        USB_DMACTL0_EPR { bits }
    }
    #[doc = "Bit 8 - Bus Error Bit"]
    #[inline(always)]
    pub fn usb_dmactl0_err(&self) -> USB_DMACTL0_ERRR {
        let bits = ((self.bits >> 8) & 1) != 0;
        USB_DMACTL0_ERRR { bits }
    }
    #[doc = "Bits 9:10 - Burst Mode"]
    #[inline(always)]
    pub fn usb_dmactl0_brstm(&self) -> USB_DMACTL0_BRSTMR {
        USB_DMACTL0_BRSTMR::_from(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - DMA Transfer Enable"]
    #[inline(always)]
    pub fn usb_dmactl0_enable(&mut self) -> _USB_DMACTL0_ENABLEW {
        _USB_DMACTL0_ENABLEW { w: self }
    }
    #[doc = "Bit 1 - DMA Direction"]
    #[inline(always)]
    pub fn usb_dmactl0_dir(&mut self) -> _USB_DMACTL0_DIRW {
        _USB_DMACTL0_DIRW { w: self }
    }
    #[doc = "Bit 2 - DMA Transfer Mode"]
    #[inline(always)]
    pub fn usb_dmactl0_mode(&mut self) -> _USB_DMACTL0_MODEW {
        _USB_DMACTL0_MODEW { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline(always)]
    pub fn usb_dmactl0_ie(&mut self) -> _USB_DMACTL0_IEW {
        _USB_DMACTL0_IEW { w: self }
    }
    #[doc = "Bits 4:7 - Endpoint number"]
    #[inline(always)]
    pub fn usb_dmactl0_ep(&mut self) -> _USB_DMACTL0_EPW {
        _USB_DMACTL0_EPW { w: self }
    }
    #[doc = "Bit 8 - Bus Error Bit"]
    #[inline(always)]
    pub fn usb_dmactl0_err(&mut self) -> _USB_DMACTL0_ERRW {
        _USB_DMACTL0_ERRW { w: self }
    }
    #[doc = "Bits 9:10 - Burst Mode"]
    #[inline(always)]
    pub fn usb_dmactl0_brstm(&mut self) -> _USB_DMACTL0_BRSTMW {
        _USB_DMACTL0_BRSTMW { w: self }
    }
}

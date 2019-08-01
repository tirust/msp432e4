#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::DMAINTR {
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
    pub const fn reset_value() -> u8 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Value of the field"]
pub struct USB_DMAINTR_CH0R {
    bits: bool,
}
impl USB_DMAINTR_CH0R {
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
pub struct _USB_DMAINTR_CH0W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DMAINTR_CH0W<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_DMAINTR_CH1R {
    bits: bool,
}
impl USB_DMAINTR_CH1R {
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
pub struct _USB_DMAINTR_CH1W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DMAINTR_CH1W<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 1;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_DMAINTR_CH2R {
    bits: bool,
}
impl USB_DMAINTR_CH2R {
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
pub struct _USB_DMAINTR_CH2W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DMAINTR_CH2W<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_DMAINTR_CH3R {
    bits: bool,
}
impl USB_DMAINTR_CH3R {
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
pub struct _USB_DMAINTR_CH3W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DMAINTR_CH3W<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 3;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_DMAINTR_CH4R {
    bits: bool,
}
impl USB_DMAINTR_CH4R {
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
pub struct _USB_DMAINTR_CH4W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DMAINTR_CH4W<'a> {
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
        self.w.bits &= !(1 << 4);
        self.w.bits |= ((value as u8) & 1) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_DMAINTR_CH5R {
    bits: bool,
}
impl USB_DMAINTR_CH5R {
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
pub struct _USB_DMAINTR_CH5W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DMAINTR_CH5W<'a> {
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
        self.w.bits &= !(1 << 5);
        self.w.bits |= ((value as u8) & 1) << 5;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_DMAINTR_CH6R {
    bits: bool,
}
impl USB_DMAINTR_CH6R {
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
pub struct _USB_DMAINTR_CH6W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DMAINTR_CH6W<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_DMAINTR_CH7R {
    bits: bool,
}
impl USB_DMAINTR_CH7R {
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
pub struct _USB_DMAINTR_CH7W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DMAINTR_CH7W<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 7;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Channel 0 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch0(&self) -> USB_DMAINTR_CH0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        USB_DMAINTR_CH0R { bits }
    }
    #[doc = "Bit 1 - Channel 1 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch1(&self) -> USB_DMAINTR_CH1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        USB_DMAINTR_CH1R { bits }
    }
    #[doc = "Bit 2 - Channel 2 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch2(&self) -> USB_DMAINTR_CH2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        USB_DMAINTR_CH2R { bits }
    }
    #[doc = "Bit 3 - Channel 3 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch3(&self) -> USB_DMAINTR_CH3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        USB_DMAINTR_CH3R { bits }
    }
    #[doc = "Bit 4 - Channel 4 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch4(&self) -> USB_DMAINTR_CH4R {
        let bits = ((self.bits >> 4) & 1) != 0;
        USB_DMAINTR_CH4R { bits }
    }
    #[doc = "Bit 5 - Channel 5 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch5(&self) -> USB_DMAINTR_CH5R {
        let bits = ((self.bits >> 5) & 1) != 0;
        USB_DMAINTR_CH5R { bits }
    }
    #[doc = "Bit 6 - Channel 6 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch6(&self) -> USB_DMAINTR_CH6R {
        let bits = ((self.bits >> 6) & 1) != 0;
        USB_DMAINTR_CH6R { bits }
    }
    #[doc = "Bit 7 - Channel 7 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch7(&self) -> USB_DMAINTR_CH7R {
        let bits = ((self.bits >> 7) & 1) != 0;
        USB_DMAINTR_CH7R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Channel 0 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch0(&mut self) -> _USB_DMAINTR_CH0W {
        _USB_DMAINTR_CH0W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch1(&mut self) -> _USB_DMAINTR_CH1W {
        _USB_DMAINTR_CH1W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch2(&mut self) -> _USB_DMAINTR_CH2W {
        _USB_DMAINTR_CH2W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch3(&mut self) -> _USB_DMAINTR_CH3W {
        _USB_DMAINTR_CH3W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch4(&mut self) -> _USB_DMAINTR_CH4W {
        _USB_DMAINTR_CH4W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch5(&mut self) -> _USB_DMAINTR_CH5W {
        _USB_DMAINTR_CH5W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch6(&mut self) -> _USB_DMAINTR_CH6W {
        _USB_DMAINTR_CH6W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 DMA Interrupt"]
    #[inline(always)]
    pub fn usb_dmaintr_ch7(&mut self) -> _USB_DMAINTR_CH7W {
        _USB_DMAINTR_CH7W { w: self }
    }
}

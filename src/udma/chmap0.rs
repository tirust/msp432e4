#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHMAP0 {
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
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP0_CH0SELR {
    bits: u8,
}
impl UDMA_CHMAP0_CH0SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP0_CH0SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP0_CH0SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP0_CH1SELR {
    bits: u8,
}
impl UDMA_CHMAP0_CH1SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP0_CH1SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP0_CH1SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 4);
        self.w.bits |= ((value as u32) & 15) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP0_CH2SELR {
    bits: u8,
}
impl UDMA_CHMAP0_CH2SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP0_CH2SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP0_CH2SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 8);
        self.w.bits |= ((value as u32) & 15) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP0_CH3SELR {
    bits: u8,
}
impl UDMA_CHMAP0_CH3SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP0_CH3SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP0_CH3SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 12);
        self.w.bits |= ((value as u32) & 15) << 12;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP0_CH4SELR {
    bits: u8,
}
impl UDMA_CHMAP0_CH4SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP0_CH4SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP0_CH4SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 16);
        self.w.bits |= ((value as u32) & 15) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP0_CH5SELR {
    bits: u8,
}
impl UDMA_CHMAP0_CH5SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP0_CH5SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP0_CH5SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 20);
        self.w.bits |= ((value as u32) & 15) << 20;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP0_CH6SELR {
    bits: u8,
}
impl UDMA_CHMAP0_CH6SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP0_CH6SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP0_CH6SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 24);
        self.w.bits |= ((value as u32) & 15) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP0_CH7SELR {
    bits: u8,
}
impl UDMA_CHMAP0_CH7SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP0_CH7SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP0_CH7SELW<'a> {
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
    #[doc = "Bits 0:3 - uDMA Channel 0 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch0sel(&self) -> UDMA_CHMAP0_CH0SELR {
        let bits = ((self.bits >> 0) & 15) as u8;
        UDMA_CHMAP0_CH0SELR { bits }
    }
    #[doc = "Bits 4:7 - uDMA Channel 1 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch1sel(&self) -> UDMA_CHMAP0_CH1SELR {
        let bits = ((self.bits >> 4) & 15) as u8;
        UDMA_CHMAP0_CH1SELR { bits }
    }
    #[doc = "Bits 8:11 - uDMA Channel 2 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch2sel(&self) -> UDMA_CHMAP0_CH2SELR {
        let bits = ((self.bits >> 8) & 15) as u8;
        UDMA_CHMAP0_CH2SELR { bits }
    }
    #[doc = "Bits 12:15 - uDMA Channel 3 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch3sel(&self) -> UDMA_CHMAP0_CH3SELR {
        let bits = ((self.bits >> 12) & 15) as u8;
        UDMA_CHMAP0_CH3SELR { bits }
    }
    #[doc = "Bits 16:19 - uDMA Channel 4 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch4sel(&self) -> UDMA_CHMAP0_CH4SELR {
        let bits = ((self.bits >> 16) & 15) as u8;
        UDMA_CHMAP0_CH4SELR { bits }
    }
    #[doc = "Bits 20:23 - uDMA Channel 5 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch5sel(&self) -> UDMA_CHMAP0_CH5SELR {
        let bits = ((self.bits >> 20) & 15) as u8;
        UDMA_CHMAP0_CH5SELR { bits }
    }
    #[doc = "Bits 24:27 - uDMA Channel 6 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch6sel(&self) -> UDMA_CHMAP0_CH6SELR {
        let bits = ((self.bits >> 24) & 15) as u8;
        UDMA_CHMAP0_CH6SELR { bits }
    }
    #[doc = "Bits 28:31 - uDMA Channel 7 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch7sel(&self) -> UDMA_CHMAP0_CH7SELR {
        let bits = ((self.bits >> 28) & 15) as u8;
        UDMA_CHMAP0_CH7SELR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - uDMA Channel 0 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch0sel(&mut self) -> _UDMA_CHMAP0_CH0SELW {
        _UDMA_CHMAP0_CH0SELW { w: self }
    }
    #[doc = "Bits 4:7 - uDMA Channel 1 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch1sel(&mut self) -> _UDMA_CHMAP0_CH1SELW {
        _UDMA_CHMAP0_CH1SELW { w: self }
    }
    #[doc = "Bits 8:11 - uDMA Channel 2 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch2sel(&mut self) -> _UDMA_CHMAP0_CH2SELW {
        _UDMA_CHMAP0_CH2SELW { w: self }
    }
    #[doc = "Bits 12:15 - uDMA Channel 3 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch3sel(&mut self) -> _UDMA_CHMAP0_CH3SELW {
        _UDMA_CHMAP0_CH3SELW { w: self }
    }
    #[doc = "Bits 16:19 - uDMA Channel 4 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch4sel(&mut self) -> _UDMA_CHMAP0_CH4SELW {
        _UDMA_CHMAP0_CH4SELW { w: self }
    }
    #[doc = "Bits 20:23 - uDMA Channel 5 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch5sel(&mut self) -> _UDMA_CHMAP0_CH5SELW {
        _UDMA_CHMAP0_CH5SELW { w: self }
    }
    #[doc = "Bits 24:27 - uDMA Channel 6 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch6sel(&mut self) -> _UDMA_CHMAP0_CH6SELW {
        _UDMA_CHMAP0_CH6SELW { w: self }
    }
    #[doc = "Bits 28:31 - uDMA Channel 7 Source Select"]
    #[inline(always)]
    pub fn udma_chmap0_ch7sel(&mut self) -> _UDMA_CHMAP0_CH7SELW {
        _UDMA_CHMAP0_CH7SELW { w: self }
    }
}

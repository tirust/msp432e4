#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHMAP2 {
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
pub struct UDMA_CHMAP2_CH16SELR {
    bits: u8,
}
impl UDMA_CHMAP2_CH16SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP2_CH16SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP2_CH16SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP2_CH17SELR {
    bits: u8,
}
impl UDMA_CHMAP2_CH17SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP2_CH17SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP2_CH17SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 4);
        self.w.bits |= ((value as u32) & 15) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP2_CH18SELR {
    bits: u8,
}
impl UDMA_CHMAP2_CH18SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP2_CH18SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP2_CH18SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 8);
        self.w.bits |= ((value as u32) & 15) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP2_CH19SELR {
    bits: u8,
}
impl UDMA_CHMAP2_CH19SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP2_CH19SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP2_CH19SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 12);
        self.w.bits |= ((value as u32) & 15) << 12;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP2_CH20SELR {
    bits: u8,
}
impl UDMA_CHMAP2_CH20SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP2_CH20SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP2_CH20SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 16);
        self.w.bits |= ((value as u32) & 15) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP2_CH21SELR {
    bits: u8,
}
impl UDMA_CHMAP2_CH21SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP2_CH21SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP2_CH21SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 20);
        self.w.bits |= ((value as u32) & 15) << 20;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP2_CH22SELR {
    bits: u8,
}
impl UDMA_CHMAP2_CH22SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP2_CH22SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP2_CH22SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 24);
        self.w.bits |= ((value as u32) & 15) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP2_CH23SELR {
    bits: u8,
}
impl UDMA_CHMAP2_CH23SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP2_CH23SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP2_CH23SELW<'a> {
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
    #[doc = "Bits 0:3 - uDMA Channel 16 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch16sel(&self) -> UDMA_CHMAP2_CH16SELR {
        let bits = ((self.bits >> 0) & 15) as u8;
        UDMA_CHMAP2_CH16SELR { bits }
    }
    #[doc = "Bits 4:7 - uDMA Channel 17 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch17sel(&self) -> UDMA_CHMAP2_CH17SELR {
        let bits = ((self.bits >> 4) & 15) as u8;
        UDMA_CHMAP2_CH17SELR { bits }
    }
    #[doc = "Bits 8:11 - uDMA Channel 18 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch18sel(&self) -> UDMA_CHMAP2_CH18SELR {
        let bits = ((self.bits >> 8) & 15) as u8;
        UDMA_CHMAP2_CH18SELR { bits }
    }
    #[doc = "Bits 12:15 - uDMA Channel 19 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch19sel(&self) -> UDMA_CHMAP2_CH19SELR {
        let bits = ((self.bits >> 12) & 15) as u8;
        UDMA_CHMAP2_CH19SELR { bits }
    }
    #[doc = "Bits 16:19 - uDMA Channel 20 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch20sel(&self) -> UDMA_CHMAP2_CH20SELR {
        let bits = ((self.bits >> 16) & 15) as u8;
        UDMA_CHMAP2_CH20SELR { bits }
    }
    #[doc = "Bits 20:23 - uDMA Channel 21 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch21sel(&self) -> UDMA_CHMAP2_CH21SELR {
        let bits = ((self.bits >> 20) & 15) as u8;
        UDMA_CHMAP2_CH21SELR { bits }
    }
    #[doc = "Bits 24:27 - uDMA Channel 22 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch22sel(&self) -> UDMA_CHMAP2_CH22SELR {
        let bits = ((self.bits >> 24) & 15) as u8;
        UDMA_CHMAP2_CH22SELR { bits }
    }
    #[doc = "Bits 28:31 - uDMA Channel 23 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch23sel(&self) -> UDMA_CHMAP2_CH23SELR {
        let bits = ((self.bits >> 28) & 15) as u8;
        UDMA_CHMAP2_CH23SELR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - uDMA Channel 16 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch16sel(&mut self) -> _UDMA_CHMAP2_CH16SELW {
        _UDMA_CHMAP2_CH16SELW { w: self }
    }
    #[doc = "Bits 4:7 - uDMA Channel 17 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch17sel(&mut self) -> _UDMA_CHMAP2_CH17SELW {
        _UDMA_CHMAP2_CH17SELW { w: self }
    }
    #[doc = "Bits 8:11 - uDMA Channel 18 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch18sel(&mut self) -> _UDMA_CHMAP2_CH18SELW {
        _UDMA_CHMAP2_CH18SELW { w: self }
    }
    #[doc = "Bits 12:15 - uDMA Channel 19 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch19sel(&mut self) -> _UDMA_CHMAP2_CH19SELW {
        _UDMA_CHMAP2_CH19SELW { w: self }
    }
    #[doc = "Bits 16:19 - uDMA Channel 20 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch20sel(&mut self) -> _UDMA_CHMAP2_CH20SELW {
        _UDMA_CHMAP2_CH20SELW { w: self }
    }
    #[doc = "Bits 20:23 - uDMA Channel 21 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch21sel(&mut self) -> _UDMA_CHMAP2_CH21SELW {
        _UDMA_CHMAP2_CH21SELW { w: self }
    }
    #[doc = "Bits 24:27 - uDMA Channel 22 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch22sel(&mut self) -> _UDMA_CHMAP2_CH22SELW {
        _UDMA_CHMAP2_CH22SELW { w: self }
    }
    #[doc = "Bits 28:31 - uDMA Channel 23 Source Select"]
    #[inline(always)]
    pub fn udma_chmap2_ch23sel(&mut self) -> _UDMA_CHMAP2_CH23SELW {
        _UDMA_CHMAP2_CH23SELW { w: self }
    }
}

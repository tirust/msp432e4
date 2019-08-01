#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHMAP1 {
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
pub struct UDMA_CHMAP1_CH8SELR {
    bits: u8,
}
impl UDMA_CHMAP1_CH8SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP1_CH8SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP1_CH8SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP1_CH9SELR {
    bits: u8,
}
impl UDMA_CHMAP1_CH9SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP1_CH9SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP1_CH9SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 4);
        self.w.bits |= ((value as u32) & 15) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP1_CH10SELR {
    bits: u8,
}
impl UDMA_CHMAP1_CH10SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP1_CH10SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP1_CH10SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 8);
        self.w.bits |= ((value as u32) & 15) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP1_CH11SELR {
    bits: u8,
}
impl UDMA_CHMAP1_CH11SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP1_CH11SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP1_CH11SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 12);
        self.w.bits |= ((value as u32) & 15) << 12;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP1_CH12SELR {
    bits: u8,
}
impl UDMA_CHMAP1_CH12SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP1_CH12SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP1_CH12SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 16);
        self.w.bits |= ((value as u32) & 15) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP1_CH13SELR {
    bits: u8,
}
impl UDMA_CHMAP1_CH13SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP1_CH13SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP1_CH13SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 20);
        self.w.bits |= ((value as u32) & 15) << 20;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP1_CH14SELR {
    bits: u8,
}
impl UDMA_CHMAP1_CH14SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP1_CH14SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP1_CH14SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 24);
        self.w.bits |= ((value as u32) & 15) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP1_CH15SELR {
    bits: u8,
}
impl UDMA_CHMAP1_CH15SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP1_CH15SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP1_CH15SELW<'a> {
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
    #[doc = "Bits 0:3 - uDMA Channel 8 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch8sel(&self) -> UDMA_CHMAP1_CH8SELR {
        let bits = ((self.bits >> 0) & 15) as u8;
        UDMA_CHMAP1_CH8SELR { bits }
    }
    #[doc = "Bits 4:7 - uDMA Channel 9 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch9sel(&self) -> UDMA_CHMAP1_CH9SELR {
        let bits = ((self.bits >> 4) & 15) as u8;
        UDMA_CHMAP1_CH9SELR { bits }
    }
    #[doc = "Bits 8:11 - uDMA Channel 10 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch10sel(&self) -> UDMA_CHMAP1_CH10SELR {
        let bits = ((self.bits >> 8) & 15) as u8;
        UDMA_CHMAP1_CH10SELR { bits }
    }
    #[doc = "Bits 12:15 - uDMA Channel 11 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch11sel(&self) -> UDMA_CHMAP1_CH11SELR {
        let bits = ((self.bits >> 12) & 15) as u8;
        UDMA_CHMAP1_CH11SELR { bits }
    }
    #[doc = "Bits 16:19 - uDMA Channel 12 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch12sel(&self) -> UDMA_CHMAP1_CH12SELR {
        let bits = ((self.bits >> 16) & 15) as u8;
        UDMA_CHMAP1_CH12SELR { bits }
    }
    #[doc = "Bits 20:23 - uDMA Channel 13 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch13sel(&self) -> UDMA_CHMAP1_CH13SELR {
        let bits = ((self.bits >> 20) & 15) as u8;
        UDMA_CHMAP1_CH13SELR { bits }
    }
    #[doc = "Bits 24:27 - uDMA Channel 14 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch14sel(&self) -> UDMA_CHMAP1_CH14SELR {
        let bits = ((self.bits >> 24) & 15) as u8;
        UDMA_CHMAP1_CH14SELR { bits }
    }
    #[doc = "Bits 28:31 - uDMA Channel 15 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch15sel(&self) -> UDMA_CHMAP1_CH15SELR {
        let bits = ((self.bits >> 28) & 15) as u8;
        UDMA_CHMAP1_CH15SELR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - uDMA Channel 8 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch8sel(&mut self) -> _UDMA_CHMAP1_CH8SELW {
        _UDMA_CHMAP1_CH8SELW { w: self }
    }
    #[doc = "Bits 4:7 - uDMA Channel 9 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch9sel(&mut self) -> _UDMA_CHMAP1_CH9SELW {
        _UDMA_CHMAP1_CH9SELW { w: self }
    }
    #[doc = "Bits 8:11 - uDMA Channel 10 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch10sel(&mut self) -> _UDMA_CHMAP1_CH10SELW {
        _UDMA_CHMAP1_CH10SELW { w: self }
    }
    #[doc = "Bits 12:15 - uDMA Channel 11 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch11sel(&mut self) -> _UDMA_CHMAP1_CH11SELW {
        _UDMA_CHMAP1_CH11SELW { w: self }
    }
    #[doc = "Bits 16:19 - uDMA Channel 12 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch12sel(&mut self) -> _UDMA_CHMAP1_CH12SELW {
        _UDMA_CHMAP1_CH12SELW { w: self }
    }
    #[doc = "Bits 20:23 - uDMA Channel 13 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch13sel(&mut self) -> _UDMA_CHMAP1_CH13SELW {
        _UDMA_CHMAP1_CH13SELW { w: self }
    }
    #[doc = "Bits 24:27 - uDMA Channel 14 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch14sel(&mut self) -> _UDMA_CHMAP1_CH14SELW {
        _UDMA_CHMAP1_CH14SELW { w: self }
    }
    #[doc = "Bits 28:31 - uDMA Channel 15 Source Select"]
    #[inline(always)]
    pub fn udma_chmap1_ch15sel(&mut self) -> _UDMA_CHMAP1_CH15SELW {
        _UDMA_CHMAP1_CH15SELW { w: self }
    }
}

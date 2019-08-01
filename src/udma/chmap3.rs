#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHMAP3 {
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
pub struct UDMA_CHMAP3_CH24SELR {
    bits: u8,
}
impl UDMA_CHMAP3_CH24SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP3_CH24SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP3_CH24SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP3_CH25SELR {
    bits: u8,
}
impl UDMA_CHMAP3_CH25SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP3_CH25SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP3_CH25SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 4);
        self.w.bits |= ((value as u32) & 15) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP3_CH26SELR {
    bits: u8,
}
impl UDMA_CHMAP3_CH26SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP3_CH26SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP3_CH26SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 8);
        self.w.bits |= ((value as u32) & 15) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP3_CH27SELR {
    bits: u8,
}
impl UDMA_CHMAP3_CH27SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP3_CH27SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP3_CH27SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 12);
        self.w.bits |= ((value as u32) & 15) << 12;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP3_CH28SELR {
    bits: u8,
}
impl UDMA_CHMAP3_CH28SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP3_CH28SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP3_CH28SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 16);
        self.w.bits |= ((value as u32) & 15) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP3_CH29SELR {
    bits: u8,
}
impl UDMA_CHMAP3_CH29SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP3_CH29SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP3_CH29SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 20);
        self.w.bits |= ((value as u32) & 15) << 20;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP3_CH30SELR {
    bits: u8,
}
impl UDMA_CHMAP3_CH30SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP3_CH30SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP3_CH30SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 24);
        self.w.bits |= ((value as u32) & 15) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UDMA_CHMAP3_CH31SELR {
    bits: u8,
}
impl UDMA_CHMAP3_CH31SELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _UDMA_CHMAP3_CH31SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_CHMAP3_CH31SELW<'a> {
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
    #[doc = "Bits 0:3 - uDMA Channel 24 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch24sel(&self) -> UDMA_CHMAP3_CH24SELR {
        let bits = ((self.bits >> 0) & 15) as u8;
        UDMA_CHMAP3_CH24SELR { bits }
    }
    #[doc = "Bits 4:7 - uDMA Channel 25 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch25sel(&self) -> UDMA_CHMAP3_CH25SELR {
        let bits = ((self.bits >> 4) & 15) as u8;
        UDMA_CHMAP3_CH25SELR { bits }
    }
    #[doc = "Bits 8:11 - uDMA Channel 26 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch26sel(&self) -> UDMA_CHMAP3_CH26SELR {
        let bits = ((self.bits >> 8) & 15) as u8;
        UDMA_CHMAP3_CH26SELR { bits }
    }
    #[doc = "Bits 12:15 - uDMA Channel 27 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch27sel(&self) -> UDMA_CHMAP3_CH27SELR {
        let bits = ((self.bits >> 12) & 15) as u8;
        UDMA_CHMAP3_CH27SELR { bits }
    }
    #[doc = "Bits 16:19 - uDMA Channel 28 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch28sel(&self) -> UDMA_CHMAP3_CH28SELR {
        let bits = ((self.bits >> 16) & 15) as u8;
        UDMA_CHMAP3_CH28SELR { bits }
    }
    #[doc = "Bits 20:23 - uDMA Channel 29 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch29sel(&self) -> UDMA_CHMAP3_CH29SELR {
        let bits = ((self.bits >> 20) & 15) as u8;
        UDMA_CHMAP3_CH29SELR { bits }
    }
    #[doc = "Bits 24:27 - uDMA Channel 30 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch30sel(&self) -> UDMA_CHMAP3_CH30SELR {
        let bits = ((self.bits >> 24) & 15) as u8;
        UDMA_CHMAP3_CH30SELR { bits }
    }
    #[doc = "Bits 28:31 - uDMA Channel 31 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch31sel(&self) -> UDMA_CHMAP3_CH31SELR {
        let bits = ((self.bits >> 28) & 15) as u8;
        UDMA_CHMAP3_CH31SELR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - uDMA Channel 24 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch24sel(&mut self) -> _UDMA_CHMAP3_CH24SELW {
        _UDMA_CHMAP3_CH24SELW { w: self }
    }
    #[doc = "Bits 4:7 - uDMA Channel 25 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch25sel(&mut self) -> _UDMA_CHMAP3_CH25SELW {
        _UDMA_CHMAP3_CH25SELW { w: self }
    }
    #[doc = "Bits 8:11 - uDMA Channel 26 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch26sel(&mut self) -> _UDMA_CHMAP3_CH26SELW {
        _UDMA_CHMAP3_CH26SELW { w: self }
    }
    #[doc = "Bits 12:15 - uDMA Channel 27 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch27sel(&mut self) -> _UDMA_CHMAP3_CH27SELW {
        _UDMA_CHMAP3_CH27SELW { w: self }
    }
    #[doc = "Bits 16:19 - uDMA Channel 28 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch28sel(&mut self) -> _UDMA_CHMAP3_CH28SELW {
        _UDMA_CHMAP3_CH28SELW { w: self }
    }
    #[doc = "Bits 20:23 - uDMA Channel 29 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch29sel(&mut self) -> _UDMA_CHMAP3_CH29SELW {
        _UDMA_CHMAP3_CH29SELW { w: self }
    }
    #[doc = "Bits 24:27 - uDMA Channel 30 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch30sel(&mut self) -> _UDMA_CHMAP3_CH30SELW {
        _UDMA_CHMAP3_CH30SELW { w: self }
    }
    #[doc = "Bits 28:31 - uDMA Channel 31 Source Select"]
    #[inline(always)]
    pub fn udma_chmap3_ch31sel(&mut self) -> _UDMA_CHMAP3_CH31SELW {
        _UDMA_CHMAP3_CH31SELW { w: self }
    }
}

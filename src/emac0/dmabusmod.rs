#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMABUSMOD {
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
pub struct EMAC_DMABUSMOD_SWRR {
    bits: bool,
}
impl EMAC_DMABUSMOD_SWRR {
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
pub struct _EMAC_DMABUSMOD_SWRW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMABUSMOD_SWRW<'a> {
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
        self.w.bits |= ((value as u32) & 1) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMABUSMOD_DAR {
    bits: bool,
}
impl EMAC_DMABUSMOD_DAR {
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
pub struct _EMAC_DMABUSMOD_DAW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMABUSMOD_DAW<'a> {
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
        self.w.bits |= ((value as u32) & 1) << 1;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMABUSMOD_DSLR {
    bits: u8,
}
impl EMAC_DMABUSMOD_DSLR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_DMABUSMOD_DSLW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMABUSMOD_DSLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(31 << 2);
        self.w.bits |= ((value as u32) & 31) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMABUSMOD_ATDSR {
    bits: bool,
}
impl EMAC_DMABUSMOD_ATDSR {
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
pub struct _EMAC_DMABUSMOD_ATDSW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMABUSMOD_ATDSW<'a> {
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
        self.w.bits |= ((value as u32) & 1) << 7;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMABUSMOD_PBLR {
    bits: u8,
}
impl EMAC_DMABUSMOD_PBLR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_DMABUSMOD_PBLW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMABUSMOD_PBLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(63 << 8);
        self.w.bits |= ((value as u32) & 63) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMABUSMOD_PRR {
    bits: u8,
}
impl EMAC_DMABUSMOD_PRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_DMABUSMOD_PRW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMABUSMOD_PRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 14);
        self.w.bits |= ((value as u32) & 3) << 14;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMABUSMOD_FBR {
    bits: bool,
}
impl EMAC_DMABUSMOD_FBR {
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
pub struct _EMAC_DMABUSMOD_FBW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMABUSMOD_FBW<'a> {
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
pub struct EMAC_DMABUSMOD_RPBLR {
    bits: u8,
}
impl EMAC_DMABUSMOD_RPBLR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_DMABUSMOD_RPBLW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMABUSMOD_RPBLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(63 << 17);
        self.w.bits |= ((value as u32) & 63) << 17;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMABUSMOD_USPR {
    bits: bool,
}
impl EMAC_DMABUSMOD_USPR {
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
pub struct _EMAC_DMABUSMOD_USPW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMABUSMOD_USPW<'a> {
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
pub struct EMAC_DMABUSMOD_8XPBLR {
    bits: bool,
}
impl EMAC_DMABUSMOD_8XPBLR {
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
pub struct _EMAC_DMABUSMOD_8XPBLW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMABUSMOD_8XPBLW<'a> {
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
        self.w.bits &= !(1 << 24);
        self.w.bits |= ((value as u32) & 1) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMABUSMOD_AALR {
    bits: bool,
}
impl EMAC_DMABUSMOD_AALR {
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
pub struct _EMAC_DMABUSMOD_AALW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMABUSMOD_AALW<'a> {
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
        self.w.bits &= !(1 << 25);
        self.w.bits |= ((value as u32) & 1) << 25;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMABUSMOD_MBR {
    bits: bool,
}
impl EMAC_DMABUSMOD_MBR {
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
pub struct _EMAC_DMABUSMOD_MBW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMABUSMOD_MBW<'a> {
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
pub struct EMAC_DMABUSMOD_TXPRR {
    bits: bool,
}
impl EMAC_DMABUSMOD_TXPRR {
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
pub struct _EMAC_DMABUSMOD_TXPRW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMABUSMOD_TXPRW<'a> {
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
pub struct EMAC_DMABUSMOD_RIBR {
    bits: bool,
}
impl EMAC_DMABUSMOD_RIBR {
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
pub struct _EMAC_DMABUSMOD_RIBW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMABUSMOD_RIBW<'a> {
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
    #[doc = "Bit 0 - DMA Software Reset"]
    #[inline(always)]
    pub fn emac_dmabusmod_swr(&self) -> EMAC_DMABUSMOD_SWRR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EMAC_DMABUSMOD_SWRR { bits }
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme"]
    #[inline(always)]
    pub fn emac_dmabusmod_da(&self) -> EMAC_DMABUSMOD_DAR {
        let bits = ((self.bits >> 1) & 1) != 0;
        EMAC_DMABUSMOD_DAR { bits }
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length"]
    #[inline(always)]
    pub fn emac_dmabusmod_dsl(&self) -> EMAC_DMABUSMOD_DSLR {
        let bits = ((self.bits >> 2) & 31) as u8;
        EMAC_DMABUSMOD_DSLR { bits }
    }
    #[doc = "Bit 7 - Alternate Descriptor Size"]
    #[inline(always)]
    pub fn emac_dmabusmod_atds(&self) -> EMAC_DMABUSMOD_ATDSR {
        let bits = ((self.bits >> 7) & 1) != 0;
        EMAC_DMABUSMOD_ATDSR { bits }
    }
    #[doc = "Bits 8:13 - Programmable Burst Length"]
    #[inline(always)]
    pub fn emac_dmabusmod_pbl(&self) -> EMAC_DMABUSMOD_PBLR {
        let bits = ((self.bits >> 8) & 63) as u8;
        EMAC_DMABUSMOD_PBLR { bits }
    }
    #[doc = "Bits 14:15 - Priority Ratio"]
    #[inline(always)]
    pub fn emac_dmabusmod_pr(&self) -> EMAC_DMABUSMOD_PRR {
        let bits = ((self.bits >> 14) & 3) as u8;
        EMAC_DMABUSMOD_PRR { bits }
    }
    #[doc = "Bit 16 - Fixed Burst"]
    #[inline(always)]
    pub fn emac_dmabusmod_fb(&self) -> EMAC_DMABUSMOD_FBR {
        let bits = ((self.bits >> 16) & 1) != 0;
        EMAC_DMABUSMOD_FBR { bits }
    }
    #[doc = "Bits 17:22 - RX DMA Programmable Burst Length (PBL)"]
    #[inline(always)]
    pub fn emac_dmabusmod_rpbl(&self) -> EMAC_DMABUSMOD_RPBLR {
        let bits = ((self.bits >> 17) & 63) as u8;
        EMAC_DMABUSMOD_RPBLR { bits }
    }
    #[doc = "Bit 23 - Use Separate Programmable Burst Length (PBL)"]
    #[inline(always)]
    pub fn emac_dmabusmod_usp(&self) -> EMAC_DMABUSMOD_USPR {
        let bits = ((self.bits >> 23) & 1) != 0;
        EMAC_DMABUSMOD_USPR { bits }
    }
    #[doc = "Bit 24 - 8 x Programmable Burst Length (PBL) Mode"]
    #[inline(always)]
    pub fn emac_dmabusmod_8xpbl(&self) -> EMAC_DMABUSMOD_8XPBLR {
        let bits = ((self.bits >> 24) & 1) != 0;
        EMAC_DMABUSMOD_8XPBLR { bits }
    }
    #[doc = "Bit 25 - Address Aligned Beats"]
    #[inline(always)]
    pub fn emac_dmabusmod_aal(&self) -> EMAC_DMABUSMOD_AALR {
        let bits = ((self.bits >> 25) & 1) != 0;
        EMAC_DMABUSMOD_AALR { bits }
    }
    #[doc = "Bit 26 - Mixed Burst"]
    #[inline(always)]
    pub fn emac_dmabusmod_mb(&self) -> EMAC_DMABUSMOD_MBR {
        let bits = ((self.bits >> 26) & 1) != 0;
        EMAC_DMABUSMOD_MBR { bits }
    }
    #[doc = "Bit 27 - Transmit Priority"]
    #[inline(always)]
    pub fn emac_dmabusmod_txpr(&self) -> EMAC_DMABUSMOD_TXPRR {
        let bits = ((self.bits >> 27) & 1) != 0;
        EMAC_DMABUSMOD_TXPRR { bits }
    }
    #[doc = "Bit 31 - Rebuild Burst"]
    #[inline(always)]
    pub fn emac_dmabusmod_rib(&self) -> EMAC_DMABUSMOD_RIBR {
        let bits = ((self.bits >> 31) & 1) != 0;
        EMAC_DMABUSMOD_RIBR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - DMA Software Reset"]
    #[inline(always)]
    pub fn emac_dmabusmod_swr(&mut self) -> _EMAC_DMABUSMOD_SWRW {
        _EMAC_DMABUSMOD_SWRW { w: self }
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme"]
    #[inline(always)]
    pub fn emac_dmabusmod_da(&mut self) -> _EMAC_DMABUSMOD_DAW {
        _EMAC_DMABUSMOD_DAW { w: self }
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length"]
    #[inline(always)]
    pub fn emac_dmabusmod_dsl(&mut self) -> _EMAC_DMABUSMOD_DSLW {
        _EMAC_DMABUSMOD_DSLW { w: self }
    }
    #[doc = "Bit 7 - Alternate Descriptor Size"]
    #[inline(always)]
    pub fn emac_dmabusmod_atds(&mut self) -> _EMAC_DMABUSMOD_ATDSW {
        _EMAC_DMABUSMOD_ATDSW { w: self }
    }
    #[doc = "Bits 8:13 - Programmable Burst Length"]
    #[inline(always)]
    pub fn emac_dmabusmod_pbl(&mut self) -> _EMAC_DMABUSMOD_PBLW {
        _EMAC_DMABUSMOD_PBLW { w: self }
    }
    #[doc = "Bits 14:15 - Priority Ratio"]
    #[inline(always)]
    pub fn emac_dmabusmod_pr(&mut self) -> _EMAC_DMABUSMOD_PRW {
        _EMAC_DMABUSMOD_PRW { w: self }
    }
    #[doc = "Bit 16 - Fixed Burst"]
    #[inline(always)]
    pub fn emac_dmabusmod_fb(&mut self) -> _EMAC_DMABUSMOD_FBW {
        _EMAC_DMABUSMOD_FBW { w: self }
    }
    #[doc = "Bits 17:22 - RX DMA Programmable Burst Length (PBL)"]
    #[inline(always)]
    pub fn emac_dmabusmod_rpbl(&mut self) -> _EMAC_DMABUSMOD_RPBLW {
        _EMAC_DMABUSMOD_RPBLW { w: self }
    }
    #[doc = "Bit 23 - Use Separate Programmable Burst Length (PBL)"]
    #[inline(always)]
    pub fn emac_dmabusmod_usp(&mut self) -> _EMAC_DMABUSMOD_USPW {
        _EMAC_DMABUSMOD_USPW { w: self }
    }
    #[doc = "Bit 24 - 8 x Programmable Burst Length (PBL) Mode"]
    #[inline(always)]
    pub fn emac_dmabusmod_8xpbl(&mut self) -> _EMAC_DMABUSMOD_8XPBLW {
        _EMAC_DMABUSMOD_8XPBLW { w: self }
    }
    #[doc = "Bit 25 - Address Aligned Beats"]
    #[inline(always)]
    pub fn emac_dmabusmod_aal(&mut self) -> _EMAC_DMABUSMOD_AALW {
        _EMAC_DMABUSMOD_AALW { w: self }
    }
    #[doc = "Bit 26 - Mixed Burst"]
    #[inline(always)]
    pub fn emac_dmabusmod_mb(&mut self) -> _EMAC_DMABUSMOD_MBW {
        _EMAC_DMABUSMOD_MBW { w: self }
    }
    #[doc = "Bit 27 - Transmit Priority"]
    #[inline(always)]
    pub fn emac_dmabusmod_txpr(&mut self) -> _EMAC_DMABUSMOD_TXPRW {
        _EMAC_DMABUSMOD_TXPRW { w: self }
    }
    #[doc = "Bit 31 - Rebuild Burst"]
    #[inline(always)]
    pub fn emac_dmabusmod_rib(&mut self) -> _EMAC_DMABUSMOD_RIBW {
        _EMAC_DMABUSMOD_RIBW { w: self }
    }
}

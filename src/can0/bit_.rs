#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BIT {
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
pub struct CAN_BIT_BRPR {
    bits: u8,
}
impl CAN_BIT_BRPR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _CAN_BIT_BRPW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_BIT_BRPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(63 << 0);
        self.w.bits |= ((value as u32) & 63) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct CAN_BIT_SJWR {
    bits: u8,
}
impl CAN_BIT_SJWR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _CAN_BIT_SJWW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_BIT_SJWW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 6);
        self.w.bits |= ((value as u32) & 3) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct CAN_BIT_TSEG1R {
    bits: u8,
}
impl CAN_BIT_TSEG1R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _CAN_BIT_TSEG1W<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_BIT_TSEG1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 8);
        self.w.bits |= ((value as u32) & 15) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct CAN_BIT_TSEG2R {
    bits: u8,
}
impl CAN_BIT_TSEG2R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _CAN_BIT_TSEG2W<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_BIT_TSEG2W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 12);
        self.w.bits |= ((value as u32) & 7) << 12;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn can_bit_brp(&self) -> CAN_BIT_BRPR {
        let bits = ((self.bits >> 0) & 63) as u8;
        CAN_BIT_BRPR { bits }
    }
    #[doc = "Bits 6:7 - (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub fn can_bit_sjw(&self) -> CAN_BIT_SJWR {
        let bits = ((self.bits >> 6) & 3) as u8;
        CAN_BIT_SJWR { bits }
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn can_bit_tseg1(&self) -> CAN_BIT_TSEG1R {
        let bits = ((self.bits >> 8) & 15) as u8;
        CAN_BIT_TSEG1R { bits }
    }
    #[doc = "Bits 12:14 - Time Segment after Sample Point"]
    #[inline(always)]
    pub fn can_bit_tseg2(&self) -> CAN_BIT_TSEG2R {
        let bits = ((self.bits >> 12) & 7) as u8;
        CAN_BIT_TSEG2R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn can_bit_brp(&mut self) -> _CAN_BIT_BRPW {
        _CAN_BIT_BRPW { w: self }
    }
    #[doc = "Bits 6:7 - (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub fn can_bit_sjw(&mut self) -> _CAN_BIT_SJWW {
        _CAN_BIT_SJWW { w: self }
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn can_bit_tseg1(&mut self) -> _CAN_BIT_TSEG1W {
        _CAN_BIT_TSEG1W { w: self }
    }
    #[doc = "Bits 12:14 - Time Segment after Sample Point"]
    #[inline(always)]
    pub fn can_bit_tseg2(&mut self) -> _CAN_BIT_TSEG2W {
        _CAN_BIT_TSEG2W { w: self }
    }
}

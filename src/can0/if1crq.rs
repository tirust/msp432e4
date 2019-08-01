#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IF1CRQ {
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
pub struct CAN_IF1CRQ_MNUMR {
    bits: u8,
}
impl CAN_IF1CRQ_MNUMR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _CAN_IF1CRQ_MNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF1CRQ_MNUMW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(63 << 0);
        self.w.bits |= ((value as u32) & 63) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct CAN_IF1CRQ_BUSYR {
    bits: bool,
}
impl CAN_IF1CRQ_BUSYR {
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
pub struct _CAN_IF1CRQ_BUSYW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF1CRQ_BUSYW<'a> {
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
        self.w.bits &= !(1 << 15);
        self.w.bits |= ((value as u32) & 1) << 15;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Message Number"]
    #[inline(always)]
    pub fn can_if1crq_mnum(&self) -> CAN_IF1CRQ_MNUMR {
        let bits = ((self.bits >> 0) & 63) as u8;
        CAN_IF1CRQ_MNUMR { bits }
    }
    #[doc = "Bit 15 - Busy Flag"]
    #[inline(always)]
    pub fn can_if1crq_busy(&self) -> CAN_IF1CRQ_BUSYR {
        let bits = ((self.bits >> 15) & 1) != 0;
        CAN_IF1CRQ_BUSYR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Message Number"]
    #[inline(always)]
    pub fn can_if1crq_mnum(&mut self) -> _CAN_IF1CRQ_MNUMW {
        _CAN_IF1CRQ_MNUMW { w: self }
    }
    #[doc = "Bit 15 - Busy Flag"]
    #[inline(always)]
    pub fn can_if1crq_busy(&mut self) -> _CAN_IF1CRQ_BUSYW {
        _CAN_IF1CRQ_BUSYW { w: self }
    }
}

#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IRQSTATUS {
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
pub struct SHAMD5_IRQSTATUS_OUTPUT_READYR {
    bits: bool,
}
impl SHAMD5_IRQSTATUS_OUTPUT_READYR {
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
pub struct _SHAMD5_IRQSTATUS_OUTPUT_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_IRQSTATUS_OUTPUT_READYW<'a> {
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
pub struct SHAMD5_IRQSTATUS_INPUT_READYR {
    bits: bool,
}
impl SHAMD5_IRQSTATUS_INPUT_READYR {
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
pub struct _SHAMD5_IRQSTATUS_INPUT_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_IRQSTATUS_INPUT_READYW<'a> {
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
pub struct SHAMD5_IRQSTATUS_CONTEXT_READYR {
    bits: bool,
}
impl SHAMD5_IRQSTATUS_CONTEXT_READYR {
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
pub struct _SHAMD5_IRQSTATUS_CONTEXT_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_IRQSTATUS_CONTEXT_READYW<'a> {
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
        self.w.bits |= ((value as u32) & 1) << 3;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Output Ready Status"]
    #[inline(always)]
    pub fn shamd5_irqstatus_output_ready(&self) -> SHAMD5_IRQSTATUS_OUTPUT_READYR {
        let bits = ((self.bits >> 0) & 1) != 0;
        SHAMD5_IRQSTATUS_OUTPUT_READYR { bits }
    }
    #[doc = "Bit 1 - Input Ready Status"]
    #[inline(always)]
    pub fn shamd5_irqstatus_input_ready(&self) -> SHAMD5_IRQSTATUS_INPUT_READYR {
        let bits = ((self.bits >> 1) & 1) != 0;
        SHAMD5_IRQSTATUS_INPUT_READYR { bits }
    }
    #[doc = "Bit 3 - Context Ready Status"]
    #[inline(always)]
    pub fn shamd5_irqstatus_context_ready(&self) -> SHAMD5_IRQSTATUS_CONTEXT_READYR {
        let bits = ((self.bits >> 3) & 1) != 0;
        SHAMD5_IRQSTATUS_CONTEXT_READYR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Output Ready Status"]
    #[inline(always)]
    pub fn shamd5_irqstatus_output_ready(&mut self) -> _SHAMD5_IRQSTATUS_OUTPUT_READYW {
        _SHAMD5_IRQSTATUS_OUTPUT_READYW { w: self }
    }
    #[doc = "Bit 1 - Input Ready Status"]
    #[inline(always)]
    pub fn shamd5_irqstatus_input_ready(&mut self) -> _SHAMD5_IRQSTATUS_INPUT_READYW {
        _SHAMD5_IRQSTATUS_INPUT_READYW { w: self }
    }
    #[doc = "Bit 3 - Context Ready Status"]
    #[inline(always)]
    pub fn shamd5_irqstatus_context_ready(&mut self) -> _SHAMD5_IRQSTATUS_CONTEXT_READYW {
        _SHAMD5_IRQSTATUS_CONTEXT_READYW { w: self }
    }
}

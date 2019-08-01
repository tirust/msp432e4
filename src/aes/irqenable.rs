#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IRQENABLE {
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
pub struct AES_IRQENABLE_CONTEXT_INR {
    bits: bool,
}
impl AES_IRQENABLE_CONTEXT_INR {
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
pub struct _AES_IRQENABLE_CONTEXT_INW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_IRQENABLE_CONTEXT_INW<'a> {
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
pub struct AES_IRQENABLE_DATA_INR {
    bits: bool,
}
impl AES_IRQENABLE_DATA_INR {
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
pub struct _AES_IRQENABLE_DATA_INW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_IRQENABLE_DATA_INW<'a> {
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
pub struct AES_IRQENABLE_DATA_OUTR {
    bits: bool,
}
impl AES_IRQENABLE_DATA_OUTR {
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
pub struct _AES_IRQENABLE_DATA_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_IRQENABLE_DATA_OUTW<'a> {
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
        self.w.bits |= ((value as u32) & 1) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct AES_IRQENABLE_CONTEXT_OUTR {
    bits: bool,
}
impl AES_IRQENABLE_CONTEXT_OUTR {
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
pub struct _AES_IRQENABLE_CONTEXT_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_IRQENABLE_CONTEXT_OUTW<'a> {
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
    #[doc = "Bit 0 - Context In Interrupt Enable"]
    #[inline(always)]
    pub fn aes_irqenable_context_in(&self) -> AES_IRQENABLE_CONTEXT_INR {
        let bits = ((self.bits >> 0) & 1) != 0;
        AES_IRQENABLE_CONTEXT_INR { bits }
    }
    #[doc = "Bit 1 - Data In Interrupt Enable"]
    #[inline(always)]
    pub fn aes_irqenable_data_in(&self) -> AES_IRQENABLE_DATA_INR {
        let bits = ((self.bits >> 1) & 1) != 0;
        AES_IRQENABLE_DATA_INR { bits }
    }
    #[doc = "Bit 2 - Data Out Interrupt Enable"]
    #[inline(always)]
    pub fn aes_irqenable_data_out(&self) -> AES_IRQENABLE_DATA_OUTR {
        let bits = ((self.bits >> 2) & 1) != 0;
        AES_IRQENABLE_DATA_OUTR { bits }
    }
    #[doc = "Bit 3 - Context Out Interrupt Enable"]
    #[inline(always)]
    pub fn aes_irqenable_context_out(&self) -> AES_IRQENABLE_CONTEXT_OUTR {
        let bits = ((self.bits >> 3) & 1) != 0;
        AES_IRQENABLE_CONTEXT_OUTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Context In Interrupt Enable"]
    #[inline(always)]
    pub fn aes_irqenable_context_in(&mut self) -> _AES_IRQENABLE_CONTEXT_INW {
        _AES_IRQENABLE_CONTEXT_INW { w: self }
    }
    #[doc = "Bit 1 - Data In Interrupt Enable"]
    #[inline(always)]
    pub fn aes_irqenable_data_in(&mut self) -> _AES_IRQENABLE_DATA_INW {
        _AES_IRQENABLE_DATA_INW { w: self }
    }
    #[doc = "Bit 2 - Data Out Interrupt Enable"]
    #[inline(always)]
    pub fn aes_irqenable_data_out(&mut self) -> _AES_IRQENABLE_DATA_OUTW {
        _AES_IRQENABLE_DATA_OUTW { w: self }
    }
    #[doc = "Bit 3 - Context Out Interrupt Enable"]
    #[inline(always)]
    pub fn aes_irqenable_context_out(&mut self) -> _AES_IRQENABLE_CONTEXT_OUTW {
        _AES_IRQENABLE_CONTEXT_OUTW { w: self }
    }
}

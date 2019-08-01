#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::TEST {
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
pub struct USB_TEST_TESTSE0NAKR {
    bits: bool,
}
impl USB_TEST_TESTSE0NAKR {
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
pub struct _USB_TEST_TESTSE0NAKW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TEST_TESTSE0NAKW<'a> {
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
pub struct USB_TEST_TESTJR {
    bits: bool,
}
impl USB_TEST_TESTJR {
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
pub struct _USB_TEST_TESTJW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TEST_TESTJW<'a> {
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
pub struct USB_TEST_TESTKR {
    bits: bool,
}
impl USB_TEST_TESTKR {
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
pub struct _USB_TEST_TESTKW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TEST_TESTKW<'a> {
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
pub struct USB_TEST_TESTPKTR {
    bits: bool,
}
impl USB_TEST_TESTPKTR {
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
pub struct _USB_TEST_TESTPKTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TEST_TESTPKTW<'a> {
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
pub struct USB_TEST_FORCEHSR {
    bits: bool,
}
impl USB_TEST_FORCEHSR {
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
pub struct _USB_TEST_FORCEHSW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TEST_FORCEHSW<'a> {
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
pub struct USB_TEST_FORCEFSR {
    bits: bool,
}
impl USB_TEST_FORCEFSR {
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
pub struct _USB_TEST_FORCEFSW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TEST_FORCEFSW<'a> {
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
pub struct USB_TEST_FIFOACCR {
    bits: bool,
}
impl USB_TEST_FIFOACCR {
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
pub struct _USB_TEST_FIFOACCW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TEST_FIFOACCW<'a> {
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
pub struct USB_TEST_FORCEHR {
    bits: bool,
}
impl USB_TEST_FORCEHR {
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
pub struct _USB_TEST_FORCEHW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TEST_FORCEHW<'a> {
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
    #[doc = "Bit 0 - Test_SE0_NAK Test Mode Enable"]
    #[inline(always)]
    pub fn usb_test_testse0nak(&self) -> USB_TEST_TESTSE0NAKR {
        let bits = ((self.bits >> 0) & 1) != 0;
        USB_TEST_TESTSE0NAKR { bits }
    }
    #[doc = "Bit 1 - Test_J Mode Enable"]
    #[inline(always)]
    pub fn usb_test_testj(&self) -> USB_TEST_TESTJR {
        let bits = ((self.bits >> 1) & 1) != 0;
        USB_TEST_TESTJR { bits }
    }
    #[doc = "Bit 2 - Test_K Mode Enable"]
    #[inline(always)]
    pub fn usb_test_testk(&self) -> USB_TEST_TESTKR {
        let bits = ((self.bits >> 2) & 1) != 0;
        USB_TEST_TESTKR { bits }
    }
    #[doc = "Bit 3 - Test Packet Mode Enable"]
    #[inline(always)]
    pub fn usb_test_testpkt(&self) -> USB_TEST_TESTPKTR {
        let bits = ((self.bits >> 3) & 1) != 0;
        USB_TEST_TESTPKTR { bits }
    }
    #[doc = "Bit 4 - Force High-Speed Mode"]
    #[inline(always)]
    pub fn usb_test_forcehs(&self) -> USB_TEST_FORCEHSR {
        let bits = ((self.bits >> 4) & 1) != 0;
        USB_TEST_FORCEHSR { bits }
    }
    #[doc = "Bit 5 - Force Full-Speed Mode"]
    #[inline(always)]
    pub fn usb_test_forcefs(&self) -> USB_TEST_FORCEFSR {
        let bits = ((self.bits >> 5) & 1) != 0;
        USB_TEST_FORCEFSR { bits }
    }
    #[doc = "Bit 6 - FIFO Access"]
    #[inline(always)]
    pub fn usb_test_fifoacc(&self) -> USB_TEST_FIFOACCR {
        let bits = ((self.bits >> 6) & 1) != 0;
        USB_TEST_FIFOACCR { bits }
    }
    #[doc = "Bit 7 - Force Host Mode"]
    #[inline(always)]
    pub fn usb_test_forceh(&self) -> USB_TEST_FORCEHR {
        let bits = ((self.bits >> 7) & 1) != 0;
        USB_TEST_FORCEHR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Test_SE0_NAK Test Mode Enable"]
    #[inline(always)]
    pub fn usb_test_testse0nak(&mut self) -> _USB_TEST_TESTSE0NAKW {
        _USB_TEST_TESTSE0NAKW { w: self }
    }
    #[doc = "Bit 1 - Test_J Mode Enable"]
    #[inline(always)]
    pub fn usb_test_testj(&mut self) -> _USB_TEST_TESTJW {
        _USB_TEST_TESTJW { w: self }
    }
    #[doc = "Bit 2 - Test_K Mode Enable"]
    #[inline(always)]
    pub fn usb_test_testk(&mut self) -> _USB_TEST_TESTKW {
        _USB_TEST_TESTKW { w: self }
    }
    #[doc = "Bit 3 - Test Packet Mode Enable"]
    #[inline(always)]
    pub fn usb_test_testpkt(&mut self) -> _USB_TEST_TESTPKTW {
        _USB_TEST_TESTPKTW { w: self }
    }
    #[doc = "Bit 4 - Force High-Speed Mode"]
    #[inline(always)]
    pub fn usb_test_forcehs(&mut self) -> _USB_TEST_FORCEHSW {
        _USB_TEST_FORCEHSW { w: self }
    }
    #[doc = "Bit 5 - Force Full-Speed Mode"]
    #[inline(always)]
    pub fn usb_test_forcefs(&mut self) -> _USB_TEST_FORCEFSW {
        _USB_TEST_FORCEFSW { w: self }
    }
    #[doc = "Bit 6 - FIFO Access"]
    #[inline(always)]
    pub fn usb_test_fifoacc(&mut self) -> _USB_TEST_FIFOACCW {
        _USB_TEST_FIFOACCW { w: self }
    }
    #[doc = "Bit 7 - Force Host Mode"]
    #[inline(always)]
    pub fn usb_test_forceh(&mut self) -> _USB_TEST_FORCEHW {
        _USB_TEST_FORCEHW { w: self }
    }
}

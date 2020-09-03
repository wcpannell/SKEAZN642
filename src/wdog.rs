#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Control and Status Register 1"]
    pub cs1: CS1,
    #[doc = "0x01 - Watchdog Control and Status Register 2"]
    pub cs2: CS2,
    _reserved_2_cnt: [u8; 2usize],
    _reserved_3_toval: [u8; 2usize],
    _reserved_4_win: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x02 - Watchdog Counter Register: High"]
    #[inline(always)]
    pub fn cnth(&self) -> &CNTH {
        unsafe { &*(((self as *const Self) as *const u8).add(2usize) as *const CNTH) }
    }
    #[doc = "0x02 - Watchdog Counter Register: High"]
    #[inline(always)]
    pub fn cnth_mut(&self) -> &mut CNTH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2usize) as *mut CNTH) }
    }
    #[doc = "0x02 - WDOG_CNT register."]
    #[inline(always)]
    pub fn cnt(&self) -> &CNT {
        unsafe { &*(((self as *const Self) as *const u8).add(2usize) as *const CNT) }
    }
    #[doc = "0x02 - WDOG_CNT register."]
    #[inline(always)]
    pub fn cnt_mut(&self) -> &mut CNT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2usize) as *mut CNT) }
    }
    #[doc = "0x03 - Watchdog Counter Register: Low"]
    #[inline(always)]
    pub fn cntl(&self) -> &CNTL {
        unsafe { &*(((self as *const Self) as *const u8).add(3usize) as *const CNTL) }
    }
    #[doc = "0x03 - Watchdog Counter Register: Low"]
    #[inline(always)]
    pub fn cntl_mut(&self) -> &mut CNTL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3usize) as *mut CNTL) }
    }
    #[doc = "0x04 - WDOG_TOVAL register."]
    #[inline(always)]
    pub fn toval(&self) -> &TOVAL {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const TOVAL) }
    }
    #[doc = "0x04 - WDOG_TOVAL register."]
    #[inline(always)]
    pub fn toval_mut(&self) -> &mut TOVAL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut TOVAL) }
    }
    #[doc = "0x04 - Watchdog Timeout Value Register: High"]
    #[inline(always)]
    pub fn tovalh(&self) -> &TOVALH {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const TOVALH) }
    }
    #[doc = "0x04 - Watchdog Timeout Value Register: High"]
    #[inline(always)]
    pub fn tovalh_mut(&self) -> &mut TOVALH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut TOVALH) }
    }
    #[doc = "0x05 - Watchdog Timeout Value Register: Low"]
    #[inline(always)]
    pub fn tovall(&self) -> &TOVALL {
        unsafe { &*(((self as *const Self) as *const u8).add(5usize) as *const TOVALL) }
    }
    #[doc = "0x05 - Watchdog Timeout Value Register: Low"]
    #[inline(always)]
    pub fn tovall_mut(&self) -> &mut TOVALL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(5usize) as *mut TOVALL) }
    }
    #[doc = "0x06 - WDOG_WIN register."]
    #[inline(always)]
    pub fn win(&self) -> &WIN {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const WIN) }
    }
    #[doc = "0x06 - WDOG_WIN register."]
    #[inline(always)]
    pub fn win_mut(&self) -> &mut WIN {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut WIN) }
    }
    #[doc = "0x06 - Watchdog Window Register: High"]
    #[inline(always)]
    pub fn winh(&self) -> &WINH {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const WINH) }
    }
    #[doc = "0x06 - Watchdog Window Register: High"]
    #[inline(always)]
    pub fn winh_mut(&self) -> &mut WINH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut WINH) }
    }
    #[doc = "0x07 - Watchdog Window Register: Low"]
    #[inline(always)]
    pub fn winl(&self) -> &WINL {
        unsafe { &*(((self as *const Self) as *const u8).add(7usize) as *const WINL) }
    }
    #[doc = "0x07 - Watchdog Window Register: Low"]
    #[inline(always)]
    pub fn winl_mut(&self) -> &mut WINL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(7usize) as *mut WINL) }
    }
}
#[doc = "Watchdog Control and Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs1](cs1) module"]
pub type CS1 = crate::Reg<u8, _CS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS1;
#[doc = "`read()` method returns [cs1::R](cs1::R) reader structure"]
impl crate::Readable for CS1 {}
#[doc = "`write(|w| ..)` method takes [cs1::W](cs1::W) writer structure"]
impl crate::Writable for CS1 {}
#[doc = "Watchdog Control and Status Register 1"]
pub mod cs1;
#[doc = "Watchdog Control and Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs2](cs2) module"]
pub type CS2 = crate::Reg<u8, _CS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS2;
#[doc = "`read()` method returns [cs2::R](cs2::R) reader structure"]
impl crate::Readable for CS2 {}
#[doc = "`write(|w| ..)` method takes [cs2::W](cs2::W) writer structure"]
impl crate::Writable for CS2 {}
#[doc = "Watchdog Control and Status Register 2"]
pub mod cs2;
#[doc = "WDOG_CNT register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u16, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "WDOG_CNT register."]
pub mod cnt;
#[doc = "Watchdog Counter Register: High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnth](cnth) module"]
pub type CNTH = crate::Reg<u8, _CNTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTH;
#[doc = "`read()` method returns [cnth::R](cnth::R) reader structure"]
impl crate::Readable for CNTH {}
#[doc = "Watchdog Counter Register: High"]
pub mod cnth;
#[doc = "Watchdog Counter Register: Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntl](cntl) module"]
pub type CNTL = crate::Reg<u8, _CNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTL;
#[doc = "`read()` method returns [cntl::R](cntl::R) reader structure"]
impl crate::Readable for CNTL {}
#[doc = "Watchdog Counter Register: Low"]
pub mod cntl;
#[doc = "Watchdog Timeout Value Register: High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tovalh](tovalh) module"]
pub type TOVALH = crate::Reg<u8, _TOVALH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOVALH;
#[doc = "`read()` method returns [tovalh::R](tovalh::R) reader structure"]
impl crate::Readable for TOVALH {}
#[doc = "`write(|w| ..)` method takes [tovalh::W](tovalh::W) writer structure"]
impl crate::Writable for TOVALH {}
#[doc = "Watchdog Timeout Value Register: High"]
pub mod tovalh;
#[doc = "WDOG_TOVAL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [toval](toval) module"]
pub type TOVAL = crate::Reg<u16, _TOVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOVAL;
#[doc = "`read()` method returns [toval::R](toval::R) reader structure"]
impl crate::Readable for TOVAL {}
#[doc = "`write(|w| ..)` method takes [toval::W](toval::W) writer structure"]
impl crate::Writable for TOVAL {}
#[doc = "WDOG_TOVAL register."]
pub mod toval;
#[doc = "Watchdog Timeout Value Register: Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tovall](tovall) module"]
pub type TOVALL = crate::Reg<u8, _TOVALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOVALL;
#[doc = "`read()` method returns [tovall::R](tovall::R) reader structure"]
impl crate::Readable for TOVALL {}
#[doc = "`write(|w| ..)` method takes [tovall::W](tovall::W) writer structure"]
impl crate::Writable for TOVALL {}
#[doc = "Watchdog Timeout Value Register: Low"]
pub mod tovall;
#[doc = "Watchdog Window Register: High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winh](winh) module"]
pub type WINH = crate::Reg<u8, _WINH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINH;
#[doc = "`read()` method returns [winh::R](winh::R) reader structure"]
impl crate::Readable for WINH {}
#[doc = "`write(|w| ..)` method takes [winh::W](winh::W) writer structure"]
impl crate::Writable for WINH {}
#[doc = "Watchdog Window Register: High"]
pub mod winh;
#[doc = "WDOG_WIN register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [win](win) module"]
pub type WIN = crate::Reg<u16, _WIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WIN;
#[doc = "`read()` method returns [win::R](win::R) reader structure"]
impl crate::Readable for WIN {}
#[doc = "`write(|w| ..)` method takes [win::W](win::W) writer structure"]
impl crate::Writable for WIN {}
#[doc = "WDOG_WIN register."]
pub mod win;
#[doc = "Watchdog Window Register: Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winl](winl) module"]
pub type WINL = crate::Reg<u8, _WINL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINL;
#[doc = "`read()` method returns [winl::R](winl::R) reader structure"]
impl crate::Readable for WINL {}
#[doc = "`write(|w| ..)` method takes [winl::W](winl::W) writer structure"]
impl crate::Writable for WINL {}
#[doc = "Watchdog Window Register: Low"]
pub mod winl;

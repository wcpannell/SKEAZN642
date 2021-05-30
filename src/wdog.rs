#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Control and Status Register 1"]
    pub cs1: crate::Reg<cs1::CS1_SPEC>,
    #[doc = "0x01 - Watchdog Control and Status Register 2"]
    pub cs2: crate::Reg<cs2::CS2_SPEC>,
    _reserved_2_cntl: [u8; 2usize],
    _reserved_3_tovall: [u8; 2usize],
    _reserved_4_winl: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x02 - Watchdog Counter Register: High"]
    #[inline(always)]
    pub fn wdog_cnth(&self) -> &crate::Reg<wdog_cnth::WDOG_CNTH_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2usize)
                as *const crate::Reg<wdog_cnth::WDOG_CNTH_SPEC>)
        }
    }
    #[doc = "0x02 - WDOG_CNT register."]
    #[inline(always)]
    pub fn wdog_cnt(&self) -> &crate::Reg<wdog_cnt::WDOG_CNT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2usize)
                as *const crate::Reg<wdog_cnt::WDOG_CNT_SPEC>)
        }
    }
    #[doc = "0x03 - Watchdog Counter Register: Low"]
    #[inline(always)]
    pub fn cntl(&self) -> &crate::Reg<cntl::CNTL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3usize)
                as *const crate::Reg<cntl::CNTL_SPEC>)
        }
    }
    #[doc = "0x04 - WDOG_TOVAL register."]
    #[inline(always)]
    pub fn wdog_toval(&self) -> &crate::Reg<wdog_toval::WDOG_TOVAL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<wdog_toval::WDOG_TOVAL_SPEC>)
        }
    }
    #[doc = "0x04 - Watchdog Timeout Value Register: High"]
    #[inline(always)]
    pub fn wdog_tovalh(&self) -> &crate::Reg<wdog_tovalh::WDOG_TOVALH_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<wdog_tovalh::WDOG_TOVALH_SPEC>)
        }
    }
    #[doc = "0x05 - Watchdog Timeout Value Register: Low"]
    #[inline(always)]
    pub fn tovall(&self) -> &crate::Reg<tovall::TOVALL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5usize)
                as *const crate::Reg<tovall::TOVALL_SPEC>)
        }
    }
    #[doc = "0x06 - WDOG_WIN register."]
    #[inline(always)]
    pub fn wdog_win(&self) -> &crate::Reg<wdog_win::WDOG_WIN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(6usize)
                as *const crate::Reg<wdog_win::WDOG_WIN_SPEC>)
        }
    }
    #[doc = "0x06 - Watchdog Window Register: High"]
    #[inline(always)]
    pub fn wdog_winh(&self) -> &crate::Reg<wdog_winh::WDOG_WINH_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(6usize)
                as *const crate::Reg<wdog_winh::WDOG_WINH_SPEC>)
        }
    }
    #[doc = "0x07 - Watchdog Window Register: Low"]
    #[inline(always)]
    pub fn winl(&self) -> &crate::Reg<winl::WINL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(7usize)
                as *const crate::Reg<winl::WINL_SPEC>)
        }
    }
}
#[doc = "CS1 register accessor: an alias for `Reg<CS1_SPEC>`"]
pub type CS1 = crate::Reg<cs1::CS1_SPEC>;
#[doc = "Watchdog Control and Status Register 1"]
pub mod cs1;
#[doc = "CS2 register accessor: an alias for `Reg<CS2_SPEC>`"]
pub type CS2 = crate::Reg<cs2::CS2_SPEC>;
#[doc = "Watchdog Control and Status Register 2"]
pub mod cs2;
#[doc = "WDOG_CNT register accessor: an alias for `Reg<WDOG_CNT_SPEC>`"]
pub type WDOG_CNT = crate::Reg<wdog_cnt::WDOG_CNT_SPEC>;
#[doc = "WDOG_CNT register."]
pub mod wdog_cnt;
#[doc = "WDOG_CNTH register accessor: an alias for `Reg<WDOG_CNTH_SPEC>`"]
pub type WDOG_CNTH = crate::Reg<wdog_cnth::WDOG_CNTH_SPEC>;
#[doc = "Watchdog Counter Register: High"]
pub mod wdog_cnth;
#[doc = "CNTL register accessor: an alias for `Reg<CNTL_SPEC>`"]
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
#[doc = "Watchdog Counter Register: Low"]
pub mod cntl;
#[doc = "WDOG_TOVALH register accessor: an alias for `Reg<WDOG_TOVALH_SPEC>`"]
pub type WDOG_TOVALH = crate::Reg<wdog_tovalh::WDOG_TOVALH_SPEC>;
#[doc = "Watchdog Timeout Value Register: High"]
pub mod wdog_tovalh;
#[doc = "WDOG_TOVAL register accessor: an alias for `Reg<WDOG_TOVAL_SPEC>`"]
pub type WDOG_TOVAL = crate::Reg<wdog_toval::WDOG_TOVAL_SPEC>;
#[doc = "WDOG_TOVAL register."]
pub mod wdog_toval;
#[doc = "TOVALL register accessor: an alias for `Reg<TOVALL_SPEC>`"]
pub type TOVALL = crate::Reg<tovall::TOVALL_SPEC>;
#[doc = "Watchdog Timeout Value Register: Low"]
pub mod tovall;
#[doc = "WDOG_WINH register accessor: an alias for `Reg<WDOG_WINH_SPEC>`"]
pub type WDOG_WINH = crate::Reg<wdog_winh::WDOG_WINH_SPEC>;
#[doc = "Watchdog Window Register: High"]
pub mod wdog_winh;
#[doc = "WDOG_WIN register accessor: an alias for `Reg<WDOG_WIN_SPEC>`"]
pub type WDOG_WIN = crate::Reg<wdog_win::WDOG_WIN_SPEC>;
#[doc = "WDOG_WIN register."]
pub mod wdog_win;
#[doc = "WINL register accessor: an alias for `Reg<WINL_SPEC>`"]
pub type WINL = crate::Reg<winl::WINL_SPEC>;
#[doc = "Watchdog Window Register: Low"]
pub mod winl;

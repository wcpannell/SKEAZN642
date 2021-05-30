#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ICS Control Register 1"]
    pub c1: crate::Reg<c1::C1_SPEC>,
    #[doc = "0x01 - ICS Control Register 2"]
    pub c2: crate::Reg<c2::C2_SPEC>,
    #[doc = "0x02 - ICS Control Register 3"]
    pub c3: crate::Reg<c3::C3_SPEC>,
    #[doc = "0x03 - ICS Control Register 4"]
    pub c4: crate::Reg<c4::C4_SPEC>,
    #[doc = "0x04 - ICS Status Register"]
    pub s: crate::Reg<s::S_SPEC>,
}
#[doc = "C1 register accessor: an alias for `Reg<C1_SPEC>`"]
pub type C1 = crate::Reg<c1::C1_SPEC>;
#[doc = "ICS Control Register 1"]
pub mod c1;
#[doc = "C2 register accessor: an alias for `Reg<C2_SPEC>`"]
pub type C2 = crate::Reg<c2::C2_SPEC>;
#[doc = "ICS Control Register 2"]
pub mod c2;
#[doc = "C3 register accessor: an alias for `Reg<C3_SPEC>`"]
pub type C3 = crate::Reg<c3::C3_SPEC>;
#[doc = "ICS Control Register 3"]
pub mod c3;
#[doc = "C4 register accessor: an alias for `Reg<C4_SPEC>`"]
pub type C4 = crate::Reg<c4::C4_SPEC>;
#[doc = "ICS Control Register 4"]
pub mod c4;
#[doc = "S register accessor: an alias for `Reg<S_SPEC>`"]
pub type S = crate::Reg<s::S_SPEC>;
#[doc = "ICS Status Register"]
pub mod s;

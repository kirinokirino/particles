#![deny(clippy::all)]
#![warn(
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    clippy::clone_on_ref_ptr,
    clippy::else_if_without_else,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_division,
    clippy::let_underscore_must_use,
    clippy::mem_forget,
    clippy::multiple_inherent_impl,
    clippy::unwrap_used,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::string_add,
    clippy::wildcard_enum_match_arm,
    clippy::wrong_pub_self_convention
)]
#![allow(
    clippy::missing_docs_in_private_items,
    unknown_lints,
    clippy::expect_used,
    clippy::shadow_reuse
)]

pub mod components;
pub mod systems;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

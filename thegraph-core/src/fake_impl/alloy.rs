//! Implementation of the [`fake`] crate for generating random test data for different [`alloy`]
//! crate types.
//!
//! ```rust
//! use fake::Fake;
//! use thegraph_core::{
//!    alloy::primitives::{Address, B256, PrimitiveSignature, U256},
//!    fake_impl::alloy::Alloy
//! };
//!
//! // Generate random bytes
//! let bytes: B256 = Alloy.fake();
//!
//! // Generate random U256 value
//! let value: U256 = Alloy.fake();
//!
//! // Generate random address
//! let address: Address = Alloy.fake();
//!
//! // Generate random PrimitiveSignature
//! let signature: PrimitiveSignature = Alloy.fake();
//!
//! // etc.
//! ```

/// The `Alloy` struct is used to implement the [`fake::Dummy`] trait for the `alloy` crate types.
pub struct Alloy;

#[doc(hidden)]
pub mod primitives {
    use alloy::primitives::{Address, PrimitiveSignature, B128, B256, U256};
    use fake::{Dummy, Faker, Rng};

    use super::Alloy;

    impl Dummy<Alloy> for B128 {
        /// Generate a random [`B128`] value.
        ///
        /// ```rust
        /// # use fake::Fake;
        /// use thegraph_core::{
        ///     alloy::primitives::B128,
        ///     fake_impl::alloy::Alloy
        /// };
        ///
        /// let value: B128 = Alloy.fake();
        /// # assert_ne!(value, B128::ZERO);
        /// ```
        fn dummy_with_rng<R: Rng + ?Sized>(_: &Alloy, rng: &mut R) -> Self {
            <[u8; 16]>::dummy_with_rng(&Faker, rng).into()
        }
    }

    impl Dummy<Alloy> for B256 {
        /// Generate a random [`B256`] value.
        ///
        /// ```rust
        /// # use fake::Fake;
        /// use thegraph_core::{
        ///     alloy::primitives::B256,
        ///     fake_impl::alloy::Alloy
        /// };
        ///
        /// let value: B256 = Alloy.fake();
        /// # assert_ne!(value, B256::ZERO);
        /// ```
        fn dummy_with_rng<R: Rng + ?Sized>(_: &Alloy, rng: &mut R) -> Self {
            <[u8; 32]>::dummy_with_rng(&Faker, rng).into()
        }
    }

    impl Dummy<Alloy> for U256 {
        /// Generate a random [`U256`] value.
        ///
        /// ```rust
        /// # use fake::Fake;
        /// use thegraph_core::{
        ///     alloy::primitives::U256,
        ///     fake_impl::alloy::Alloy
        /// };
        ///
        /// let value: U256 = Alloy.fake();
        /// # assert_ne!(value, U256::ZERO);
        fn dummy_with_rng<R: Rng + ?Sized>(_: &Alloy, rng: &mut R) -> Self {
            U256::from_be_bytes(<[u8; 32]>::dummy_with_rng(&Faker, rng))
        }
    }

    impl Dummy<Alloy> for Address {
        /// Generate a random [`Address`] value.
        /// ```rust
        /// # use fake::Fake;
        /// use thegraph_core::{
        ///     alloy::primitives::Address,
        ///     fake_impl::alloy::Alloy
        /// };
        ///
        /// let value: Address = Alloy.fake();
        /// # assert_ne!(value, Address::ZERO);
        /// ```
        fn dummy_with_rng<R: Rng + ?Sized>(_: &Alloy, rng: &mut R) -> Self {
            <[u8; 20]>::dummy_with_rng(&Faker, rng).into()
        }
    }

    impl Dummy<Alloy> for PrimitiveSignature {
        /// Generate a random [`PrimitiveSignature`] value.
        ///
        /// ```rust
        /// # use fake::Fake;
        /// use thegraph_core::{
        ///     alloy::primitives::PrimitiveSignature,
        ///     fake_impl::alloy::Alloy
        /// };
        ///
        /// let value: PrimitiveSignature = Alloy.fake();
        /// ```
        fn dummy_with_rng<R: Rng + ?Sized>(config: &Alloy, rng: &mut R) -> Self {
            PrimitiveSignature::from_scalars_and_parity(
                B256::dummy_with_rng(config, rng),
                B256::dummy_with_rng(config, rng),
                rng.gen(),
            )
        }
    }
}

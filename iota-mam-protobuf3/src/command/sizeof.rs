//! Implementation of command traits for calculating the size for output buffer in Wrap operation.
use failure::ensure;
use std::iter;

use iota_mam_core::key_encapsulation::ntru;
use iota_mam_core::signature::mss;

use crate::command::*;
use crate::types::*;
use failure::Fallible;

/// Message size counting context.
#[derive(Debug)]
pub struct Context {
    /// The current message size in trits.
    size: usize,
}

impl Context {
    /// Creates a new Context.
    pub fn new() -> Self {
        Self { size: 0 }
    }
    /// Returns calculated message size.
    pub fn get_size(&self) -> usize {
        self.size
    }
}

/// All Trint3 values are encoded with 3 trits.
impl Absorb<&Trint3> for Context {
    fn absorb(&mut self, _trint3: &Trint3) -> Fallible<&mut Self> {
        self.size += 3;
        Ok(self)
    }
}

/// All Trint3 values are encoded with 3 trits.
impl Absorb<Trint3> for Context {
    fn absorb(&mut self, trint3: Trint3) -> Fallible<&mut Self> {
        self.absorb(&trint3)
    }
}

/// Size has var-size encoding.
impl Absorb<&Size> for Context {
    fn absorb(&mut self, size: &Size) -> Fallible<&mut Self> {
        self.size += sizeof_sizet(size.0);
        Ok(self)
    }
}

/// Size has var-size encoding.
impl Absorb<Size> for Context {
    fn absorb(&mut self, size: Size) -> Fallible<&mut Self> {
        self.absorb(&size)
    }
}

/// External values are not encoded in the trinary stream.
impl<'a, T: 'a> Absorb<&'a External<T>> for Context
where
    Self: Absorb<T>,
{
    fn absorb(&mut self, _external: &'a External<T>) -> Fallible<&mut Self> {
        Ok(self)
    }
}

/// External values are not encoded in the trinary stream.
impl<'a, T: 'a> Absorb<External<&'a T>> for Context
where
//Self: Absorb<&'a T>,
{
    fn absorb(&mut self, _external: External<&'a T>) -> Fallible<&mut Self> {
        Ok(self)
    }
}

/// `trytes` has variable size thus the size is encoded before the content trytes.
impl<'a> Absorb<&'a Trytes> for Context {
    fn absorb(&mut self, trytes: &'a Trytes) -> Fallible<&mut Self> {
        ensure!(
            (trytes.0).size() % 3 == 0,
            "Trit size of `trytes` must be a multiple of 3."
        );
        self.size += sizeof_sizet((trytes.0).size() / 3) + (trytes.0).size();
        Ok(self)
    }
}

/// `trytes` has variable size thus the size is encoded before the content trytes.
impl Absorb<Trytes> for Context {
    fn absorb(&mut self, trytes: Trytes) -> Fallible<&mut Self> {
        self.absorb(&trytes)
    }
}

/// `tryte [n]` is fixed-size and is encoded with `3 * n` trits.
impl<'a> Absorb<&'a NTrytes> for Context {
    fn absorb(&mut self, ntrytes: &'a NTrytes) -> Fallible<&mut Self> {
        ensure!(
            (ntrytes.0).size() % 3 == 0,
            "Trit size of `tryte [n]` must be a multiple of 3."
        );
        self.size += (ntrytes.0).size();
        Ok(self)
    }
}

/// `tryte [n]` is fixed-size and is encoded with `3 * n` trits.
impl Absorb<NTrytes> for Context {
    fn absorb(&mut self, ntrytes: NTrytes) -> Fallible<&mut Self> {
        self.absorb(&ntrytes)
    }
}

/// MSS public key has fixed size.
impl<'a> Absorb<&'a mss::PublicKey> for Context {
    fn absorb(&mut self, pk: &'a mss::PublicKey) -> Fallible<&mut Self> {
        ensure!(pk.trits().size() == mss::PK_SIZE);
        self.size += mss::PK_SIZE;
        Ok(self)
    }
}

/// NTRU public key has fixed size.
impl<'a> Absorb<&'a ntru::PublicKey> for Context {
    fn absorb(&mut self, pk: &'a ntru::PublicKey) -> Fallible<&mut Self> {
        ensure!(pk.trits().size() == ntru::PK_SIZE);
        self.size += ntru::PK_SIZE;
        Ok(self)
    }
}

/// External values are not encoded.
impl<'a> Squeeze<&'a External<NTrytes>> for Context {
    fn squeeze(&mut self, _external_ntrytes: &'a External<NTrytes>) -> Fallible<&mut Self> {
        Ok(self)
    }
}

/// External values are not encoded.
impl Squeeze<&External<Mac>> for Context {
    fn squeeze(&mut self, _mac: &External<Mac>) -> Fallible<&mut Self> {
        Ok(self)
    }
}

/// Mac is just like NTrytes.
impl Squeeze<&Mac> for Context {
    fn squeeze(&mut self, mac: &Mac) -> Fallible<&mut Self> {
        ensure!(
            mac.0 % 3 == 0,
            "Trit size of `mac` must be a multiple of 3: {}.",
            mac.0
        );
        self.size += mac.0;
        Ok(self)
    }
}

/// Mac is just like NTrytes.
impl Squeeze<Mac> for Context {
    fn squeeze(&mut self, val: Mac) -> Fallible<&mut Self> {
        self.squeeze(&val)
    }
}

/// Mask Trint3.
impl Mask<&Trint3> for Context {
    fn mask(&mut self, _val: &Trint3) -> Fallible<&mut Self> {
        self.size += 3;
        Ok(self)
    }
}

/// Mask Trint3.
impl Mask<Trint3> for Context {
    fn mask(&mut self, val: Trint3) -> Fallible<&mut Self> {
        self.mask(&val)
    }
}

/// Mask Size.
impl Mask<&Size> for Context {
    fn mask(&mut self, val: &Size) -> Fallible<&mut Self> {
        self.size += sizeof_sizet(val.0);
        Ok(self)
    }
}

/// Mask Size.
impl Mask<Size> for Context {
    fn mask(&mut self, val: Size) -> Fallible<&mut Self> {
        self.mask(&val)
    }
}

/// Mask `n` trytes.
impl Mask<&NTrytes> for Context {
    fn mask(&mut self, val: &NTrytes) -> Fallible<&mut Self> {
        self.size += (val.0).size();
        Ok(self)
    }
}

/// Mask trytes, the size prefixed before the content trytes is also masked.
impl Mask<&Trytes> for Context {
    fn mask(&mut self, trytes: &Trytes) -> Fallible<&mut Self> {
        ensure!(
            (trytes.0).size() % 3 == 0,
            "Trit size of `trytes` must be a multiple of 3: {}.",
            (trytes.0).size()
        );
        let size = Size((trytes.0).size() / 3);
        self.mask(&size)?;
        self.size += (trytes.0).size();
        Ok(self)
    }
}

impl Mask<&ntru::PublicKey> for Context {
    fn mask(&mut self, ntru_pk: &ntru::PublicKey) -> Fallible<&mut Self> {
        ensure!(ntru_pk.trits().size() == ntru::PK_SIZE);
        self.size += ntru::PK_SIZE;
        Ok(self)
    }
}

impl Mask<&mss::PublicKey> for Context {
    fn mask(&mut self, mss_pk: &mss::PublicKey) -> Fallible<&mut Self> {
        ensure!(mss_pk.trits().size() == mss::PK_SIZE);
        self.size += mss::PK_SIZE;
        Ok(self)
    }
}

/// Skipped values are just encoded.
/// All Trint3 values are encoded with 3 trits.
impl Skip<&Trint3> for Context {
    fn skip(&mut self, _trint3: &Trint3) -> Fallible<&mut Self> {
        self.size += 3;
        Ok(self)
    }
}

/// All Trint3 values are encoded with 3 trits.
impl Skip<Trint3> for Context {
    fn skip(&mut self, trint3: Trint3) -> Fallible<&mut Self> {
        self.skip(&trint3)
    }
}

/// Size has var-size encoding.
impl Skip<&Size> for Context {
    fn skip(&mut self, size: &Size) -> Fallible<&mut Self> {
        self.size += sizeof_sizet(size.0);
        Ok(self)
    }
}

/// Size has var-size encoding.
impl Skip<Size> for Context {
    fn skip(&mut self, size: Size) -> Fallible<&mut Self> {
        self.skip(&size)
    }
}

/// `trytes` is encoded with `sizeof_sizet(n) + 3 * n` trits.
impl<'a> Skip<&'a Trytes> for Context {
    fn skip(&mut self, trytes: &'a Trytes) -> Fallible<&mut Self> {
        ensure!(
            (trytes.0).size() % 3 == 0,
            "Trit size of `trytes` must be a multiple of 3."
        );
        self.size += sizeof_sizet((trytes.0).size() / 3) + (trytes.0).size();
        Ok(self)
    }
}

/// `trytes` is encoded with `sizeof_sizet(n) + 3 * n` trits.
impl Skip<Trytes> for Context {
    fn skip(&mut self, trytes: Trytes) -> Fallible<&mut Self> {
        self.skip(&trytes)
    }
}

/// `tryte [n]` is encoded with `3 * n` trits.
impl<'a> Skip<&'a NTrytes> for Context {
    fn skip(&mut self, ntrytes: &'a NTrytes) -> Fallible<&mut Self> {
        ensure!(
            (ntrytes.0).size() % 3 == 0,
            "Trit size of `tryte [n]` must be a multiple of 3."
        );
        self.size += (ntrytes.0).size();
        Ok(self)
    }
}

/// `tryte [n]` is encoded with `3 * n` trits.
impl Skip<NTrytes> for Context {
    fn skip(&mut self, ntrytes: NTrytes) -> Fallible<&mut Self> {
        self.skip(&ntrytes)
    }
}

/// Commit costs nothing in the trinary stream.
impl Commit for Context {
    fn commit(&mut self) -> Fallible<&mut Self> {
        Ok(self)
    }
}

/// Signature size depends on Merkle tree height.
impl Mssig<&mss::PrivateKey, &External<NTrytes>> for Context {
    fn mssig(&mut self, sk: &mss::PrivateKey, hash: &External<NTrytes>) -> Fallible<&mut Self> {
        ensure!(
            mss::HASH_SIZE == ((hash.0).0).size(),
            "Trit size of `external tryte hash[n]` to be signed with MSS must be equal {} trits.",
            mss::HASH_SIZE
        );
        ensure!(sk.private_keys_left() > 0, "All WOTS private keys in MSS Merkle tree have been exhausted, nothing to sign hash with.");
        self.size += mss::sig_size(sk.height());
        Ok(self)
    }
}

impl Mssig<&mss::PrivateKey, &External<Mac>> for Context {
    fn mssig(&mut self, sk: &mss::PrivateKey, hash: &External<Mac>) -> Fallible<&mut Self> {
        ensure!(
            mss::HASH_SIZE == (hash.0).0,
            "Trit size of `external tryte hash[n]` to be signed with MSS must be equal {} trits.",
            mss::HASH_SIZE
        );
        ensure!(sk.private_keys_left() > 0, "All WOTS private keys in MSS Merkle tree have been exhausted, nothing to sign hash with.");
        self.size += mss::sig_size(sk.height());
        Ok(self)
    }
}

impl Mssig<&mss::PrivateKey, MssHashSig> for Context {
    fn mssig(&mut self, sk: &mss::PrivateKey, _hash: MssHashSig) -> Fallible<&mut Self> {
        // Squeeze external and commit cost nothing in the stream.
        self.size += mss::sig_size(sk.height());
        Ok(self)
    }
}

/// Sizeof encapsulated secret is fixed.
impl Ntrukem<&ntru::PublicKey, &NTrytes> for Context {
    fn ntrukem(&mut self, _key: &ntru::PublicKey, secret: &NTrytes) -> Fallible<&mut Self> {
        //TODO: Ensure key is valid.
        ensure!(ntru::KEY_SIZE == (secret.0).size(), "Trit size of `external tryte secret[n]` to be encapsulated with NTRU must be equal {} trits.", ntru::KEY_SIZE);
        self.size += ntru::EKEY_SIZE;
        Ok(self)
    }
}

/// Forks cost nothing in the trinary stream.
impl<F> Fork<F> for Context
where
    F: for<'a> FnMut(&'a mut Self) -> Fallible<&'a mut Self>,
{
    fn fork(&mut self, mut cont: F) -> Fallible<&mut Self> {
        cont(self)
    }
}

/// Repeated modifier. The actual number of repetitions must be wrapped
/// (absorbed/masked/skipped) explicitly.
impl<I, F> Repeated<I, F> for Context
where
    I: iter::Iterator,
    F: for<'a> FnMut(&'a mut Self, <I as iter::Iterator>::Item) -> Fallible<&'a mut Self>,
{
    fn repeated(&mut self, values_iter: I, mut value_handle: F) -> Fallible<&mut Self> {
        values_iter.fold(Ok(self), |rctx, item| -> Fallible<&mut Self> {
            match rctx {
                Ok(ctx) => value_handle(ctx, item),
                Err(e) => Err(e),
            }
        })
    }
}

/*
/// It's the size of the link.
impl<'a, L: Link> Absorb<&'a L> for Context {
    fn absorb(&mut self, link: &'a L) -> Fallible<&mut Self> {
        self.size += link.size();
        Ok(self)
    }
}
/// It's the size of the link.
impl<'a, L: Link, S: LinkStore<L>> Join<&'a L, &'a S> for Context {
    fn join(&mut self, store: &'a S, link: &'a L) -> Fallible<&mut Self> {
        self.size += link.size();
        Ok(self)
    }
}
*/

/// It's the size of the link.
impl<'a, T: 'a + AbsorbFallback> Absorb<&'a T> for Context {
    fn absorb(&mut self, val: &'a T) -> Fallible<&mut Self> {
        val.sizeof_absorb(self)?;
        Ok(self)
    }
}
/*
impl<'a, T: 'a + AbsorbExternalFallback> Absorb<External<&'a T>> for Context {
    fn absorb(&mut self, val: External<&'a T>) -> Fallible<&mut Self> {
        (val.0).sizeof_absorb_external(self)?;
        Ok(self)
    }
}
*/
impl<'a, T: 'a + SkipFallback> Skip<&'a T> for Context {
    fn skip(&mut self, val: &'a T) -> Fallible<&mut Self> {
        val.sizeof_skip(self)?;
        Ok(self)
    }
}

/// It's the size of the link.
impl<'a, L: SkipFallback, S: LinkStore<L>> Join<&'a L, &'a S> for Context {
    fn join(&mut self, _store: &'a S, link: &'a L) -> Fallible<&mut Self> {
        link.sizeof_skip(self)?;
        Ok(self)
    }
}

/*
impl<'a, L, S: LinkStore<L>> Join<&'a L, &'a S> for Context where
    Self: Skip<&'a L>
{
    fn join(&mut self, _store: &'a S, link: &'a L) -> Fallible<&mut Self> {
        self.skip(link)
    }
}
 */

impl Dump for Context {
    fn dump<'a>(&mut self, args: std::fmt::Arguments<'a>) -> Fallible<&mut Self> {
        println!("{}: size=[{}]", args, self.size);
        Ok(self)
    }
}

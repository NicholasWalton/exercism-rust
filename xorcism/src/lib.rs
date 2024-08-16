use std::borrow::Borrow;
use std::iter::Cycle;
use std::slice::Iter;

pub trait Captures<'a> {}
impl<'a, T> Captures<'a> for T {}

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: Cycle<Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key>(key: &'a Key) -> Self
    where
        Key: ?Sized + AsRef<[u8]>,
    {
        Xorcism {
            key: key.as_ref().iter().cycle(),
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// # Panics
    /// Would panic if key is exhausted, but key is a Cycle
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for byte in data {
            *byte ^= self.key.next().expect("key is a Cycle");
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl Iterator<Item = u8> + 'b + Captures<'a>
    where
        'a: 'b,
        Data: 'b,
        Data: IntoIterator<Item: Borrow<u8>>,
    {
        data.into_iter()
            .zip(self.key.by_ref())
            .map(|(byte, key_byte)| byte.borrow() ^ key_byte)
    }
}

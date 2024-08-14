use std::ops::{BitAnd, BitXor};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    // This field is just to suppress compiler complaints;
    // feel free to delete it at any point.
    // _phantom: std::marker::PhantomData<&'a u8>,
    key: &'a [u8],
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key>(key: Key) -> Xorcism<'a>
    where
        &'a [u8]: From<Key>,
    {
        Xorcism { key: key.into() }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        let mut key = self.key.iter().cycle();
        for byte in data {
            *byte = *byte ^ key.next().unwrap();
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8> {
        let mut key = self.key.iter().cycle();
        data.into_iter().map(move |byte| byte ^ key.next().unwrap())
    }
}

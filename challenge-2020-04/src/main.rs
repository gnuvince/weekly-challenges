use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

#[derive(Debug)]
struct BloomFilter {
    bitmap: Bitmap,
    salts: Vec<String>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BloomResult { Maybe, No }

const N: usize = 1<<15;

impl BloomFilter {
    fn new() -> BloomFilter {
        return BloomFilter {
            bitmap: Bitmap::new(N),
            salts: vec![
                "FOO".to_owned(),
                "BAR".to_owned(),
                "BAZ".to_owned()
            ]
        };
    }

    fn hash(key: &str, salt: &str) -> usize {
        let mut h = DefaultHasher::new();
        h.write(key.as_bytes());
        h.write(salt.as_bytes());
        let x = h.finish();
        return (x as usize % N);
    }

    fn insert(&mut self, key: &str) {
        for salt in self.salts.iter() {
            self.bitmap.set(Self::hash(key, salt));
        }
    }

    fn contains(&self, key: &str) -> BloomResult {
        for salt in self.salts.iter() {
            if !self.bitmap.get(Self::hash(key, salt)).unwrap() {
                return BloomResult::No;
            }
        }
        return BloomResult::Maybe;
    }
}

#[test]
fn bloom_filter() {
    let mut bf = BloomFilter::new();

    assert_eq!(BloomResult::No, bf.contains("remi"));
    assert_eq!(BloomResult::No, bf.contains("mina"));

    bf.insert("mina");

    assert_eq!(BloomResult::No, bf.contains("remi"));
    assert_eq!(BloomResult::Maybe, bf.contains("mina"));
}



#[derive(Debug)]
struct Bitmap {
    num_entries: usize,
    bits: Vec<u64>,
}

impl Bitmap {
    /// Creates a bitmap with room for `num_entries`,
    /// from 0 to `num_entries` (exclusive).
    pub fn new(num_entries: usize) -> Self {
        let num_buckets = {
            let n = num_entries / 64;
            let d = if num_entries % 64 == 0 { 0 } else { 1 };
            n + d
        };
        Bitmap {
            num_entries: num_entries,
            bits: vec![0; num_buckets],
        }
    }

    /// Returns the number of set bits.
    pub fn len(&self) -> usize {
        self.bits.iter().map(|x| x.count_ones() as usize).sum()
    }

    fn pos(&self, bit: usize) -> Option<(usize, u64)> {
        if bit >= self.num_entries {
            return None;
        }

        let bucket: usize = bit / 64;
        let offset: u64 = (bit as u64) % 64;
        return Some((bucket, offset));
    }

    /// Turns on `bit`; returns the previous value of the bit.
    /// Returns `None` if the bit is out of range.
    pub fn set(&mut self, bit: usize) -> Option<bool> {
        let (bucket, offset) = self.pos(bit)?;
        let prev = (self.bits[bucket] >> offset & 1) != 0;
        self.bits[bucket] |= 1 << offset;
        return Some(prev);
    }

    /// Turns off `bit`; returns the previous value of the bit.
    /// Returns `None` if the bit is out of range.
    pub fn clear(&mut self, bit: usize) -> Option<bool> {
        let (bucket, offset) = self.pos(bit)?;
        let prev = (self.bits[bucket] >> offset & 1) != 0;
        self.bits[bucket] &= !(1 << offset);
        return Some(prev);
    }

    /// Gets the value of `bit`.
    /// Returns `None` if the bit is out of range.
    pub fn get(&self, bit: usize) -> Option<bool> {
        let (bucket, offset) = self.pos(bit)?;
        return Some((self.bits[bucket] >> offset & 1) != 0);
    }

    pub fn union(&mut self, other: &Self) -> Option<()> {
        if self.num_entries != other.num_entries {
            return None;
        }
        for (x, y) in self.bits.iter_mut().zip(other.bits.iter()) {
            *x |= *y;
        }
        return Some(());
    }

    pub fn inter(&mut self, other: &Self) -> Option<()> {
        if self.num_entries != other.num_entries {
            return None;
        }
        for (x, y) in self.bits.iter_mut().zip(other.bits.iter()) {
            *x &= *y;
        }
        return Some(());
    }

}


#[test]
fn test_capacity() {
    let b = Bitmap::new(0);
    assert_eq!(b.bits.len(), 0);

    let b = Bitmap::new(1);
    assert_eq!(b.bits.len(), 1);

    let b = Bitmap::new(63);
    assert_eq!(b.bits.len(), 1);

    let b = Bitmap::new(64);
    assert_eq!(b.bits.len(), 1);

    let b = Bitmap::new(65);
    assert_eq!(b.bits.len(), 2);
}

#[test]
fn test_pos() {
    let b = Bitmap::new(129); // 3 buckets
    assert_eq!(None, b.pos(200));
    assert_eq!(None, b.pos(129));

    assert_eq!(Some((0, 0)), b.pos(0));
    assert_eq!(Some((0, 1)), b.pos(1));
    assert_eq!(Some((0, 2)), b.pos(2));
    assert_eq!(Some((0, 63)), b.pos(63));

    assert_eq!(Some((1, 0)), b.pos(64));
    assert_eq!(Some((1, 1)), b.pos(65));
    assert_eq!(Some((1, 2)), b.pos(66));
    assert_eq!(Some((1, 63)), b.pos(127));

    assert_eq!(Some((2, 0)), b.pos(128));
}

#[test]
fn test_get_set_clear() {
    let mut b = Bitmap::new(1);

    assert_eq!(None, b.get(200));
    assert_eq!(None, b.set(200));
    assert_eq!(None, b.clear(200));

    assert_eq!(Some(false), b.get(0));
    assert_eq!(Some(false), b.set(0)); // side-effect
    assert_eq!(1, b.len());
    assert_eq!(Some(true), b.get(0));
    assert_eq!(Some(true), b.clear(0)); // side-effect
    assert_eq!(0, b.len());
    assert_eq!(Some(false), b.get(0));
}

#[test]
fn test_len() {
    let mut b = Bitmap::new(128);
    assert_eq!(0, b.len());

    b.set(0);
    b.set(64);
    assert_eq!(2, b.len());

    b.clear(0);
    assert_eq!(1, b.len());

    b.clear(64);
    assert_eq!(0, b.len());
}

#[test]
fn test_union() {
    let mut b1 = Bitmap::new(4);
    let mut b2 = Bitmap::new(4);
    let b3 = Bitmap::new(5);

    assert_eq!(None, b1.union(&b3));

    b1.set(0);
    b2.set(1);
    assert_eq!(Some(()), b1.union(&b2));
    assert_eq!(Some(true), b1.get(0));
    assert_eq!(Some(true), b1.get(1));
}

#[test]
fn test_inter() {
    let mut b1 = Bitmap::new(4);
    let mut b2 = Bitmap::new(4);
    let b3 = Bitmap::new(5);

    assert_eq!(None, b1.inter(&b3));

    b1.set(0);
    b1.set(1);
    b2.set(1);
    assert_eq!(Some(()), b1.inter(&b2));
    assert_eq!(Some(false), b1.get(0));
    assert_eq!(Some(true), b1.get(1));
}


fn main() {
    let mut b = Bitmap::new(200);
    b.set(1);
    b.set(199);
    println!("{}", b.len());
    b.clear(1);
    println!("{:?}", b.get(1));
}

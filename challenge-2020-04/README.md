Weekly challenge — 2020-04
==========================

  In this week's challenge, you have to implement a [bloom filter].
  A bloom filter supports two basic set operations: `insert` and `contains`.
  A bloom filter is a *probabilistic* data structure: the `contains`
  operation returns `maybe/no` instead of `yes/no`. Here's a sample
  execution of a bloom filter.


    bf = new BloomFilter();

    bf.contains("remi"); // "no"
    bf.contains("mina"); // "no"

    bf.insert("mina");

    bf.contains("remi"); // "no"
    bf.contains("mina"); // "maybe"


  Your bloom filter will require a bitmap data structure: steal
  the one from two weeks ago!

  To insert a key into the bloom filter, hash the key with three
  different hashing functions and turn on the appropriate bits in the
  bloom filter. (You'll want to use the modulo operation to make
  sure the bitmap keys fit.) Rather than using different hashing
  functions, you can simply [salt] your keys with a three distinct
  prefix strings. For example:


    bit1 = hash("FOO" + my_key)
    bit2 = hash("BAR" + my_key)
    bit3 = hash("BAZ" + my_key)


  To check if a bloom filter contains a key, hash the key three times
  using the same salt strings and query the bitmap. If *any* of the
  bits are zero, the key was never inserted and you return "no". If
  all the bits are one, it's *possible* that the key was inserted and
  you return "maybe". (The answer is "maybe" because the bits *could*
  have been turned on by the insertion of other keys.)

  Here is the API for your implementation:

  - `new()`: create a new bloom filter;
  - `insert(string)`: insert a new key in the bloom filter;
  - `contains(string) -> maybe|no`: return `no` if the key is not in
    the bloom filter and `maybe` otherwise.

  Good luck!


[bloom filter]: https://en.wikipedia.org/wiki/Bloom_filter
[salt]: https://en.wikipedia.org/wiki/Salt_(cryptography)

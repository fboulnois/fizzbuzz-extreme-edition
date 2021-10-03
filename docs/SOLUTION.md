> Note: this would have been easier to write in C/C++ since you don't need to
deal with the borrow checker and you can silently convert between primitive
types.

The implementation is based on the fact that the divisibility of FizzBuzz
numbers can be stored directly as bits of state in a large number. Since
FizzBuzz is usually from 1..100, a 128-bit number (the nearest power of 2) can
be used to set all the bits where the numbers are divisible by 3:

```
index: bit
1: 0
2: 0
3: 1
4: 0
5: 0
6: 1
7: 0
...
```

Although Rust supports 128-bit integers natively, this integer is instead split
into low and high 64-bit values which are easier to manipulate. The low value
value represents indices 1..64, and the high value represents indices 65..100.

In binary (little-endian), these two 64-bit values look like this:

```
low3:  0b0100100100100100100100100100100100100100100100100100100100100100
high3: 0b0000000000000000000000000000010010010010010010010010010010010010
```

Similarly, with another two 64-bit values we can store the indices where the
numbers are divisible by 5:

```
low5:  0b0000100001000010000100001000010000100001000010000100001000010000
high5: 0b0000000000000000000000000000100001000010000100001000010000100001
```

Notice that the high bits in both values have leading zeroes. Since only 100
bits in the 128-bit integer are used, there are 28 bits remaining in the high
values to encode the words `Fizz` and `Buzz`. To compress the amount of space
each letter will take in a 28-bit integer, the letters are stored as an ASCII
offset from the letter `A`. `F` has an offset of 5, whereas `z` has an offset
of 57. Also, `z` appears consecutively in both `Fizz` and `Buzz`, so to save
space it is encoded once per string. All the ASCII offsets are smaller than 64,
so it is technically possible to only use 6 bits per letter (2^6 = 64). However,
there is enough space to encode `Fiz` and `Buz` using 8-bit encoding instead (8
bits * 3 letters = 24 bits total which is less than the 28 bits that are
available), and this makes bit operations straightforward.

In binary, the strings `Fiz` and `Buz` are encoded as follows:

```
  [---F--][---i--][---z--]
0b000001010010100000111001

  [---B--][---u--][---z--]
0b000000010011010000111001
```

To ensure that the most significant bit (MSB) of the high value includes one of
these strings, the string needs to be shifted left by 40 (= 64 bits - 24 bits)
and logically `OR`'d with the high value. The expression to obtain the final
value corresponds to `final = (str << 40) | high`.

Result of the binary expression encoding the strings into the high values:

```
   0b0000000000000000000000000000010010010010010010010010010010010010  # high3
|  0b000001010010100000111001                                          # 'fiz'
=  0b0000010100101000001110010000010010010010010010010010010010010010

   0b0000000000000000000000000000100001000010000100001000010000100001  # high5
|  0b000000010011010000111001                                          # 'buz'
=  0b0000000100110100001110010000100001000010000100001000010000100001
```

Each of the low and high binary values can now be converted to their decimal
equivalent to hide the underlying bit patterns:

```
0b0100100100100100100100100100100100100100100100100100100100100100 = 5270498306774157604
0b0000010100101000001110010000010010010010010010010010010010010010 = 371609661054985362
0b0000100001000010000100001000010000100001000010000100001000010000 = 595056260442243600  
0b0000000100110100001110010000100001000010000100001000010000100001 = 86757000457782305
```

These four decimal values are added to a vector of `u64`. To obfuscate that some
of the operations of multiples of 64, a pair of numbers is found such that
(5270498306774157604 + N)/M = 64. The constant M, selected as `82351536043346213`,
is also added to the vector of `u64`.

Thus the final vector includes the following values:

```rust
let v = vec![
    5270498306774157604,
    371609661054985362,
    595056260442243600,
    86757000457782305,
    82351536043346213,
];
```

At this point, the implementation to decode all the values is straightforward.

First, the bits are pulled out of the high values to create the strings `Fizz`,
`Buzz`, and `FizzBuzz`, and stored into a vector.

Next, the code loops over all the low bits and part of the high bits and
extracts whether the index is divisible by 3 or by 5. With a `match` statement,
both of these expressions can be tested simultaneously. If both bits are set,
then print `FizzBuzz`, otherwise if either bit is set, print `Fizz` or `Buzz` as
appropriate. If neither are set, print the index instead.

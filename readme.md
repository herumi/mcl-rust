# mcl for Rust

This is a wrapper library of [mcl](https://github.com/herumi/mcl/),
which is a portable and fast pairing-based cryptography library.

# Test

```
git clone https://github.com/herumi/mcl
cd mcl
make lib/libmcl.a lib/libmclbn384_256.a
env RUSTFLAGS="-L<directory of mcl/lib>" cargo test
```

# License

modified new BSD License
http://opensource.org/licenses/BSD-3-Clause

# Author

光成滋生 MITSUNARI Shigeo(herumi@nifty.com)

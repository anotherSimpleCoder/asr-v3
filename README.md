# asr-v2

Documentation
-------------

asr-v3 is the third version of asr, which is actually just a rewrite of asr-v2: Instead of using SDL and C++,
this version got now changed to Rust and is working with the speedy2d framework. Not much has changed algorithm wise.
So the way asr functions is pretty much the same.


Get started
-----------

Thanks to Rust's awesome cargo package service you don't have to worry about any dependencies, since these will be automatically
fetched by cargo.

Of course you have to include asr inside your code file

```rust
mod asr;

use asr::*;
```

That's it! You are now able to use the asr library. The main.rs also includes a simple example on how
to use the library.

Use asr in your project
------------------------

Now that you have included asr into your code file you just need to create an asr instance and you're good to go!

```rust
let a: asr = ASR::new(<width>, <height>);
```

You can call asr functions with:
```rust
a.<asr function>;
```

Again a small example is given in ```main.rs```.

asr functions
-------------

Here is an overview of all the asr functions:
    * ```new(width: i64, height: i64)```                    Creates a new asr instance.
    * ```assign(color: ASRColor, position: ASRVector)```    Assign a color to a pixel in the pixel map.
    * ```render()```                                        Renders the final image.


Well....why Rust?
-----------------

I have decided to rewrite ASR in Rust, because of it's memory safety and still having a great low-level access, which gives you
small rendering times.
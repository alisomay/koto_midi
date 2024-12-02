# koto-midi

## Introduction

`koto-midi` is a module for working with midi messages in koto scripts.

This crate does not have big claims to be "the" midi module for `koto`.

I've done it in the past and it is useful to me for some scenarios. I'm sharing it in case it is useful to you.

For the summary of the api please run `just tests` or `cargo watch -x "test --test test_runner"` and check the `stdout`.

## Embedding

In the application which embeds `koto`,

```rust

// ..

let mut koto = Koto::new();
let mut prelude = koto.prelude();
prelude.insert("midi", koto_midi::make_module());

// ..

```

In the koto script which `koto-midi` wants the be used in, it could be brought to scope by,

```coffee

import midi
# ..

```

| For more on using and embedding `koto` in your rust applications please visit [koto repository](https://github.com/koto-lang/koto).

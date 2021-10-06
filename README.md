# koto_midi

## Introduction

`koto_midi` is a module for working with midi messages in koto scripts.

For the summary of the api please run `just tests` or `cargo watch -x "test --test test_runner"` and check the `stdout`.

## Embedding

In the application which embeds `koto`,

```rust

// ..

let mut koto = Koto::new();
let mut prelude = koto.prelude();
prelude.add_map("midi", koto_midi::make_module());

// ..

```

In the koto script which `koto_midi` wants the be used in, it could be brought to scope by,

```coffee

import midi
# ..

```

| For more on using and embedding `koto` in your rust applications please visit [koto repository](https://github.com/koto-lang/koto).

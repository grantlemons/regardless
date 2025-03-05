# Regardless (an anyhow clone)

Regardless is a blatant clone of anyhow, but without nearly as many features.

It supports context and `From<Error>` but the output isn't very helpful and it is missing literally everything else that anyhow supports.
I do not recommend using this crate for anything at all, and I'm not going to maintain it beyond what I need.
Just use anyhow.

## Rationale

I wrote this for an assignment where I'm not allowed to use any external crates, ergo make bad versions of my favorite crates.

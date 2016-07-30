# Sooty

![sooty puppet](sooty.png)

An extremely simple lisp interpreter, exploring GC implementation.

## License

GPLv3. Sooty image
[is a promotional picture from the TV show 'Sooty and Sweep'](https://en.wikipedia.org/wiki/File:Sooty2011.png)

## Related Work

* [rust-gc](https://github.com/Manishearth/rust-gc)
* [Graphs and arena allocation](Graphs and arena allocation)

## Syntactic Choices

Where possible, Sooty will use `[]` for literal lists,
and `()` for lists to evaluate. This should minimise quoting, and
distinguish lists for data from unevaluated syntax.

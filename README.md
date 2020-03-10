# find-reify-usage

This project finds reify usages in Clojure files provided on the command line.
It is implemented using [tree-sitter-clojure](https://github.com/sogaiu/tree-sitter-clojure) and Rust.

## Build

Execute `script/build`. You will need `npm` and `cargo`.
This will create a `find-reify-usage` binary in `target/release`.

## Usage

```
$ time find ~/git/clojure/src/clj -name "*.clj" | xargs ./find-reify-usage
clojure.core.protocols/CollReduce
clojure.core.protocols/CollReduce
clojure.lang.IDeref
clojure.lang.IDeref
java.util.Iterator
java.util.ListIterator
clojure.core.ArrayManager
find ~/git/clojure/src/clj -name "*.clj"  0.00s user 0.00s system 63% cpu 0.006 total
xargs ./find-reify-usage  0.12s user 0.02s system 89% cpu 0.153 total
```

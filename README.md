# find-reify-usage

This project finds reify usages in Clojure files you provide to it via the command line.
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
```

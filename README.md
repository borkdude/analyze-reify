# find-reify-usage

This project finds reify usages in Clojure files you provide to it via the command line.

## Build

Execute `script/build`. You will need `npm` and `cargo`.
This will create a `find-reify-usage` binary in `target/release`.

## Usage

```
find ~/git/clojure/src/clj -name "*.clj" | xargs ./find-reify-usage
```

## License

See LICENSE.

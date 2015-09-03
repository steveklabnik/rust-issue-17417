```bash
$ cargo run
   Compiling issue-17417 v0.1.0 (file:///home/steve/tmp/rust-issue-17417)
     Running `target/debug/issue-17417`
new pointer is 0x2a
$ cargo run --features fail
   Compiling issue-17417 v0.1.0 (file:///home/steve/tmp/rust-issue-17417)
     Running `target/debug/issue-17417`
new pointer is 0x0
```

If #17417 is fixed, the `fail` case should print some kind of warning.

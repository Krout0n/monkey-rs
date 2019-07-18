# monkey-rs
Writing A Monkey Interpreter In Rust. It is a subset language of Monkey-lang.

See: https://interpreterbook.com/

```
$ cargo run --bin run-repl
➜  monkey-rs git:(master) ✗ cargo run --bin run-repl
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/run-repl`
Yo this is a Monkey programming language REPL!
Feel free to type some statement!
>> let y = 10;
Integer(10)
>> y + 1;
Integer(11)
>> 
```

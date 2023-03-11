# grepharder

Common Linux system calls implemented in Rust because why not?

## grepharder

```
grep implemented in Rust.

Usage: grepharder --pattern <PATTERN> --filename <FILE>

Options:
  -p, --pattern <PATTERN>  Pattern to search provided file argument for (case-sensitive).
  -f, --filename <FILE>    Filename to search for pattern.
  -h, --help               Print help
  -V, --version            Print version
```

### Sample Output

```
toeknee@macaroni-pc:~/git-repos/grephardest/grepharder$ cargo run -- -p "Lorem" -f ../examples/input.txt 
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/grepharder -p Lorem -f ../examples/input.txt`
1: Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
```

## catharder

```
cat implemented in Rust.

Usage: catharder --filename <FILE>

Options:
  -f, --filename <FILE>  File to display.
  -h, --help             Print help
  -V, --version          Print version
```

### Sample Output

```
toeknee@macaroni-pc:~/git-repos/grephardest/catharder$ cargo run -- -f ../examples/input.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/catharder -f ../examples/input.txt`
1: Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
2: Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. 
3: Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. 
4: Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum
```


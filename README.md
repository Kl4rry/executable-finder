# executable-finder
A rust library for finding installed executables

```rust
use executable_finder::executables;

fn main() {
    let executables = executables().unwrap();
    for executable in executables {
        println!("{:?}", executable);
    }
}
```

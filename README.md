# GenFrom Macro

You can generate ```From<MyStructA> for MyStructB``` trait:

```rust
    #[derive(Debug)]
    struct MyStructB {
        v1: u64,
        v2: u64,
        v3: u64,
    }

    #[derive(Debug, GenFrom)]
    #[into(MyStructB)]
    struct MyStructA {
        #[into(name="v1")]
        short_tag: u16,
        #[into(name="v2")]
        value: u8,
        #[into(name="v3")]
        data: u64,
    }
```

Then just do something like:
```rust
let b: MyStructB = a.into();
```

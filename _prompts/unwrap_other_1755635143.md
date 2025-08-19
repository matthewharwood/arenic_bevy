# unwrap_other

**Usecase:** unwrap vs ? vs other

**Created:** 1755635143

---


So when using the `Arena::new(2)` construc

```rust
impl Arena {
    const MAX_ARENAS: u8 = 9;

    /// Creates new Arena if value is valid (0-8)
    #[must_use]
    pub fn new(idx: u8) -> Option<Self> {
        (idx < Self::MAX_ARENAS).then(|| Self(idx))
    }

    #[must_use]
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}
```


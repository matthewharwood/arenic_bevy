# unwrap_other

**Usecase:** unwrap vs ? vs other

**Created:** 1755635143

---

@agent-blow-game-engineer I need you help
So when using the `Arena::new(2)` what is a better way to handle the error?

1. the ? operator?
2. Explict expects?
3. unwrap?
4. Result?
5. other?

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

Moreover, if we were to make a rule about this for @agent-blow-game-engineer what would it be?
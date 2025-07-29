---
description: "Generate Bevy ECS systems and Rust code following project conventions"
allowed-tools: [ "Read", "Write", "Edit", "MultiEdit", "Grep", "Glob", "LS" ]
argument-hint: "<component/system/trait name> [description]"
---

# Bevy ECS & Rust Code Generator

Generate clean, maintainable code following this project's conventions.

## Context

Project emphasizes:

- Clean Bevy ECS patterns with Required Components (0.16+)
- Minimal, focused traits with const associated values
- Comprehensive unit tests with descriptive assertions
- Clear inline documentation
- No unnecessary comments in implementation
  b

## Code Style Guidelines

### Component Pattern

```rust
#[derive(Component, Debug, Clone)]
#[require(OtherComponent)]  // Use Required Components pattern
pub struct MyComponent {
    field: Type,
}

impl MyComponent {
    pub fn new() -> Self { ... }

    #[inline]
    pub fn getter(&self) -> Type { ... }
}
```

### Trait Pattern

```rust
pub trait MyTrait {
    const CONSTANT: Type = value;

    fn method(&self) -> Type;
}
```

### System Pattern

```rust
pub fn my_system(
    query: Query<&Component, With<Filter>>,
    resource: Res<Resource>,
) {
    for component in &query {
        // Logic here
    }
}
```

### Testing Philosophy

- Test constants and configuration values
- Test public API behavior
- Use descriptive test names and assertion messages
- Group related tests in modules
- Validate edge cases and invariants

Example test pattern:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptive_name() {
        assert_eq!(
            actual, expected,
            "Clear explanation of what should happen"
        );
    }
}
```

## Task

Generate $ARGUMENTS following these patterns. Include:

1. Component/System/Trait implementation
2. Required helper functions
3. Comprehensive unit tests
4. Brief module documentation

Focus on clarity, maintainability, and Bevy best practices.
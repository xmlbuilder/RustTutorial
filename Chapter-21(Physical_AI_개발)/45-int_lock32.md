## 소스 코드
```rust
#[derive(Debug)]
pub struct IntLock32 {
    locked: bool,
}

impl IntLock32 {
    pub const UNLOCKED: i32 = 0;
    pub const DEFAULT_LOCKED: i32 = 1;

    pub fn new() -> Self {
        Self { locked: false }
    }

    pub fn is_locked_flag(&self) -> bool {
        self.locked
    }

    pub fn get_default_lock(&mut self) -> bool {
        if self.locked {
            false
        } else {
            self.locked = true;
            true
        }
    }

    pub fn return_default_lock(&mut self) -> bool {
        if self.locked {
            self.locked = false;
            true
        } else {
            false
        }
    }
}
```

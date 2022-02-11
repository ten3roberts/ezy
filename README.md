# ezy

## Ezy

Rust eazy to use tweening library

Tweens are created separate from instantiation, which fixes the *jump* at the
start if the tween start does not match the current value.

The trait [Lerp](crate::Lerp) describes how a value is interpolated.

If feature `glam` is enabled, `Lerp` is available for Vec and Quat (using Slerp)

### Example

```rust
use std::time::Duration;
use ezy::*;
let mut val = 0.0;

// Go to `1.0` under 2 seconds
let tween = Tween::new(1.0, Duration::from_secs(2))

let mut anim = tween.start(val);

// Update tween
anim.progress(&mut val, Duration::from_millis(200))
```

License: MIT

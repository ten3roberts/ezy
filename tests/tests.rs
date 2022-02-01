use std::time::Duration;

use ez::*;

#[test]
fn float() {
    let mut val = 0.0;

    // Define a tween
    let tween = Tween::new(1.0, Duration::from_secs(2));
    let mut anim = tween.start(val);
    for _ in 0..5 {
        anim.progress(&mut val, Duration::from_secs_f32(0.2))
    }
    assert_eq!(val, 0.5);
}

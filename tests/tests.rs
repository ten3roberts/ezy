use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use ezy::*;

#[test]
fn float() {
    fn generate_tween() -> Tween<f32> {
        Tween::new(1.0, Duration::from_secs(2))
    }
    let mut val = 0.0;

    // Define a tween
    let tween = generate_tween();

    let mut anim = tween.start(val);
    for _ in 0..5 {
        anim.progress(&mut val, Duration::from_secs_f32(0.2))
    }

    assert_eq!(val, 0.5);
}

#[test]
fn thread() {
    fn generate_tween() -> Tween<f32> {
        Tween::new(1.0, Duration::from_secs(2))
    }
    let val = Arc::new(Mutex::new(0.0));

    // Define a tween
    let tween = generate_tween();
    let thread_val = val.clone();

    let thread = thread::spawn(move || {
        let mut anim = tween.start(*thread_val.lock().unwrap());
        for _ in 0..5 {
            anim.progress(
                &mut thread_val.lock().unwrap(),
                Duration::from_secs_f32(0.2),
            )
        }
    });

    thread.join().unwrap();

    assert_eq!(*val.lock().unwrap(), 0.5);
}

#[test]
#[cfg(feature = "glam")]
fn tuple() {
    use glam::vec3;
    let mut val = (0.0, vec3(0.0, 1.0, 0.0));
    let tween = Tween::new((5.0, vec3(0.0, 2.0, -10.0)), Duration::from_secs(5));
    let mut anim = tween.start(val);
    for _ in 0..10 {
        anim.progress((&mut val.0, &mut val.1), Duration::from_millis(100));
    }

    assert_eq!(val, (1.0, vec3(0.0, 1.2, -2.0)));
}

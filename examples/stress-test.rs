use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::spawn;

use iana_time_zone::{GetTimezoneError, Timezone};

const THREADS: usize = 10;
const ITERATIONS: usize = 100_000;

static COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() -> Result<(), GetTimezoneError> {
    let mut threads = Vec::with_capacity(THREADS);
    for _ in 0..THREADS {
        threads.push(spawn(|| {
            for _ in 0..ITERATIONS {
                let _ = Timezone::system()?;
                COUNT.fetch_add(1, Ordering::Relaxed);
            }
            Result::<(), GetTimezoneError>::Ok(())
        }));
    }
    for thread in threads {
        thread.join().unwrap()?;
    }
    assert_eq!(COUNT.load(Ordering::SeqCst), THREADS * ITERATIONS);
    Ok(())
}

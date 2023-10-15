use std::{sync::*, thread::JoinHandle, *};

const PerSecondRateLimit: u16 = 10;

pub struct RateLimiter {
    state: Arc<(Mutex<u16>, Condvar)>,
    timer_handle: JoinHandle<()>,
}

impl RateLimiter {
    pub fn new() -> RateLimiter {
        let pair = Arc::new((Mutex::new(PerSecondRateLimit), Condvar::new()));
        let timer_handle = Self::token_replenish_loop(pair.clone());

        RateLimiter {
            state: pair,
            timer_handle,
        }
    }

    fn token_replenish_loop(pair: Arc<(Mutex<u16>, Condvar)>) -> JoinHandle<()> {
        thread::spawn(move || loop {
            thread::sleep(time::Duration::from_secs(1));
            let (semaphore, condvar) = &*pair;
            *semaphore.lock().unwrap() = PerSecondRateLimit;
            condvar.notify_all();
        })
    }

    pub fn wait(&self) {
        let (semaphore, condvar) = &*self.state;
        let mut tokens = semaphore.lock().unwrap();
        while *tokens <= 0 {
            tokens = condvar.wait(tokens).unwrap();
            if *tokens > 0 {
                *tokens -= 1;
                return;
            }
        }
    }
}

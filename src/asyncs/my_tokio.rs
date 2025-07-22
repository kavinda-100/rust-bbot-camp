use std::pin::Pin;
use std::task::{Context, Poll};

pub async fn start() {
    print_async_message().await;

    f1racer_fn().await;
}

struct F1Racer {
    name: String,
    completed_laps: u32,
    laps: u32,
    best_lap_time: u32,
    lap_times: Vec<u32>,
}
impl F1Racer {
    fn new(name: &str, laps: u32) -> Self {
        F1Racer {
            name: name.to_string(),
            completed_laps: 0,
            laps,
            best_lap_time: u32::MAX,
            lap_times: Vec::with_capacity(laps as usize),
        }
    }
    fn do_laps(&mut self, lap_times: Vec<u32>) {
        for lap_time in lap_times {
            self.lap_times.push(lap_time);
            self.best_lap_time = self.best_lap_time.min(lap_time);
            self.completed_laps += 1;
            println!(
                "Lap {} completed by {}: Time: {} seconds",
                self.completed_laps, self.name, lap_time
            );
        }
    }
}
impl std::future::Future for F1Racer {
    type Output = u32;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        return std::task::Poll::Ready(self.completed_laps);
    }
}

async fn f1racer_fn() {
    let mut racer = F1Racer::new("Lewis Hamilton", 5);
    let lap_times = vec![32, 31, 30, 29, 28];
    racer.do_laps(lap_times);
    println!(
        "Racer {} completed {} laps with best lap time of {} seconds.",
        racer.name, racer.completed_laps, racer.best_lap_time
    );
}

async fn print_async_message() {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    println!("This is an asynchronous message from the my_tokio module.");
}

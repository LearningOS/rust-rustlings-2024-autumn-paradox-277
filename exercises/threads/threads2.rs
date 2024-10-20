// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.



use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 创建一个 Arc 包裹 Mutex，保护 JobStatus
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        // 克隆 Arc，以便在多个线程之间共享
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 锁定 Mutex，安全地更新 jobs_completed
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1; // 更新工作状态
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 在所有线程完成后打印 jobs_completed 的值
    let status = status.lock().unwrap();
    println!("jobs completed {}", status.jobs_completed);
}

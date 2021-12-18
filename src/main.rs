use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
  let size = 5e7 as usize;
  println!("Filling array({} elements)...", size);
  let arr = get_arr(size);
  println!("Calculating results...");

  log_single_threaded(&arr);
  log_multi_threaded(&arr, 2);
  log_multi_threaded(&arr, 8);
  log_multi_threaded(&arr, 16);
  log_multi_threaded(&arr, 100_000);
}

fn get_arr(arr_size: usize) -> Vec<f64> {
  const SAMPLE: usize = 20;
  let rand: [f64; SAMPLE] = rand::random();
  return (0..arr_size).map(|n| rand[n % SAMPLE] * n as f64).collect();
}

fn calc_single_threaded(arr: &Vec<f64>) -> (f64, Duration) {
  let start = Instant::now();
  (arr.iter().map(|n| n.sin()).sum(), start.elapsed())
}

fn calc_multi_threaded(arr: &Vec<f64>, threads: usize) -> (f64, Duration) {
  let start = Instant::now();
  let (tx, rx): (Sender<f64>, Receiver<f64>) = channel();
  let chunks = if arr.len() % threads == 0 { arr.len() / threads } else { arr.len() / (threads - 1) };
  // todo seems like chunks might contain repeating elements, or some are missing
  arr.chunks(chunks)
    .map(|chunk| (Vec::from_iter(chunk.iter().cloned()), tx.clone()))
    .for_each(|(chunk, tx)|
      { thread::spawn(move || tx.send(chunk.iter().map(|n| n.sin()).sum())); });

  drop(tx);
  (rx.iter().sum(), start.elapsed())
}

fn log_single_threaded(arr: &Vec<f64>) {
  let (value, duration) = calc_single_threaded(&arr);
  println!("Single threaded:\nValue: {}\nTime elapsed {:?}", value, duration);
}

fn log_multi_threaded(arr: &Vec<f64>, threads: usize) {
  let (value, duration) = calc_multi_threaded(&arr, threads);
  println!("\nMulti threaded({}):\nValue: {}\nTime elapsed {:?}", threads, value, duration);
}

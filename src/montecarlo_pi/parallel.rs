use std::sync::mpsc::{self, Receiver, Sender};

use failure::format_err;
use futures::{
    prelude::*,
    task::{self, Task},
};
use log::{debug, trace};
use rand::prelude::*;
use tokio::runtime::{self, Runtime, TaskExecutor};

use super::prelude::*;

struct ParallelCalculator {
    executor: TaskExecutor,
    plot_num: usize,
    thread_num: usize,
    invoked: bool,
    tx: Sender<Vec<f64>>,
    rx: Receiver<Vec<f64>>,
    result: Vec<f64>,
}

impl ParallelCalculator {
    fn new(executor: TaskExecutor, plot_num: usize, thread_num: usize) -> ParallelCalculator {
        let (tx, rx) = mpsc::channel();
        ParallelCalculator {
            executor,
            plot_num,
            thread_num,
            invoked: false,
            tx,
            rx,
            result: Vec::with_capacity(plot_num),
        }
    }

    fn print_result(&self) {
        println!("{:.5}", calculate_pi(&self.result));
    }
}

impl Future for ParallelCalculator {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        debug!("poll invoked: {}", self.invoked);
        if self.invoked {
            for ret in self.rx.try_iter() {
                let mut ret = ret;
                trace!("poll ret: {:?}", ret);
                self.result.append(&mut ret);
            }
            debug!("poll len: {}", self.result.len());
            if self.result.len() < self.plot_num {
                Ok(Async::NotReady)
            } else {
                self.print_result();
                Ok(Async::Ready(()))
            }
        } else {
            self.invoked = true;
            let num = self.plot_num / self.thread_num;
            let m = self.plot_num % self.thread_num;
            for i in 0..self.thread_num {
                let n = if i == self.thread_num - 1 {
                    num + m
                } else {
                    num
                };
                self.executor.spawn(Calculate::new(
                    &format!("calc-{}", i), n, self.tx.clone(), task::current()));
            }
            Ok(Async::NotReady)
        }
    }
}

struct Calculate {
    tag: String,
    num: usize,
    tx: Sender<Vec<f64>>,
    parent_task: Task,
}

impl Calculate {
    fn new(tag: &str, num: usize, tx: Sender<Vec<f64>>, parent_task: Task) -> Calculate {
        Calculate {
            tag: tag.to_string(),
            num,
            tx,
            parent_task,
        }
    }

    fn generate_point(&mut self, gen: &mut ThreadRng) -> Point {
        Point::new(gen.gen_range(0.0, 1.0), gen.gen_range(0.0, 1.0))
    }

    fn distance(&self, point: &Point) -> f64 {
        (point.x.powi(2) + point.y.powi(2)).sqrt()
    }
}

impl Future for Calculate {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut gen = rand::thread_rng();
        let mut result = Vec::with_capacity(self.num);
        for _ in 0..self.num {
            let point = self.generate_point(&mut gen);
            let distance = self.distance(&point);
            debug!("{} {}, distance: {:.3}", self.tag, point.flatten_short(), distance);
            result.push(distance);
        }

        debug!("{} send", self.tag);
        self.tx.send(result).unwrap();
        self.parent_task.notify();
        Ok(Async::Ready(()))
    }
}

pub fn parallel(num: usize, thread: usize) -> Fallible<()> {
    debug!("parallel");
    let mut runtime = runtime::Builder::new()
        .core_threads(thread)
        .build()? as Runtime;
    runtime.spawn(ParallelCalculator::new(runtime.executor(), num, thread));
    runtime.shutdown_on_idle()
        .wait()
        .or_else(|_| Err(format_err!("Runtime Error")))
}

use crate::montecarlo_pi::prelude::*;
use futures::prelude::*;
use rand::prelude::*;
use tracing::{debug, Instrument};

pub async fn parallel(num: usize, thread: usize, window: usize) -> Fallible<()> {
    debug!("parallel");

    let fut_num = if window == 0 {
        // auto window size.
        let a_num = num / thread;
        let mut val = (0..thread - 1).map(|_| a_num).collect::<Vec<_>>();
        val.push(a_num + num % thread);
        val
    } else {
        if num < window {
            // serial.
            vec![num]
        } else {
            let mut val = (0..num / window - 1).map(|_| window).collect::<Vec<_>>();
            val.push(window + num % window);
            val
        }
    };

    let futs = futures::stream::FuturesUnordered::new();
    for (i, num_sub) in fut_num.into_iter().enumerate() {
        let fut = async move {
            let mut gen = rand::thread_rng();
            let mut result = Vec::with_capacity(num_sub);

            for _ in 0..num_sub {
                let point = Point::new(gen.gen_range(0.0..=1.0), gen.gen_range(0.0..=1.0));
                let distance = (point.x.powi(2) + point.y.powi(2)).sqrt();
                debug!(
                    point = %point.flatten_short(),
                    distance = %format!("{:.3}", distance)
                );
                result.push(distance);
            }

            result
        }
        .instrument(tracing::info_span!("fut", %i));

        futs.push(fut);
    }

    let fut_results = futs
        .fold(Vec::with_capacity(num), |mut acc, mut data| async move {
            acc.append(&mut data);
            acc
        })
        .await;

    println!("{:.5}", calculate_pi(&fut_results));

    Ok(())
}

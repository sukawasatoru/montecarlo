use crate::prelude::*;
use futures::prelude::*;
use rand::prelude::*;

pub async fn parallel(num: usize, window: usize) -> Fallible<()> {
    debug!("parallel");

    let fut_num = if window == 0 {
        // auto window size.
        calc_window_size(num, num_cpus::get())
    } else {
        calc_window_size(num, window)
    };
    debug!(?fut_num);

    let futs = stream::FuturesUnordered::new();
    for (i, num_sub) in fut_num.into_iter().enumerate() {
        let fut = tokio::task::spawn(
            async move {
                let mut gen = thread_rng();
                let mut result = Vec::with_capacity(num_sub);

                for _ in 0..num_sub {
                    let point = generate_random_point(&mut gen);
                    let distance = distance(&point);
                    trace!(
                        point = %point.flatten_short(),
                        distance = %format!("{:.3}", distance),
                    );
                    result.push(distance);
                }

                result
            }
            .instrument(tracing::info_span!("fut", %i)),
        );

        futs.push(fut);
    }

    let fut_results = futs
        .fold(Vec::with_capacity(num), |mut acc, data| async move {
            acc.append(&mut data.unwrap());
            acc
        })
        .await;

    println!("{:.5}", calculate_pi(&fut_results));

    Ok(())
}

fn calc_window_size(num: usize, thread: usize) -> Vec<usize> {
    let a_num = num / thread;
    let mut val = (0..thread).map(|_| a_num).collect::<Vec<_>>();

    let mut rest = num % thread;
    'outer: loop {
        for entry in &mut val {
            if rest == 0 {
                break 'outer;
            }
            *entry += 1;
            rest -= 1;
        }
    }

    val.into_iter().filter(|&data| 0 < data).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_window_size_correct() {
        assert_eq!(vec![1], calc_window_size(1, 1));
        assert_eq!(vec![1], calc_window_size(1, 2));
        assert_eq!(vec![1], calc_window_size(1, 3));
        assert_eq!(vec![1], calc_window_size(1, 4));
        assert_eq!(vec![1], calc_window_size(1, 5));
        assert_eq!(vec![1], calc_window_size(1, 6));

        assert_eq!(vec![2], calc_window_size(2, 1));
        assert_eq!(vec![1, 1], calc_window_size(2, 2));
        assert_eq!(vec![1, 1], calc_window_size(2, 3));
        assert_eq!(vec![1, 1], calc_window_size(2, 4));
        assert_eq!(vec![1, 1], calc_window_size(2, 5));
        assert_eq!(vec![1, 1], calc_window_size(2, 6));

        assert_eq!(vec![3], calc_window_size(3, 1));
        assert_eq!(vec![2, 1], calc_window_size(3, 2));
        assert_eq!(vec![1, 1, 1], calc_window_size(3, 3));
        assert_eq!(vec![1, 1, 1], calc_window_size(3, 4));
        assert_eq!(vec![1, 1, 1], calc_window_size(3, 5));
        assert_eq!(vec![1, 1, 1], calc_window_size(3, 6));

        assert_eq!(vec![4], calc_window_size(4, 1));
        assert_eq!(vec![2, 2], calc_window_size(4, 2));
        assert_eq!(vec![2, 1, 1], calc_window_size(4, 3));
        assert_eq!(vec![1, 1, 1, 1], calc_window_size(4, 4));
        assert_eq!(vec![1, 1, 1, 1], calc_window_size(4, 5));
        assert_eq!(vec![1, 1, 1, 1], calc_window_size(4, 6));

        assert_eq!(vec![5], calc_window_size(5, 1));
        assert_eq!(vec![3, 2], calc_window_size(5, 2));
        assert_eq!(vec![2, 2, 1], calc_window_size(5, 3));
        assert_eq!(vec![2, 1, 1, 1], calc_window_size(5, 4));
        assert_eq!(vec![1, 1, 1, 1, 1], calc_window_size(5, 5));
        assert_eq!(vec![1, 1, 1, 1, 1], calc_window_size(5, 6));

        assert_eq!(vec![6], calc_window_size(6, 1));
        assert_eq!(vec![3, 3], calc_window_size(6, 2));
        assert_eq!(vec![2, 2, 2], calc_window_size(6, 3));
        assert_eq!(vec![2, 2, 1, 1], calc_window_size(6, 4));
        assert_eq!(vec![2, 1, 1, 1, 1], calc_window_size(6, 5));
        assert_eq!(vec![1, 1, 1, 1, 1, 1], calc_window_size(6, 6));
        assert_eq!(vec![1, 1, 1, 1, 1, 1], calc_window_size(6, 7));

        assert_eq!(vec![7], calc_window_size(7, 1));
        assert_eq!(vec![4, 3], calc_window_size(7, 2));
        assert_eq!(vec![3, 2, 2], calc_window_size(7, 3));
        assert_eq!(vec![2, 2, 2, 1], calc_window_size(7, 4));
        assert_eq!(vec![2, 2, 1, 1, 1], calc_window_size(7, 5));
        assert_eq!(vec![2, 1, 1, 1, 1, 1], calc_window_size(7, 6));
        assert_eq!(vec![1, 1, 1, 1, 1, 1, 1], calc_window_size(7, 7));
        assert_eq!(vec![1, 1, 1, 1, 1, 1, 1], calc_window_size(7, 8));
    }
}

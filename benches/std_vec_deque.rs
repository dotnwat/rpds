/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![cfg_attr(feature = "fatal-warnings", deny(warnings))]

#[macro_use]
extern crate criterion;

mod utils;

use criterion::{black_box, Criterion};
use std::collections::VecDeque;
use utils::limit;

fn std_vec_dequeue_push_back(c: &mut Criterion) {
    let limit = limit(10_000);

    c.bench_function("std vec dequeue push back", move |b| {
        b.iter(|| {
            let mut deque: VecDeque<usize> = VecDeque::new();

            for i in 0..limit {
                deque.push_back(i);
            }

            deque
        })
    });
}

fn std_vec_dequeue_pop_front(c: &mut Criterion) {
    let limit = limit(10_000);

    c.bench_function("std vec dequeue pop front", move |b| {
        b.iter_with_setup(
            || {
                let mut queue: VecDeque<usize> = VecDeque::new();

                for i in 0..limit {
                    queue.push_back(i);
                }

                queue
            },
            |mut queue| {
                for _ in 0..limit {
                    queue.pop_front();
                }

                queue
            },
        );
    });
}

fn std_vec_dequeue_iterate(c: &mut Criterion) {
    let limit = limit(10_000);
    let mut deque: VecDeque<usize> = VecDeque::new();

    for i in 0..limit {
        deque.push_back(i);
    }

    c.bench_function("std vec dequeue iterate", move |b| {
        b.iter(|| {
            for i in deque.iter() {
                black_box(i);
            }
        })
    });
}

criterion_group!(
    benches,
    std_vec_dequeue_push_back,
    std_vec_dequeue_pop_front,
    std_vec_dequeue_iterate
);
criterion_main!(benches);

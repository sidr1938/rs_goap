use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

fn bench_insert(c: &mut Criterion) {
    c.bench_function("hashmap_insert", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..10_000 {
                map.insert(i, i);
            }
        });
    });
}

fn bench_get_mut(c: &mut Criterion) {
    c.bench_function("hashmap_get_mut", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..10_000 {
                if let Some(val) = map.get_mut(&i) {
                    *val = i;
                } else {
                    map.insert(i, i);
                }
            }
        });
    });
}

criterion_group!(benches, bench_insert, bench_get_mut);
criterion_main!(benches);
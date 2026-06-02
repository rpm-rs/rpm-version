use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rpm_version::{Evr, EvrSortKey};

fn make_evr_strings() -> Vec<&'static str> {
    vec![
        "3:9.2.1-4.fc40",
        "1.0~rc1-1.el9",
        "2:3.14.159-2.6.el8",
        "0:1.0.0-1",
        "5.6.7-8.fc39",
        "1:0.99-100.el9",
        "2.0^git20230901-1.fc40",
        "1.0~alpha1-0.1",
        "10.0.0-1.el8",
        "1:2.0~beta3-2.fc39",
        "3.3.3-33.el9",
        "0:0.1-0.1.alpha",
        "2:1.0^post1-1",
        "4.0-1.fc40",
        "1.2.3-45.el9",
        "1:7.0~rc2-3.fc38",
        "99:0.0.1-1",
        "2.18.4-5.el9",
        "0:3.0^20240101-1",
        "1.0-1.fc40",
        "5:1.0~rc1^git1-1.el9",
        "12.0.0-1.fc39",
        "0:4.5.6-7.8.el8",
        "3:2.1.0-1.fc40",
        "1.99-2.el9",
        "0:1.0-1",
        "2:5.0~alpha-1.fc39",
        "8.0.1-3.el8",
        "1:3.2.1-1.fc40",
        "0:0.0.1~pre1-1",
        "6.6.6-6.el9",
        "2.0-1.fc40",
    ]
}

fn bench_sort(c: &mut Criterion) {
    let evr_strings = make_evr_strings();

    let mut group = c.benchmark_group("sort_evrs");

    for size in [8, 32, 128, 512] {
        let input: Vec<&str> = evr_strings.iter().copied().cycle().take(size).collect();

        group.bench_with_input(BenchmarkId::new("Ord", size), &input, |b, input| {
            b.iter(|| {
                let mut evrs: Vec<Evr> = input.iter().map(|s| Evr::parse(s)).collect();
                evrs.sort_unstable();
                evrs
            });
        });

        group.bench_with_input(BenchmarkId::new("sortkey", size), &input, |b, input| {
            b.iter(|| {
                let mut keyed: Vec<(EvrSortKey, usize)> = input
                    .iter()
                    .enumerate()
                    .map(|(i, s)| (EvrSortKey::parse(s), i))
                    .collect();
                keyed.sort_unstable();
                keyed
            });
        });

        group.bench_with_input(
            BenchmarkId::new("sortkey_precalc", size),
            &input,
            |b, input| {
                let precomputed: Vec<(EvrSortKey, usize)> = input
                    .iter()
                    .enumerate()
                    .map(|(i, s)| (EvrSortKey::parse(s), i))
                    .collect();
                b.iter(|| {
                    let mut keyed = precomputed.clone();
                    keyed.sort_unstable();
                    keyed
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_sort);
criterion_main!(benches);

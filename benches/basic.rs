use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

const EXAMPLE_DATA: &[&str] = &[
    // todo: longer data
    r#"\epsilon \delta \lambda"#,
    r#"3_4 r_{34} \alpha_7"#,
];
const AVAILABLE_REPLACE_IMPLS: [(&str, fn(&str) -> String); 2] = [
    ("optmizied", unicodeit::replace_optimized),
    ("naive", unicodeit::replace_naive),
];

fn bench_replace(c: &mut Criterion) {
    let mut group = c.benchmark_group("replace");
    for (index, data) in EXAMPLE_DATA.iter().enumerate() {
        group.throughput(Throughput::Bytes(data.len() as u64));
        for (impl_name, replace_func) in AVAILABLE_REPLACE_IMPLS {
            let bid = BenchmarkId::new(impl_name, format!("data{index:02}"));
            group.bench_with_input(bid, &index, |b, &actual_index| {
                b.iter(|| replace_func(EXAMPLE_DATA[actual_index]))
            });
        }
    }
    group.finish()
}

criterion_group!(benches, bench_replace);
criterion_main!(benches);

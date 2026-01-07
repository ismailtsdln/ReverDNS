use criterion::{black_box, criterion_group, criterion_main, Criterion};
use reverdns::DnsResolver;

fn dns_lookup_benchmark(c: &mut Criterion) {
    c.bench_function("dns_lookup_single", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let resolver = DnsResolver::new(5).await.unwrap();
                let _ = resolver.lookup(black_box("8.8.8.8")).await;
            })
    });
}

criterion_group!(benches, dns_lookup_benchmark);
criterion_main!(benches);

//! eval_harness.rs — criterion benchmark suite for tossinvest-rs
//!
//! Measures the SDK's hot paths: request body serialization and response
//! envelope deserialization. These dominate per-call CPU outside of network I/O.
//!
//! Run: cargo bench --bench eval_harness

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use tossinvest_rs::v1::domain::models::{
    ApiResponse, OrderCreateRequest, OrderSide, OrderType, PriceResponse,
};

/// 주문 생성 요청 직렬화 — adapter가 POST 본문을 만들 때마다 실행.
fn bench_serialize_order_request(c: &mut Criterion) {
    let request =
        OrderCreateRequest::quantity_based("005930", OrderSide::Buy, OrderType::Limit, "10")
            .price("70000")
            .client_order_id("client-order-id-001");
    c.bench_function("serialize/order_request", |b| {
        b.iter(|| {
            black_box(serde_json::to_string(black_box(&request)).unwrap());
        })
    });
}

/// 현재가 응답(`ApiResponse<Vec<PriceResponse>>`) 역직렬화 — throughput은 항목 수에 비례.
fn bench_deserialize_prices(c: &mut Criterion) {
    let mut group = c.benchmark_group("deserialize/prices");
    for size in [10usize, 50, 200] {
        let item = r#"{"symbol":"005930","timestamp":"2026-07-23T10:00:00+09:00","lastPrice":"70000","currency":"KRW"}"#;
        let body = format!(
            r#"{{"result":[{}]}}"#,
            (0..size).map(|_| item).collect::<Vec<_>>().join(",")
        );
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(format!("items/{size}"), &body, |b, body| {
            b.iter(|| {
                black_box(
                    serde_json::from_str::<ApiResponse<Vec<PriceResponse>>>(black_box(body))
                        .unwrap(),
                );
            })
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_serialize_order_request,
    bench_deserialize_prices
);
criterion_main!(benches);

# 벤치마크 및 품질 메트릭

> 최종 갱신: 2026-07-23 · 커밋 `04f8aac`

이 문서는 tossinvest-rs SDK의 **로컬 CPU 벤치마크**, **테스트 커버리지**, **정적 품질 메트릭**을 기록합니다. HTTP API 클라이언트이므로 실제 엔드투엔드 latency는 네트워크/토스증권 서버에 의존하지만, SDK 자체가 요청·응답마다 수행하는 직렬화·역직렬화 오버헤드는 로컬로 측정 가능하고 회귀 추적 가치가 있습니다.

## 측정 환경

| 항목 | 값 |
|------|-----|
| OS | macOS (Darwin 25.5.0, arm64) |
| Rust | 1.95.0 (stable) |
| 벤치 프레임워크 | criterion 0.5 |
| 실행 모드 | `cargo bench --bench eval_harness -- --quick` |

## 로컬 마이크로벤치

[`benches/eval_harness.rs`](../benches/eval_harness.rs)가 SDK의 핫패스를 측정합니다. 구간은 criterion이 보고하는 중앙값이며, 괄호 안은 95% 신뢰구간 하한/상한입니다.

| 벤치 | 측정 항목 | 중앙값 | 구간 |
|------|----------|-------:|------|
| `serialize/order_request` | 주문 생성 요청 본문 직렬화 (`OrderCreateRequest`) | **128 ns** | 128–130 ns |
| `deserialize/prices/items/10` | 현재가 응답 역직렬화 (10개 항목) | **1.85 µs** | 1.83–1.85 µs |
| `deserialize/prices/items/50` | 현재가 응답 역직렬화 (50개 항목) | **8.50 µs** | 8.45–8.73 µs |
| `deserialize/prices/items/200` | 현재가 응답 역직렬화 (200개 항목) | **33.47 µs** | 33.43–33.48 µs |

### 해석

- **직렬화 (~128 ns)**: 단일 주문 요청 본문 직렬화. 마이크로초 미만으로 사용자 체감 무시 가능.
- **역직렬화**: 항목당 약 **167 ns**로 선형 증가. 목록 응답(체결 내역, 순위 등) 크기에 비례합니다.

> 측정은 `--quick` 모드이므로 절대 수치보다 **회귀 추세**(미래 실행과의 비교)가 주 목적입니다.

## 테스트 커버리지

| 항목 | 값 |
|------|-----|
| 단위 테스트 (`src/`) | 4 통과 |
| 통합 회귀 (`tests/endpoints.rs`) | 18 통과 |
| Doctest | 1 통과 |
| **합계** | **23 통과 / 0 실패** |

통합 테스트는 wiremock 기반으로 OAuth2 토큰 발급·`Bearer`/`AccountSeq` 헤더 주입·라우팅·비즈니스/OAuth2 에러 매핑을 검증합니다.

## 정적 품질 메트릭 (epic eval)

`epic eval` 게이트 결과:

| Dimension | 점수 | 판정 |
|-----------|-----:|------|
| correctness | 1.00 | PASS |
| performance | 1.00 | PASS |
| quality | 1.00 | PASS |
| regression | 1.00 | PASS |
| **overall** | **1.00** | **PASS** |

- `cargo fmt --check`: clean
- `cargo clippy --all-targets -- -D warnings`: clean

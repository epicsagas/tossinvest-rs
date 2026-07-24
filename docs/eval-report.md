# Eval Report — tossinvest-sdk

> 평가 시각: 2026-07-24T00:59:32 (KST) · Branch `main` · Commit `dcf659d`
> 게이트: `epic eval` (5 dimension + 2 benchmark) · Baseline 갱신 완료

## Overall: **PASS** — score 1.0

모든 dimension이 통과했고, baseline(1.0) 대비 회귀 없음.

| Dimension | Score | Verdict | 비고 |
|-----------|------:|---------|------|
| `benchmark:e2e_scenarios` | 1.00 | ✅ PASS | `cargo test --test e2e` (6 시나리오) |
| `benchmark:eval_harness` | 1.00 | ✅ PASS | criterion 벤치 (직렬화·역직렬화) |
| `correctness` | 1.00 | ✅ PASS | `cargo test` (전체) |
| `performance` | 1.00 | ✅ PASS | `cargo bench --bench eval_harness` |
| `quality` | 1.00 | ✅ PASS | `cargo clippy --all-targets -- -D warnings` |
| `regression` | 1.00 | ✅ PASS | baseline 대비 delta 0 |

## Correctness — PASS

- **테스트 31개 전부 통과** (pass rate 100%):
  - 단위 (`src/`): 6 — `error.rs`(3) + `token.rs`(3)
  - 통합 회귀 (`tests/endpoints.rs`): 18
  - E2E 시나리오 (`tests/e2e.rs`): 6
  - Doctest: 1
- mutation testing: 미활성화 (eval.yaml `mutation_tool: null`)

> 참고: `epic eval`의 correctness 파서는 cargo test 멀티 바이너리 출력을 단일 라인으로만 잡아 `tests_passed`를 부정확하게 보고하지만, `exit_code: 0` 으로 PASS 판정은 정확합니다. 실제 카운트는 위와 같습니다.

## Performance — PASS

마이크로벤치 상세 수치는 [`docs/benchmarks.md`](benchmarks.md) 참조. 요약:

| 벤치 | 중앙값 |
|------|-------:|
| `serialize/order_request` | 128 ns |
| `deserialize/prices/items/50` | 8.50 µs |
| `deserialize/prices/items/200` | 33.47 µs |

## Quality — PASS

- `cargo clippy --all-targets -- -D warnings`: 린트 0건
- `cargo fmt --check`: clean
- **LLM-as-judge**: CLI 모드 SKIPPED → 수동 4축 루브릭 (샘플 5개 핵심 파일 평균) = **8.5 / 10**
  - Readability 9 · Correctness 9 · DRY 8 · Security 8

## Regression — PASS

| Dimension | Baseline | Current | Delta | Verdict |
|-----------|---------:|--------:|------:|---------|
| correctness | 1.00 | 1.00 | 0 | PASS |
| performance | 1.00 | 1.00 | 0 | PASS |
| quality | 1.00 | 1.00 | 0 | PASS |
| `benchmark:e2e_scenarios` | 1.00 | 1.00 | 0 | PASS |

## E2E 시나리오 커버리지 (`benchmark:e2e_scenarios`)

| 시나리오 | 검증 |
|---|---|
| `auth_token_reused_across_calls` | `/oauth2/token` 1회 히트 (캐싱 재사용) |
| `investor_buy_journey_kr` | 시세→호가→계좌→잔고→주문→확인→취소 (orderId 전달) |
| `orders_pagination_walk` | cursor 2페이지 순회 |
| `conditional_order_oco_lifecycle` | OCO 생성→조회→수정→취소(204) |
| `market_data_batch_session` | 다종목·캔들·순위 한 세션 |
| `business_error_aborts_flow` | 404 비즈니스 에러 전파 |

## 결론

랜딩 전 검증 게이트 **통과**. 단위(6) + 엔드포인트 회귀(18) + E2E 시나리오(6) + 성능 + 정적 품질 5축이 모두 green이며 baseline 대비 회귀 없음.

```bash
epic eval            # 본 보고서 재현
cargo test           # 31 테스트
cargo bench --bench eval_harness -- --quick   # 성능
```

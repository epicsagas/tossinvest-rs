# 기여 가이드

토스증권 Open API 비공식 Rust SDK에 기여해 주셔서 감사합니다. 핵심 요약:

```bash
cargo fmt                                         # 포맷
cargo clippy --all-targets -- -D warnings         # 린트
cargo test                                        # 단위 + 통합 회귀
cargo bench --bench eval_harness -- --quick       # 성능 (선택)
```

## 아키텍처 (Hexagonal / Ports & Adapters)

```
src/
├── domain/models/   # OpenAPI 스펙의 72개 데이터 구조 (serde)
├── port/            # 도메인별 trait (MarketDataPort · TradingPort · …)
├── adapter/
│   ├── http_client.rs   # 모든 포트 trait 의 구체 구현 (진입점)
│   ├── token.rs         # OAuth2 토큰 발급·캐싱·자동갱신
│   └── *_adapter.rs     # HttpClient 의 포트 trait 구현
└── error.rs         # 단일 SdkError
```

- **`port/`** 는 "무엇을" 할지 정의. 신규 엔드포인트는 먼저 해당 도메인 trait에 메서드를 추가하세요.
- **`adapter/`** 는 "어떻게" HTTP로 수행할지 구현. `HttpClient` 의 `get`/`post`/`post_no_body`/`delete_void` 헬퍼를 사용합니다.
- **`domain/models/`** 는 API 스펙의 요청·응답 구조체. 응답은 `{"result": T}` envelope으로 오므로, 어댑터가 `ApiResponse<T>`를 풀어 `T`를 반환합니다.

## 신규 엔드포인트 추가 절차

1. `docs/openapi.json`에서 응답/요청 스키마를 확인해 `src/domain/models/`에 serde 구조체 추가.
2. 적절한 `src/port/*_port.rs` trait에 `async fn` 추가 (입출력은 도메인 모델 + `SdkError`).
3. 대응하는 `src/adapter/*_adapter.rs`에 `impl Port for HttpClient` 구현.
4. `tests/endpoints.rs`에 wiremock 회귀 테스트(성공 + 에러 매핑) 추가.
5. `cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test`.

## 커밋 규칙

[Conventional Commits](https://www.conventionalcommits.org/) — `type(scope): description` (소문자, 이모지 없음, 공동저자 푸터 없음).

- `feat`: 신규 엔드포인트/기능
- `fix`: 버그 수정
- `test`: 테스트 추가/수정
- `docs`: 문서
- `refactor`: 리팩터링

## API 스펙 갱신 시

`docs/openapi.json`을 교체한 뒤, 영향받는 도메인 모델·포트·어댑터·테스트를 수동으로 맞춥니다. (portone-rs와 달리 코드 생성기를 쓰지 않습니다 — 27 엔드포인트 규모에서는 수작업이 더 단순하고 투명합니다.)

## 서버 전용 (Wasm 미지원)

본 SDK는 **서버 사이드 전용**입니다. 토스증권 Open API는 OAuth2 **Client Credentials** 그랜트(`client_id`/`client_secret`)를 사용하므로, 브라우저(wasm)에서 호출하면 `client_secret`이 클라이언트에 노출되어 **보안상 허용되지 않습니다**. wasm 타겟 컴파일이나 브라우저 fetch는 지원하지 않습니다. 브라우저 클라이언트는 백엔드(BFF)를 통해 호출하세요.

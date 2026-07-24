# Changelog

본 프로젝트의 주요 변경사항은 [Keep a Changelog](https://keepachangelog.com/ko/1.1.0/) 형식을 따르며,
버전은 [Semantic Versioning](https://semver.org/lang/ko/)을 준수합니다.

## [Unreleased]

## [0.1.0] - 2026-07-24

### Added
- **헥사고날 아키텍처 SDK**: domain models(72 스키마) · 5개 port trait(`MarketDataPort` · `AccountPort` · `TradingPort` · `ConditionalOrderPort` · `AuthPort`) · 어댑터 · 단일 `SdkError`
- **OAuth2 Client Credentials** 토큰 자동 발급·캐싱·만료 갱신
- **`AccountSeq` 헤더** 자동 주입 (`with_account` 빌더)
- **`ApiResponse { result }` envelope** 자동 처리
- **29개 엔드포인트**: 시장 데이터 14 · 계좌/잔고 2 · 주문 8 · 조건부주문 5
- **테스트**: wiremock 회귀(`endpoints` 18) · E2E 시나리오 6 · 단위 6 · doctest 1 (합계 31)
- **성능**: criterion 벤치마크 (직렬화 128ns · 역직렬화 50개 8.50µs)
- **편의**: `prelude` 모듈 · `examples/quickstart.rs`
- **인프라**: CI 워크플로 · `CONTRIBUTING.md` · `SECURITY.md` · `epic eval` 품질 게이트

### Notes
- 토스증권 Open API v1.2.4 (OpenAPI 3.1.0) 기반
- 비공식(Unofficial) SDK — 토스증권 공식 지원 아님

[Unreleased]: https://github.com/epicsagas/tossinvest-sdk/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/epicsagas/tossinvest-sdk/releases/tag/v0.1.0

# 보안 정책 (Security Policy)

## 지원 버전

| 버전 | 지원 여부 |
|------|-----------|
| 0.1.x | ✅ 지원 |

## 취약점 신고

보안 취약점은 공개 이슈가 아닌 **비공개 채널**로 신고해 주세요.

- **GitHub Private Vulnerability Reporting**: [신고하기](https://github.com/epicsagas/tossinvest-sdk/security/advisories/new)

공개 전 패치가 완료될 때까지 취약점 세부사항을 비공개로 유지합니다.

## 응답 SLA

- **신고 확인**: 48시간 이내
- **패치 목표**: 90일 이내 (심각도에 따라 단축)

## 범위

본 SDK는 토스증권 Open API를 호출하는 클라이언트 라이브러리입니다. **SDK 자체**의 취약점(자격증명 처리·직렬화·에러 처리 등)만 다룹니다.

- 토스증권 API 서버 측 취약점 → 토스증권 공식 채널에 직접 신고하세요.
- 일반 사용 문의·버그 → GitHub [Issues](https://github.com/epicsagas/tossinvest-sdk/issues)를 사용하세요.

## 보안 설계 참고

- `client_secret`은 서버 사이드에서만 사용해야 합니다. 브라우저/wasm 노출 금지 — [CONTRIBUTING.md](CONTRIBUTING.md) 참고.

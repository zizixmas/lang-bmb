# BMB v0.2 Sprout 구현 ✅ 완료

## 개요

v0.2는 SMT 솔버(Z3)를 연동하여 함수 계약(pre/post)을 정적으로 검증하는 기능을 추가합니다.

## 기술 스택

| 구성요소 | 방식 | 설명 |
|----------|------|------|
| SMT 솔버 | Z3 외부 프로세스 | SMT-LIB2 텍스트 기반 연동 |
| SMT 이론 | QF_LIA | 선형 정수 산술 (Quantifier-Free Linear Integer Arithmetic) |
| 프로세스 연동 | stdin/stdout | SMT-LIB2 스크립트 전달 및 결과 파싱 |

## 검증 원리

### 계약 검증 방식

```
함수: fn f(x: T) -> R pre P post Q = body

검증할 조건:
1. Pre 만족성: ∃x. P(x) - 유효한 입력이 존재하는가?
2. Post 정확성: ∀x. P(x) ∧ (ret = body) → Q(x, ret)
   - 반증: ∃x. P(x) ∧ (ret = body) ∧ ¬Q(x, ret) 가 UNSAT이면 검증 성공
```

### SMT 변환 규칙

| BMB 표현식 | SMT-LIB |
|-----------|---------|
| `i32`, `i64` | `Int` |
| `bool` | `Bool` |
| `a + b` | `(+ a b)` |
| `a - b` | `(- a b)` |
| `a * b` | `(* a b)` |
| `a / b` | `(div a b)` |
| `a % b` | `(mod a b)` |
| `a == b` | `(= a b)` |
| `a != b` | `(not (= a b))` |
| `a < b` | `(< a b)` |
| `a <= b` | `(<= a b)` |
| `a > b` | `(> a b)` |
| `a >= b` | `(>= a b)` |
| `a and b` | `(and a b)` |
| `a or b` | `(or a b)` |
| `not a` | `(not a)` |
| `if c then t else e` | `(ite c t e)` |
| `ret` (post) | `__ret__` 변수 |

## 프로젝트 구조 ✅

```
bmb/src/
├── smt/
│   ├── mod.rs          # SMT 모듈 진입점
│   ├── translator.rs   # AST → SMT-LIB2 텍스트 변환
│   └── solver.rs       # Z3 프로세스 연동 및 결과 파싱
├── verify/
│   ├── mod.rs          # 검증 모듈 진입점
│   └── contract.rs     # pre/post 검증 + 반례 리포팅
```

## 구현 페이즈

### Phase 1: SMT 모듈 기반 (smt/) ✅

1. `translator.rs`: BMB Expr → SMT-LIB2 텍스트 변환
   - `SmtLibGenerator`: SMT-LIB2 스크립트 생성
   - `SmtTranslator`: AST 표현식 → SMT-LIB2 문자열
   - 산술/비교/논리 연산 변환
   - 조건문 (ite) 변환

### Phase 2: Z3 프로세스 연동 ✅

1. `solver.rs`: Z3 외부 프로세스 호출
   - stdin으로 SMT-LIB2 스크립트 전달
   - stdout에서 결과 파싱 (sat/unsat/unknown)
   - 모델 파싱 (define-fun 형식)
   - 타임아웃 및 에러 처리

### Phase 3: 계약 검증기 (verify/) ✅

1. `contract.rs`:
   - Pre-condition 검증: 만족 가능성 검사 (SAT)
   - Post-condition 검증: 타당성 검사 (¬post가 UNSAT)
   - 함수 본문을 `__ret__` 변수에 바인딩
   - `VerificationReport`: 프로그램 수준 결과
   - `FunctionReport`: 함수 수준 결과

### Phase 4: 반례 리포터 ✅

1. `solver.rs`의 `Counterexample` 구조체:
   - Z3 모델에서 변수 값 추출
   - `(define-fun var () Int value)` 형식 파싱
   - 사용자 친화적 포맷으로 출력

### Phase 5: CLI 통합 ✅

1. `bmb verify <file>` 명령어 추가
   - `--z3-path`: Z3 실행 파일 경로 지정
   - `--timeout`: 검증 타임아웃 설정
2. 검증 결과 출력: ✓/✗ 기호로 성공/실패 표시
3. 에러 코드 정의

## 예상 출력

### 검증 성공
```
$ bmb verify examples/safe_divide.bmb
✓ safe_divide: pre verified
✓ safe_divide: post verified
All contracts verified successfully.
```

### 검증 실패 (반례)
```
$ bmb verify examples/bad_abs.bmb
✗ bad_abs: post verification failed

  │ fn bad_abs(x: i32) -> i32
  │   post ret >= 0
  │ = x;
  │   ─ returns x directly without abs
  │
  │ Counterexample:
  │   x = -1
  │   ret = -1 (violates: ret >= 0)
```

## 테스트 케이스 계획

### 검증 성공 케이스
- `verify_001_identity.bmb`: 항등 함수 (pre/post 없음)
- `verify_002_positive_guard.bmb`: `pre x > 0 post ret > 0`
- `verify_003_addition.bmb`: 덧셈 결과 검증
- `verify_004_if_else.bmb`: 조건문 검증

### 검증 실패 케이스 (반례 생성)
- `fail_001_missing_abs.bmb`: abs 없이 음수 반환
- `fail_002_division_by_zero.bmb`: 0으로 나눔 가능
- `fail_003_overflow.bmb`: 오버플로우 미검증

## 제한사항 (v0.2)

1. **지원 타입**: i32, i64, bool만 검증
2. **비선형 산술**: 곱셈은 상수와의 곱만 완전 지원
3. **함수 호출**: 인라인 또는 @trust 필요
4. **재귀**: 미지원 (v0.3+)
5. **배열/참조**: 미지원 (v0.3+)

## 의존성 변경

```toml
[dependencies]
z3 = "0.12"
```

## 마일스톤

- [ ] Phase 1: SMT 모듈 기반 구현
- [ ] Phase 2: Z3 연동 및 기본 테스트
- [ ] Phase 3: pre/post 검증 로직
- [ ] Phase 4: 반례 추출 및 포매팅
- [ ] Phase 5: CLI 통합
- [ ] Phase 6: 테스트 및 문서화

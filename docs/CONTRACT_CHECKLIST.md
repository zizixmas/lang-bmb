# BMB Contract Programming Checklist

> Design-by-Contract 및 정적 검증 기능 추적

---

## 계약 문법 (Contract Syntax)

### 함수 레벨 계약

| 구문 | 버전 | 상태 | 설명 |
|------|------|------|------|
| `pre <expr>` | v0.2 | ✅ | 함수 전제조건 |
| `post <expr>` | v0.2 | ✅ | 함수 후제조건 |
| `modifies <expr>` | v0.3 | 계획 | 부작용 명세 |
| `@pure` | v0.3 | 계획 | 순수 함수 표시 |
| `@trust` | v0.2 | ✅ | 검증 건너뛰기 |
| `@check` | v0.2 | ✅ | 타임아웃 시 런타임 검사 |

### 루프 레벨 계약

| 구문 | 버전 | 상태 | 설명 |
|------|------|------|------|
| `@invariant` | v0.3 | ✅ | 루프 불변식 |
| `@decreases` | v0.3 | ✅ | 종료 증명 (감소 표현식) |

### 타입 레벨 계약

| 구문 | 버전 | 상태 | 설명 |
|------|------|------|------|
| `T{constraint}` | v0.2 | ✅ | 정제 타입 (Refinement Type) |
| `T{self => cond}` | v0.2 | ✅ | 명시적 self 바인딩 |
| `NonZero = i32{self != 0}` | v0.2 | ✅ | 타입 별칭 |
| `@disjoint` 파싱 | v0.2 | ✅ | 분리 조건 파싱 |
| `@disjoint` SMT | v0.3 | ✅ | 분리 조건 SMT 검증 |

---

## 논리 연산자 (Logical Operators)

| 연산자 | 버전 | 상태 | 설명 |
|--------|------|------|------|
| `and` | v0.1 | ✅ | 논리곱 (conjunction) |
| `or` | v0.1 | ✅ | 논리합 (disjunction) |
| `not` | v0.1 | ✅ | 부정 (negation) |
| `=>` | v0.2 | ✅ | 논리적 함축 (implication) |
| `<=>` | v0.3 | ✅ | 동치 (biconditional) |

---

## 한정사 (Quantifiers)

| 구문 | 버전 | 상태 | 설명 |
|------|------|------|------|
| `forall(x in range): expr` | v0.2 | ✅ | 전칭 한정사 |
| `exists(x in range): expr` | v0.2 | ✅ | 존재 한정사 |
| `forall(x in array): expr` | v0.4 | 계획 | 배열 순회 한정사 |
| 중첩 한정사 | v0.2 | ✅ | forall 내 exists 등 |

---

## 패턴 조건 (Pattern Conditions)

| 구문 | 버전 | 상태 | 설명 |
|------|------|------|------|
| `expr is Pattern` | v0.2 | ✅ | 패턴 매칭 조건 |
| `expr is Some(x)` | v0.2 | ✅ | Option 패턴 |
| `expr is Ok(v)` | v0.2 | ✅ | Result 패턴 |
| Guard 조건 | v0.4 | 계획 | `is Pattern if cond` |

---

## 특수 표현식 (Special Expressions)

| 구문 | 버전 | 상태 | 설명 |
|------|------|------|------|
| `old(expr)` | v0.2 | ✅ | post에서 이전 값 참조 |
| `result` | v0.2 | ✅ | post에서 반환값 참조 |
| `self` | v0.2 | ✅ | 정제 타입에서 현재 값 |
| `.pre` / `.post` | v0.2 | ✅ | 계약 컨텍스트 전용 |

---

## 범위 연산자 (Range Operators)

| 구문 | 버전 | 상태 | 설명 |
|------|------|------|------|
| `a..b` | v0.2 | ✅ | 반개방 범위 [a, b) |
| `a..=b` | v0.3 | 계획 | 폐쇄 범위 [a, b] |
| `..b` | v0.3 | 계획 | 시작 없는 범위 |
| `a..` | v0.3 | 계획 | 끝 없는 범위 |

### 범위 우선순위

```
비교 연산자 < 범위 연산자 < 산술 연산자
예: 0 < x..n 파싱 → 0 < (x..n) (범위가 먼저)
```

---

## SMT 통합 (SMT Integration)

| 기능 | 버전 | 상태 | 설명 |
|------|------|------|------|
| Z3 프로세스 연동 | v0.2 | ✅ | 외부 Z3 호출 |
| SMT-LIB2 생성 | v0.2 | ✅ | AST → SMT 변환 |
| 반례 파싱 | v0.2 | ✅ | 검증 실패 시 반례 |
| 배열 이론 | v0.4 | 계획 | SMT Array Theory |
| 비트벡터 이론 | v0.4 | 계획 | 정수 오버플로우 검증 |

---

## 검증 모드 (Verification Modes)

| 모드 | 어노테이션 | 동작 |
|------|------------|------|
| 전체 검증 | (없음) | SMT 검증 필수 |
| 신뢰 | `@trust` | 검증 건너뛰기 |
| 런타임 검사 | `@check` | 타임아웃 시 런타임 어설션 |

---

## 버전별 마일스톤

### v0.2 Sprout ✅
- [x] 기본 pre/post 계약
- [x] forall/exists 한정사
- [x] => 논리적 함축
- [x] is 패턴 조건
- [x] 정제 타입 T{constraint}
- [x] old(expr), result 특수 표현식
- [x] @trust, @check 어노테이션
- [x] Z3 연동 및 반례 리포팅

### v0.3 Root ✅
- [x] 루프 @invariant, @decreases
- [x] @disjoint SMT 통합
- [x] <=> 동치 연산자
- [ ] 배열 한정사 (v0.4+)
- [ ] Guard 조건 (is ... if) (v0.4+)
- [ ] SMT 배열 이론 (v0.4+)

### v0.4+ (미래)
- [ ] modifies 절
- [ ] @pure 어노테이션
- [ ] 비트벡터 오버플로우 검증
- [ ] 병렬 계약 검증

---

## 참고 자료

- [BMB LAWS](LAWS.md) - 언어 설계 원칙
- [SYNTAX.md](SYNTAX.md) - 전체 문법
- [ROADMAP.md](ROADMAP.md) - 개발 로드맵

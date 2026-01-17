# Hangul Number Converter

## 제작동기
전화번호를 차량앞에 부착하면 뭔가 보안에 좋지 않다고 합니다.
내 전화번호, 주민번호를 읽기쉬운 한글로 부른다면 보안에 도움이 될수 있지 않을까 해서 제작했습니다.


**Base-128 변수 길이 한글 숫자 변환기**

음수가 아닌 정수를 읽기 쉬운 한글 음절로 인코딩하고, 다시 원래 숫자로 디코딩하는 라이브러리입니다. 128개의 자연스러운 한글 음절을 사용하여 효율적인 변환을 제공합니다.

## 주요 특징

- **Base-128 인코딩**: 효율적인 데이터 압축
- **변수 길이 인코딩**: 작은 수는 짧게, 큰 수는 길게 인코딩
- **씨드 기반 스크램블링**: 동일한 숫자도 128가지 다른 방식으로 인코딩 가능
- **다국어 지원**: TypeScript, C#, Rust 구현 제공
- **한글 최적화**: 자연스러운 발음의 128개 한글 음절만 사용
- **완전한 가역성**: 인코딩된 한글은 항상 정확히 원래 숫자로 복원됨

## 지원 언어

| 언어 | 파일 | 상태 |
|------|------|------|
| TypeScript | `hangulNumber.ts` | ✅ 완성 |
| C# | `hangulNumber.cs` | ✅ 완성 |
| Rust | `hangulNumber.rs` | ✅ 완성 |

## 빠른 시작

### TypeScript

```typescript
import { HangulNumberConverter } from './hangulNumber';

// 기본 인코딩 (무작위 씨드 사용)
const encoded = HangulNumberConverter.encode(12345);
console.log(encoded); // 예: "강나다마"

// 디코딩
const decoded = HangulNumberConverter.decode(encoded);
console.log(decoded); // 12345

// 특정 씨드로 인코딩
const encodedWithSeed = HangulNumberConverter.encodeWithSeed(12345, 0);

// 모든 128가지 인코딩
const allEncodings = HangulNumberConverter.encodeAll(12345);
console.log(allEncodings.length); // 128
```

실행:
```bash
npx ts-node hangulNumber.ts
```

### C#

```csharp
// 기본 인코딩
string encoded = HangulNumberConverter.Encode(12345);
Console.WriteLine(encoded); // 예: "강나다마"

// 디코딩
long decoded = HangulNumberConverter.Decode(encoded);
Console.WriteLine(decoded); // 12345

// 특정 씨드로 인코딩
string encodedWithSeed = HangulNumberConverter.EncodeWithSeed(12345, 0);

// 모든 128가지 인코딩
string[] allEncodings = HangulNumberConverter.EncodeAll(12345);
```

실행:
```bash
dotnet run hangulNumber.cs
```

### Rust

```rust
use hangul_number::HangulNumberConverter;

fn main() {
    let converter = HangulNumberConverter::new();

    // 기본 인코딩
    let encoded = converter.encode(12345).unwrap();
    println!("{}", encoded); // 예: "강나다마"

    // 디코딩
    let decoded = converter.decode(&encoded).unwrap();
    println!("{}", decoded); // 12345

    // 특정 씨드로 인코딩
    let encoded_with_seed = converter.encode_with_seed(12345, 0).unwrap();

    // 모든 128가지 인코딩
    let all_encodings = converter.encode_all(12345).unwrap();
    println!("{}", all_encodings.len()); // 128
}
```

실행:
```bash
rustc hangulNumber.rs -o hangul_number && ./hangul_number
```

## 사용 예시

### 대화형 CLI

모든 언어 구현에서 대화형 CLI를 지원합니다:

```
=== Hangul Number Converter (Base-128, Variable Length) ===
Enter a non-negative integer to encode.
Type 'exit' to quit.

Enter number: 12345

All 128 encodings for 12,345:
--------------------------------------------------
강나다마✓  강나다마✓  강나다마✓  강나다마✓  강나다마✓  강나다마✓  강나다마✓  강나다마✓
...
```

### 프로그래매틱 사용

```typescript
// 매우 큰 숫자도 처리 가능
const encoded = HangulNumberConverter.encode(999999999);
console.log(encoded.length); // 약 4-5개 한글 문자

// 모든 인코딩이 동일한 숫자로 복원됨
const allEnc = HangulNumberConverter.encodeAll(42);
allEnc.forEach(enc => {
    console.assert(HangulNumberConverter.decode(enc) === 42);
});
```

## 한글 문자 선택

128개 한글 음절은 이니셜(자음)별로 그룹화되어 있습니다:

- **ㄱ (9)**: 가, 간, 강, 개, 거, 고, 공, 구, 금
- **ㄴ (9)**: 나, 날, 남, 내, 너, 노, 눈, 늘, 니
- **ㄷ (9)**: 다, 달, 담, 대, 더, 도, 동, 두, 드
- ... 및 기타 자음

각 한글 문자는:
- 자연스러운 발음
- 일상적인 사용
- 읽기 쉬운 형태

를 고려하여 선택되었습니다.

## 인코딩 메커니즘

### 기본 원리

1. **씨드 생성**: 0~127 범위의 무작위 정수
2. **Base-128 변환**: 입력 숫자를 128진법으로 변환
3. **스크램블링**: 각 자리 숫자를 `(digit + seed) % 128`로 변환
4. **한글 매핑**: 스크램블된 값을 CHARS 배열의 한글 문자로 매핑
5. **결합**: 첫 번째는 씨드 문자, 나머지는 스크램블된 데이터

### 예시

```
숫자: 12345
씨드: 0

1. Base-128 변환:
   12345 = 96×128 + 57
   = 0x60 0x39

2. 스크램블 (seed=0):
   96 % 128 = 96 → CHARS[96]
   57 % 128 = 57 → CHARS[57]

3. 씨드 포함:
   CHARS[0] + CHARS[96] + CHARS[57]
   = "가" + "..." + "..."
```

## API 레퍼런스

### TypeScript / JavaScript

#### `encode(num: number): string`
무작위 씨드로 숫자를 인코딩합니다.

#### `encodeWithSeed(num: number, seed: number): string`
특정 씨드로 숫자를 인코딩합니다. (deterministic)

#### `encodeAll(num: number): string[]`
모든 128가지 인코딩을 반환합니다.

#### `decode(str: string): number`
한글 문자열을 원래 숫자로 디코딩합니다.

### C#

메서드명은 PascalCase를 사용합니다:
- `Encode(long num): string`
- `EncodeWithSeed(long num, int seed): string`
- `EncodeAll(long num): string[]`
- `Decode(string str): long`

### Rust

메서드명은 snake_case를 사용합니다:
- `encode(&self, num: u64) -> Result<String, String>`
- `encode_with_seed(&self, num: u64, seed: usize) -> Result<String, String>`
- `encode_all(&self, num: u64) -> Result<Vec<String>, String>`
- `decode(&self, s: &str) -> Result<u64, String>`

## 성능

| 작업 | 시간복잡도 | 설명 |
|------|-----------|------|
| 인코딩 | O(log₁₂₈ n) | n의 크기에 따라 선형 |
| 디코딩 | O(m) | m은 인코딩된 문자 수 |
| 역맵 생성 | O(1) | 정적 초기화, 일회성 |

## 제한사항

### TypeScript
- JavaScript의 Number 안전 범위: 2^53 - 1 (약 9×10^15)

### C#
- long 범위: -2^63 ~ 2^63 - 1 (음수는 지원하지 않음)

### Rust
- u64 범위: 0 ~ 2^64 - 1

## 테스트

### Rust
```bash
rustc --test hangulNumber.rs -o test_hangul
./test_hangul
```

포함된 테스트:
- Zero 인코딩/디코딩
- 다양한 숫자값 테스트
- 128가지 모든 인코딩 검증
- 잘못된 씨드 처리
- 잘못된 문자열 처리

## 사용 사례

### 🔐 데이터 난독화
- 민감한 ID를 읽을 수 없는 한글로 변환
- 씨드로 추가 보안

### 🎲 게임 개발
- 무작위 생성 씨드로 다양한 코드 생성
- 플레이어가 쉽게 읽을 수 있는 코드

### 🏷️ 쿠폰 / 프로모션 코드
- 128가지 변형으로 중복 가능성 감소
- 한글로 된 자연스러운 코드

### 📱 QR 코드 / 바코드
- 숫자를 효율적으로 인코딩
- 한글 문자로 사람이 읽을 수 있는 백업 코드

## 제한사항 및 주의사항

⚠️ **암호화 용도 부적합**: 이 라이브러리는 암호화를 목적으로 설계되지 않았습니다. 보안이 중요한 경우 전문 암호화 라이브러리를 사용하세요.

⚠️ **씨드 값 유지**: 동일한 인코딩을 재생성하려면 씨드 값을 저장해야 합니다.

⚠️ **한글 문자만**: 입력/출력은 정의된 128개 한글 문자만 사용합니다.

## 기여 가이드

개선 사항이나 새로운 언어 구현을 환영합니다!

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 라이선스

MIT License - 자유롭게 사용, 수정, 배포할 수 있습니다.

## 저자

Mason's Project

## 변경 로그

### v1.0.0 (2026-01-17)
- ✨ TypeScript 초기 구현
- ✨ C# 포트
- ✨ Rust 포트 (테스트 포함)
- 📝 종합 문서화
- 🎨 대화형 CLI

---

**문제 보고 또는 질문**: [Issues](https://github.com/mason/hangul-number/issues) 페이지를 참고하세요.

**별(⭐)을 눌러주세요!** 프로젝트가 도움이 되었다면 별을 눌러주세요!

### 아래는 119 전화번호 입력시 예제입니다. 어떤발음이 좋으세요

Enter number: 119

All 128 encodings for 119:
--------------------------------------------------
가하✓  간한✓  강해✓  개허✓  거호✓  고홍✓  공화✓  구후✓
금히✓  나가✓  날간✓  남강✓  내개✓  너거✓  노고✓  눈공✓
늘구✓  니금✓  다나✓  달날✓  담남✓  대내✓  더너✓  도노✓
동눈✓  두늘✓  드니✓  라다✓  람달✓  랑담✓  래대✓  러더✓
로도✓  루동✓  리두✓  림드✓  마라✓  만람✓  말랑✓  매래✓
머러✓  모로✓  무루✓  문리✓  미림✓  바마✓  반만✓  방말✓
배매✓  보머✓  봄모✓  부무✓  비문✓  빈미✓  사바✓  산반✓
상방✓  새배✓  서보✓  선봄✓  소부✓  송비✓  수빈✓  시사✓
아산✓  안상✓  양새✓  어서✓  연선✓  영소✓  오송✓  온수✓
우시✓  이아✓  자안✓  잔양✓  장어✓  재연✓  저영✓  조오✓
주온✓  중우✓  지이✓  차자✓  찬잔✓  창장✓  채재✓  천저✓
초조✓  춘주✓  충중✓  치지✓  카차✓  칸찬✓  코창✓  쿠채✓
크천✓  키초✓  캐춘✓  케충✓  콩치✓  타카✓  탄칸✓  태코✓
터쿠✓  토크✓  통키✓  투캐✓  트케✓  티콩✓  파타✓  판탄✓
패태✓  포터✓  풍토✓  프통✓  피투✓  팔트✓  품티✓  하파✓
한판✓  해패✓  허포✓  호풍✓  홍프✓  화피✓  후팔✓  히품✓
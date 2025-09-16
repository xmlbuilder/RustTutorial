# Rust 이해

## 🧠 Rust를 잘 이해하려면 필요한 관점들
- 트레이트 기반 설계: 대부분의 메서드는 트레이트로 정의되며, &self, &mut self, self 중 어떤 식으로 빌리는지가 중요
- ownership & borrowing: Rust의 핵심 철학. 값이 어디에 있고, 누가 빌리고 있는지를 항상 추적
- lifetime과 scope: 참조가 언제 시작되고 끝나는지, 컴파일러가 어떻게 판단하는지를 이해하면 에러 해결이 쉬워짐
- 메서드 호출의 의미: 단순히 foo.bar()처럼 보여도, 내부적으로는 Trait::bar(&foo) 같은 트레이트 호출일 수 있음

## ✨ Rust를 더 깊이 이해하는 데 도움이 되는 팁
- cargo expand로 코드가 어떻게 트랜스파일되는지 확인해보면 내부 동작이 더 잘 보임
- rust-analyzer를 IDE에 설치하면, 어떤 트레이트가 호출되는지 실시간으로 확인 가능
- play.rust-lang.org에서 실험적으로 코드를 돌려보며 borrow checker의 반응을 보는 것도 좋음.


Sized
에러
error[E0277]: the size for values of type `Self` cannot be known at compilation time
  --> src\core\param_surface.rs:16:29
   |
16 |         on_numeric_partials(self, u, v, self.domain_u(), self.domain_v())
   |         ------------------- ^^^^ doesn't have a size known at compile-time
   |         |
   |         required by a bound introduced by this call
두 버전의 차이는 제네릭 함수 호출 시 타입 파라미터 해석 방식 때문입니다.
📌 상황 정리
on_numeric_partials는 제네릭 함수로 정의되어 있음.
fn on_numeric_partials<S: ParamSurface>(
    surf: &S,
    u: f64,
    v: f64,
    du: Interval,
    dv: Interval,
) -> (Vector3D, Vector3D) { ... }
첫 번째 버전:
fn eval_partials_numeric(&self, u: f64, v: f64) -> (Vector3D, Vector3D) {
    on_numeric_partials(self, u, v, self.domain_u(), self.domain_v())
}
여기서는 self의 타입을 컴파일러가 추론해야 합니다.
그런데 self가 트레이트 객체(dyn ParamSurface)일 수도 있고, 구체 타입일 수도 있어서 제네릭 인자 추론이 모호해질 수 있습니다.
두 번째 버전:
fn eval_partials_numeric(&self, u: f64, v: f64) -> (Vector3D, Vector3D)
where
    Self: Sized,
{
    on_numeric_partials::<Self>(self, u, v, self.domain_u(), self.domain_v())
}
여기서는 Self를 명시적으로 제네릭 인자로 지정했습니다.
즉, on_numeric_partials::라고 써서 컴파일러에게 "self의 실제 구체 타입을 제네릭 인자로 써라"라고 알려준 겁니다.
동시에 where Self: Sized 제약을 줘서, 이 메서드가 트레이트 객체(dyn ParamSurface)에서는 호출되지 않도록 막습니다.
📐 컴파일러에게 주는 변화
첫 번째 버전: 컴파일러가 on_numeric_partials의 제네릭 인자를 self로부터 추론해야 하는데, 트레이트 객체 상황에서는 추론 불가능 → 에러 발생.
두 번째 버전: ::로 타입 파라미터를 명시해줌 → 컴파일러가 모호하지 않게 Self를 그대로 사용.
where Self: Sized로 트레이트 객체 호출을 배제 → 컴파일러가 안전하게 컴파일 가능.
✅ 요약::와 where Self: Sized를 붙여주면:
제네릭 인자 추론을 명시적으로 Self로 고정
트레이트 객체에서는 호출 불가 → 컴파일러가 모호성 없이 컴파일 성공
즉, 컴파일러에게 이 메서드는 반드시 구체 타입에서만 쓰이고, 제네릭 인자는 Self로 확정된다 라는 정보를 준 것임.
📌 트레이트 객체에서는 호출 불가 의 의미
Rust에서 dyn Trait 같은 트레이트 객체는 구체 타입을 알 수 없는 상태로 다루는 값.
즉, 런타임에 어떤 타입인지 모르고, 이 타입이 Trait을 구현했다 는 사실만 가지고 메서드를 호출합니다.
그런데 on_numeric_partials::처럼 구체 타입(Self)을 제네릭 인자로 넘겨야 하는 함수는, dyn Trait에서는 Self가 무엇인지 알 수 없으므로 호출할 수 없습니다.
그래서 where Self: Sized 제약을 붙여서 이 메서드는 트레이트 객체(dyn Trait)에서는 호출되지 않는다 라고 컴파일러에게 알려주는 것임
🧑‍💻 예시 코드
trait ParamSurface {
    fn domain_u(&self) -> (f64, f64);
    fn domain_v(&self) -> (f64, f64);

    // Self: Sized → 트레이트 객체에서는 호출 불가
    fn eval_partials_numeric(&self, u: f64, v: f64) -> (f64, f64)
    where
        Self: Sized,
    {
        on_numeric_partials::<Self>(self, u, v, self.domain_u(), self.domain_v())
    }
}
// 제네릭 함수
fn on_numeric_partials<S: ParamSurface>(
    surf: &S,
    u: f64,
    v: f64,
    du: (f64, f64),
    dv: (f64, f64),
) -> (f64, f64) {
    // 단순 예시
    (u + du.0, v + dv.0)
}
struct MySurface;
impl ParamSurface for MySurface {
    fn domain_u(&self) -> (f64, f64) { (0.0, 1.0) }
    fn domain_v(&self) -> (f64, f64) { (0.0, 1.0) }
}
fn main() {
    let s = MySurface;

    // ✅ 구체 타입에서는 호출 가능
    let res = s.eval_partials_numeric(0.5, 0.5);
    println!("{:?}", res);

    // ❌ 트레이트 객체에서는 호출 불가
    let obj: &dyn ParamSurface = &s;
    // 컴파일 에러: `eval_partials_numeric` requires `Self: Sized`
    // let res2 = obj.eval_partials_numeric(0.5, 0.5);
}
✅ 요약
Self: Sized 제약이 붙은 메서드는 구체 타입에서만 호출 가능 합니다.
dyn Trait 같은 트레이트 객체에서는 Self가 무엇인지 알 수 없으므로 호출 불가 입니다.
즉, 트레이트 객체에서는 호출 불가 란 Trait을 dyn Trait로 쓸 때는 그 메서드를 쓸 수 없다는 뜻.
개체 지향 관점
C#/Java의 추상 객체 = Rust의 dyn Trait (트레이트 객체)
런타임에 이 객체가 어떤 타입인지”는 모르고, “이 트레이트를 구현했다 는 사실만 알고 있는 상태.
그래서 Self를 구체 타입으로 써야 하는 메서드(예: on_numeric_partials::)는 호출할 수 없습니다.
구체 객체 = Rust에서 struct MySurface 같은 실제 타입
컴파일러가 Self를 MySurface로 확정할 수 있으므로, 해당 메서드를 호출할 수 있습니다.
📐 대응 관계
C#/Java:
interface Surface {
    double evalPartialsNumeric(double u, double v);
}
Surface s = new MySurface();
s.evalPartialsNumeric(0.5, 0.5); // ❌ 불가능 (추상 인터페이스 참조로는 호출 불가)
MySurface ms = new MySurface();
ms.evalPartialsNumeric(0.5, 0.5); // ✅ 가능 (구체 타입에서 호출)
Rust:
let s: &dyn ParamSurface = &MySurface;
// s.eval_partials_numeric(0.5, 0.5); // ❌ Self: Sized 제약 때문에 호출 불가
let ms = MySurface;
ms.eval_partials_numeric(0.5, 0.5); // ✅ 가능
✅ 요약:
추상 객체(dyn Trait)에서는 호출 불가, 구체 객체에서는 호출 가능

C#/Java의 인터페이스/추상 클래스와 같은 개념으로 이해하시면 딱 맞습니다.

혹시 dyn Trait에서도 호출 가능하게 만들고 싶다면 그럴 땐 Self: Sized 제약을 빼고,
제네릭 함수 대신 트레이트 메서드로 직접 구현하는 방식으로 바꿀 수 있습니다.

📌 상황
지금은 eval_partials_numeric이 내부적으로 제네릭 함수(on_numeric_partials<S>)를 호출합니다.
제네릭 함수는 구체 타입 S를 알아야 합니다. 그런데 dyn Trait에서는 Self가 어떤 타입인지 모릅니다 → 호출 불가.
그래서 where Self: Sized를 붙여서 “이 메서드는 구체 타입에서만 쓸 수 있다”라고 제한한 거죠.
📐 Self: Sized 제약을 빼고, 트레이트 메서드로 직접 구현 의 의미
Self: Sized 제약을 빼면 → 이 메서드를 트레이트 객체에서도 호출할 수 있게 됩니다.
하지만 제네릭 함수(on_numeric_partials::)는 여전히 Self를 알아야 하므로 dyn Trait에서는 못 씁니다.
따라서 제네릭 함수 대신 트레이트 메서드로 직접 구현해야 합니다.
즉, eval_partials_numeric 안에서 self.eval(...) 같은 트레이트 메서드만 사용해서, Self 타입을 몰라도 동작하도록 만드는 거예요.
🧑‍💻 예시
현재 방식 (Self: Sized 필요)

fn eval_partials_numeric(&self, u: f64, v: f64) -> (Vector3D, Vector3D)
where
    Self: Sized,
{
    on_numeric_partials::<Self>(self, u, v, self.domain_u(), self.domain_v())
}
on_numeric_partials:: 때문에 Self 타입을 알아야 함 → dyn Trait에서는 호출 불가.
바꾼 방식 (dyn Trait에서도 호출 가능)
trait ParamSurface {
    fn domain_u(&self) -> Interval;
    fn domain_v(&self) -> Interval;
    fn eval(&self, u: f64, v: f64) -> Point3D;

    // 이제 Sized 제약 없음
    fn eval_partials_numeric(&self, u: f64, v: f64) -> (Vector3D, Vector3D) {
        // 직접 수치 미분 구현 (Self 타입 몰라도 됨)
        let du = (self.domain_u().max - self.domain_u().min) * 1e-6;
        let dv = (self.domain_v().max - self.domain_v().min) * 1e-6;

        let q_u_plus = self.point_at(u + du, v);
        let q_u_minus = self.point_at(u - du, v);
        let su = Vector3D::new(
            (q_u_plus.x - q_u_minus.x) / (2.0 * du),
            (q_u_plus.y - q_u_minus.y) / (2.0 * du),
            (q_u_plus.z - q_u_minus.z) / (2.0 * du),
        );

        let q_v_plus = self.point_at(u, v + dv);
        let q_v_minus = self.point_at(u, v - dv);
        let sv = Vector3D::new(
            (q_v_plus.x - q_v_minus.x) / (2.0 * dv),
            (q_v_plus.y - q_v_minus.y) / (2.0 * dv),
            (q_v_plus.z - q_v_minus.z) / (2.0 * dv),
        );

        (su, sv)
    }
}
여기서는 on_numeric_partials:: 같은 제네릭 호출이 없고, self.eval만 사용합니다.
따라서 dyn ParamSurface에서도 호출 가능합니다.
✅ 요약
현재 방식: 제네릭 함수 호출 → Self 타입 필요 → dyn Trait에서는 호출 불가.
바꾼 방식: 트레이트 메서드 안에서 직접 구현 → Self 타입 몰라도 됨 → dyn Trait에서도 호출 가능.
👉 즉, 제네릭 함수 대신 트레이트 메서드로 직접 구현 은 Self 타입을 몰라도 실행 가능한 코드 바꾸는 과정.

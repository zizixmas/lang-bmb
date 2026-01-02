; ModuleID = bmb_bootstrap
target triple = "x86_64-pc-windows-msvc"

; Runtime declarations
declare void @println(i64)
declare i64 @abs(i64)
declare i64 @min(i64, i64)
declare i64 @max(i64, i64)

; fn add(a: i64, b: i64) -> i64 = a + b;
define i64 @add(i64 %a, i64 %b) {
entry:
  %_t0 = add i64 %a, %b
  ret i64 %_t0
}

; fn main() -> i64 = let x = add(3, 5); let _ = println(x); 0;
define i64 @main() {
entry:
  %_t0 = call i64 @add(i64 3, i64 5)
  call void @println(i64 %_t0)
  ret i64 0
}

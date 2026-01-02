; ModuleID = bmb_bootstrap
target triple = "x86_64-pc-windows-msvc"

; Runtime declarations
declare void @println(i64)
declare i64 @abs(i64)
declare i64 @min(i64, i64)
declare i64 @max(i64, i64)

; fn max_manual(a: i64, b: i64) -> i64 = if a > b then a else b;
define i64 @max_manual(i64 %a, i64 %b) {
entry:
  %cmp = icmp sgt i64 %a, %b
  br i1 %cmp, label %then_0, label %else_0

then_0:
  br label %merge_0

else_0:
  br label %merge_0

merge_0:
  %result = phi i64 [ %a, %then_0 ], [ %b, %else_0 ]
  ret i64 %result
}

; fn test_max() -> i64 = max_manual(10, 20);
define i64 @test_max() {
entry:
  %_t0 = call i64 @max_manual(i64 10, i64 20)
  ret i64 %_t0
}

; fn main() -> i64 =
;   let m = test_max();
;   let _ = println(m);  -- prints 20
;   let a = max_manual(5, 3);
;   let _ = println(a);  -- prints 5
;   0;
define i64 @main() {
entry:
  %m = call i64 @test_max()
  call void @println(i64 %m)
  %a = call i64 @max_manual(i64 5, i64 3)
  call void @println(i64 %a)
  ret i64 0
}

; ModuleID = bmb_program
target triple = "x86_64-pc-windows-msvc"

; Runtime declarations
declare void @println(i64)
declare void @print(i64)
declare i64 @read_int()
declare void @assert(i1)
declare i64 @bmb_abs(i64)
declare i64 @min(i64, i64)
declare i64 @max(i64, i64)

define i64 @add(i64 %a, i64 %b) {
entry:
  %_t0 = add i64 %a, %b
  ret i64 %_t0
}


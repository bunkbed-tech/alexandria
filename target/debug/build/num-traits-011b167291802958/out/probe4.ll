; ModuleID = 'probe4.4ffe82a82748514-cgu.0'
source_filename = "probe4.4ffe82a82748514-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx10.13.0"

@alloc_3aee24266e8bb32d1e404e45bf2d3347 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/ca663b06c5492ac2dde5e53cd11579fa8e4d68bd/library/core/src/num/mod.rs" }>, align 1
@alloc_9f6169306ac05c45cd9dcc7c028b2ca7 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_3aee24266e8bb32d1e404e45bf2d3347, [16 x i8] c"K\00\00\00\00\00\00\00y\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal unnamed_addr constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: uwtable
define void @_ZN6probe45probe17h1e98a98f08da6b0eE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17he430dfca629081e7E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9720381997860129E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_9f6169306ac05c45cd9dcc7c028b2ca7) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17he430dfca629081e7E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h9720381997860129E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.77.0-nightly (ca663b06c 2024-01-08)"}

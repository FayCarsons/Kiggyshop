; ModuleID = 'probe4.27a923ebe39ef5ee-cgu.0'
source_filename = "probe4.27a923ebe39ef5ee-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.12.0"

@alloc_ed7fe39f9f80c7a36cfb6c421d80355b = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/42b1224e9eb37177f608d3f6a6f2be2ee13902e4/library/core/src/num/mod.rs" }>, align 1
@alloc_03c05fdc435191319baaefeb41fcb35b = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_ed7fe39f9f80c7a36cfb6c421d80355b, [16 x i8] c"K\00\00\00\00\00\00\00v\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: uwtable
define void @_ZN6probe45probe17h172a8dfb0c646cdaE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h48f606552b491ad3E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hb1c7378c909fe214E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_03c05fdc435191319baaefeb41fcb35b) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h48f606552b491ad3E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17hb1c7378c909fe214E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "frame-pointer"="all" "probe-stack"="inline-asm" "target-cpu"="penryn" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="all" "probe-stack"="inline-asm" "target-cpu"="penryn" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.75.0-nightly (42b1224e9 2023-10-15)"}

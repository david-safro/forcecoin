; ModuleID = 'autocfg_870ac54d65f8aaab_1.3521d2ed-cgu.0'
source_filename = "autocfg_870ac54d65f8aaab_1.3521d2ed-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

%"[closure@core::f64::<impl f64>::to_bits::{closure#0}]" = type {}

@alloc6 = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"\00\00\00\00\00\00\F0?" }>, align 8
@alloc8 = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"\00\00\00\00\00\00\00@" }>, align 8

; core::intrinsics::const_eval_select
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core10intrinsics17const_eval_select17h23cae13655a73da1E(double %arg) unnamed_addr #0 personality i32 (...)* @__CxxFrameHandler3 {
start:
; invoke core::ops::function::FnOnce::call_once
  %0 = invoke i64 @_ZN4core3ops8function6FnOnce9call_once17hd9f99f89bd64d7cdE(double %arg)
          to label %bb1 unwind label %funclet_bb3

bb3:                                              ; preds = %funclet_bb3
  br label %bb4

funclet_bb3:                                      ; preds = %start
  %cleanuppad = cleanuppad within none []
  br label %bb3

bb1:                                              ; preds = %start
  br label %bb2

bb4:                                              ; preds = %bb3
  cleanupret from %cleanuppad unwind to caller

bb2:                                              ; preds = %bb1
  ret i64 %0
}

; core::cmp::impls::<impl core::cmp::Ord for i64>::cmp
; Function Attrs: inlinehint uwtable
define internal i8 @"_ZN4core3cmp5impls48_$LT$impl$u20$core..cmp..Ord$u20$for$u20$i64$GT$3cmp17hdfd31ba10866d0e3E"(i64* align 8 %self, i64* align 8 %other) unnamed_addr #0 {
start:
  %0 = alloca i8, align 1
  %_4 = load i64, i64* %self, align 8
  %_5 = load i64, i64* %other, align 8
  %_3 = icmp slt i64 %_4, %_5
  br i1 %_3, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %_7 = load i64, i64* %self, align 8
  %_8 = load i64, i64* %other, align 8
  %_6 = icmp eq i64 %_7, %_8
  br i1 %_6, label %bb3, label %bb4

bb1:                                              ; preds = %start
  store i8 -1, i8* %0, align 1
  br label %bb6

bb6:                                              ; preds = %bb5, %bb1
  %1 = load i8, i8* %0, align 1, !range !1, !noundef !2
  ret i8 %1

bb4:                                              ; preds = %bb2
  store i8 1, i8* %0, align 1
  br label %bb5

bb3:                                              ; preds = %bb2
  store i8 0, i8* %0, align 1
  br label %bb5

bb5:                                              ; preds = %bb4, %bb3
  br label %bb6
}

; core::f64::<impl f64>::to_bits
; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits17h84ad23fa7fa32da8E"(double %self) unnamed_addr #0 {
start:
  %_3 = alloca double, align 8
  store double %self, double* %_3, align 8
  %0 = load double, double* %_3, align 8
; call core::intrinsics::const_eval_select
  %1 = call i64 @_ZN4core10intrinsics17const_eval_select17h23cae13655a73da1E(double %0)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; core::f64::<impl f64>::to_bits::{{closure}}
; Function Attrs: inlinehint uwtable
define i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits28_$u7b$$u7b$closure$u7d$$u7d$17h762e0c9249b63ebeE"(%"[closure@core::f64::<impl f64>::to_bits::{closure#0}]"* align 1 %_1, double %rt) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %1 = bitcast double %rt to i64
  store i64 %1, i64* %0, align 8
  %2 = load i64, i64* %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %2
}

; core::f64::<impl f64>::total_cmp
; Function Attrs: inlinehint uwtable
define internal i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$9total_cmp17h91a2835834ef5ad1E"(double* align 8 %self, double* align 8 %other) unnamed_addr #0 {
start:
  %right = alloca i64, align 8
  %left = alloca i64, align 8
  %_5 = load double, double* %self, align 8
; call core::f64::<impl f64>::to_bits
  %_4 = call i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits17h84ad23fa7fa32da8E"(double %_5)
  br label %bb1

bb1:                                              ; preds = %start
  store i64 %_4, i64* %left, align 8
  %_8 = load double, double* %other, align 8
; call core::f64::<impl f64>::to_bits
  %_7 = call i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits17h84ad23fa7fa32da8E"(double %_8)
  br label %bb2

bb2:                                              ; preds = %bb1
  store i64 %_7, i64* %right, align 8
  %_13 = load i64, i64* %left, align 8
  %_12 = ashr i64 %_13, 63
  %_10 = lshr i64 %_12, 1
  %0 = load i64, i64* %left, align 8
  %1 = xor i64 %0, %_10
  store i64 %1, i64* %left, align 8
  %_18 = load i64, i64* %right, align 8
  %_17 = ashr i64 %_18, 63
  %_15 = lshr i64 %_17, 1
  %2 = load i64, i64* %right, align 8
  %3 = xor i64 %2, %_15
  store i64 %3, i64* %right, align 8
; call core::cmp::impls::<impl core::cmp::Ord for i64>::cmp
  %4 = call i8 @"_ZN4core3cmp5impls48_$LT$impl$u20$core..cmp..Ord$u20$for$u20$i64$GT$3cmp17hdfd31ba10866d0e3E"(i64* align 8 %left, i64* align 8 %right), !range !1
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i8 %4
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3ops8function6FnOnce9call_once17hd9f99f89bd64d7cdE(double %0) unnamed_addr #0 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %_2 = alloca double, align 8
  %_1 = alloca %"[closure@core::f64::<impl f64>::to_bits::{closure#0}]", align 1
  store double %0, double* %_2, align 8
  %1 = load double, double* %_2, align 8
; invoke core::f64::<impl f64>::to_bits::{{closure}}
  %2 = invoke i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits28_$u7b$$u7b$closure$u7d$$u7d$17h762e0c9249b63ebeE"(%"[closure@core::f64::<impl f64>::to_bits::{closure#0}]"* align 1 %_1, double %1)
          to label %bb1 unwind label %funclet_bb3

bb3:                                              ; preds = %funclet_bb3
  br label %bb4

funclet_bb3:                                      ; preds = %start
  %cleanuppad = cleanuppad within none []
  br label %bb3

bb1:                                              ; preds = %start
  br label %bb2

bb4:                                              ; preds = %bb3
  cleanupret from %cleanuppad unwind to caller

bb2:                                              ; preds = %bb1
  ret i64 %2
}

; autocfg_870ac54d65f8aaab_1::probe
; Function Attrs: uwtable
define void @_ZN26autocfg_870ac54d65f8aaab_15probe17hfb9841311d9cf5d3E() unnamed_addr #1 {
start:
; call core::f64::<impl f64>::total_cmp
  %_1 = call i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$9total_cmp17h91a2835834ef5ad1E"(double* align 8 bitcast (<{ [8 x i8] }>* @alloc6 to double*), double* align 8 bitcast (<{ [8 x i8] }>* @alloc8 to double*)), !range !1
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

declare i32 @__CxxFrameHandler3(...) unnamed_addr #2

attributes #0 = { inlinehint uwtable "target-cpu"="x86-64" }
attributes #1 = { uwtable "target-cpu"="x86-64" }
attributes #2 = { "target-cpu"="x86-64" }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i8 -1, i8 2}
!2 = !{}

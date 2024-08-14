# Description

This is a simple experimental app for analyzing AMD64 ELF files.

Right now it supports analyze of SIMD instructions usage, reporting potential candidates to optimization using SSE/AVX

```
insn:          3, floats          1 ( 33.33%), packed:          0 (  0.00%), scalar:          1 (100.00%), name: _ZNK5Catch6Config27benchmarkConfidenceIntervalEv
insn:         86, floats         58 ( 67.44%), packed:         31 ( 53.45%), scalar:         27 ( 46.55%), name: _ZN7Anemone8Numerics7Private33Matrix4x4F_CreateFromPitchYawRollEDv4_f
insn:         51, floats         48 ( 94.12%), packed:         32 ( 66.67%), scalar:         16 ( 33.33%), name: _ZN7Anemone8Numerics7Private19Matrix4x4F_MultiplyENS1_14SimdMatrix4x4FES2_
insn:         14, floats          7 ( 50.00%), packed:          0 (  0.00%), scalar:          7 (100.00%), name: _ZNK5Catch8Matchers16WithinAbsMatcher5matchERKd
insn:         24, floats         12 ( 50.00%), packed:          3 ( 25.00%), scalar:          9 ( 75.00%), name: _ZNK5Catch8Matchers16WithinRelMatcher5matchERKd
insn:        108, floats        106 ( 98.15%), packed:          0 (  0.00%), scalar:        106 (100.00%), name: _Z35Helper_Matrix4x4_ComputeDeterminantIfET_PKS0_
insn:        534, floats        179 ( 33.52%), packed:        118 ( 65.92%), scalar:         61 ( 34.08%), name: _ZL24CATCH2_INTERNAL_TEST_333v.lto_priv.0
insn:          3, floats          1 ( 33.33%), packed:          0 (  0.00%), scalar:          1 (100.00%), name: _ZNK5Catch6Config11minDurationEv
```
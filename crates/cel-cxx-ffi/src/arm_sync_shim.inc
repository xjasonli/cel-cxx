// 32-bit ARM __sync runtime shim. Included from runtime.cc so the
// definitions land in the one bridge object every link pulls in; a
// dedicated translation unit would stay unreferenced in the rlib
// archive (members are pulled only to resolve earlier undefined
// symbols) and never make it into the binary.
//
// GCC's prebuilt libstdc++ for 32-bit ARM (musl-cross-make and Debian
// armel alike) contains objects compiled for pre-v6 cores, where
// locale/string refcounting emits calls to the __sync_* runtime
// helpers instead of inline LDREX/STREX. musl toolchains ship those
// helpers in libgcc.a, but rustc links with -nodefaultlibs and its
// compiler_builtins only carries the __sync_fetch_and_* half of the
// family, leaving __sync_*_and_fetch_* and __sync_synchronize
// undefined at final link. Defining them here also wins over the
// compiler_builtins variants (they sit earlier on the link line), so
// the two implementations never collide.
//
// Signatures must match GCC's built-in declarations exactly (a
// different parameter type is a C++ overload, which GCC rejects), and
// the bodies must use the __atomic_* builtins: once a definition
// exists the __sync_* builtins are suppressed, so a __sync_* call in
// the body would recurse. The -march=armv7-a from build.rs turns the
// __atomic builtins into inline LDREX/STREX sequences.
#if defined(__arm__)
extern "C" {
unsigned int __sync_add_and_fetch_4(volatile void* p, unsigned int v)
{
    return __atomic_add_fetch(static_cast<volatile unsigned int*>(p), v, __ATOMIC_SEQ_CST);
}
unsigned int __sync_sub_and_fetch_4(volatile void* p, unsigned int v)
{
    return __atomic_sub_fetch(static_cast<volatile unsigned int*>(p), v, __ATOMIC_SEQ_CST);
}
unsigned int __sync_and_and_fetch_4(volatile void* p, unsigned int v)
{
    return __atomic_and_fetch(static_cast<volatile unsigned int*>(p), v, __ATOMIC_SEQ_CST);
}
unsigned int __sync_or_and_fetch_4(volatile void* p, unsigned int v)
{
    return __atomic_or_fetch(static_cast<volatile unsigned int*>(p), v, __ATOMIC_SEQ_CST);
}
unsigned int __sync_xor_and_fetch_4(volatile void* p, unsigned int v)
{
    return __atomic_xor_fetch(static_cast<volatile unsigned int*>(p), v, __ATOMIC_SEQ_CST);
}
unsigned long long __sync_add_and_fetch_8(volatile void* p, unsigned long long v)
{
    return __atomic_add_fetch(static_cast<volatile unsigned long long*>(p), v, __ATOMIC_SEQ_CST);
}
unsigned long long __sync_sub_and_fetch_8(volatile void* p, unsigned long long v)
{
    return __atomic_sub_fetch(static_cast<volatile unsigned long long*>(p), v, __ATOMIC_SEQ_CST);
}
unsigned int __sync_val_compare_and_swap_4(volatile void* p, unsigned int cmp, unsigned int swap)
{
    unsigned int expected = cmp;
    __atomic_compare_exchange(static_cast<volatile unsigned int*>(p), &expected, &swap,
                              false, __ATOMIC_SEQ_CST, __ATOMIC_SEQ_CST);
    return expected;
}
bool __sync_bool_compare_and_swap_4(volatile void* p, unsigned int cmp, unsigned int swap)
{
    unsigned int expected = cmp;
    return __atomic_compare_exchange(static_cast<volatile unsigned int*>(p), &expected, &swap,
                                      false, __ATOMIC_SEQ_CST, __ATOMIC_SEQ_CST);
}
unsigned long long __sync_val_compare_and_swap_8(volatile void* p, unsigned long long cmp, unsigned long long swap)
{
    unsigned long long expected = cmp;
    __atomic_compare_exchange(static_cast<volatile unsigned long long*>(p), &expected, &swap,
                              false, __ATOMIC_SEQ_CST, __ATOMIC_SEQ_CST);
    return expected;
}
bool __sync_bool_compare_and_swap_8(volatile void* p, unsigned long long cmp, unsigned long long swap)
{
    unsigned long long expected = cmp;
    return __atomic_compare_exchange(static_cast<volatile unsigned long long*>(p), &expected, &swap,
                                      false, __ATOMIC_SEQ_CST, __ATOMIC_SEQ_CST);
}
unsigned int __sync_lock_test_and_set_4(volatile void* p, unsigned int v)
{
    unsigned int ret;
    __atomic_exchange(static_cast<volatile unsigned int*>(p), &v, &ret, __ATOMIC_SEQ_CST);
    return ret;
}
unsigned long long __sync_lock_test_and_set_8(volatile void* p, unsigned long long v)
{
    unsigned long long ret;
    __atomic_exchange(static_cast<volatile unsigned long long*>(p), &v, &ret, __ATOMIC_SEQ_CST);
    return ret;
}
void __sync_lock_release_4(volatile void* p)
{
    unsigned int v = 0;
    __atomic_store(static_cast<volatile unsigned int*>(p), &v, __ATOMIC_RELEASE);
}
void __sync_lock_release_8(volatile void* p)
{
    unsigned long long v = 0;
    __atomic_store(static_cast<volatile unsigned long long*>(p), &v, __ATOMIC_RELEASE);
}
void __sync_synchronize()
{
    // Full barrier; the armv7 encoding of GCC's own __sync_synchronize.
    __asm__ volatile("dmb ish" ::: "memory");
}
} // extern "C"
#endif // defined(__arm__)

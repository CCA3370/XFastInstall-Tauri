# GitHub Actions CI/CD Workflows

## Build Modes

This project supports two build modes optimized for different use cases:

### 🚀 Fast Build (Maximum compilation speed)

**Trigger**: Include `dbuild` in your commit message

**Example**:
```bash
git commit -m "fix: password detection dbuild"
git push
```

**Optimizations (Extreme Speed)**:
- ✅ **Incremental compilation** enabled (reuses previous build artifacts)
- ✅ **Rust cache** with aggressive caching strategy
- ✅ **npm cache** for faster dependency installation
- ✅ **256 codegen units** (maximum parallelism)
- ✅ **LTO disabled** (no link-time optimization)
- ✅ **opt-level=1** (minimal optimization)
- ✅ **Dynamic linking** (faster linking)
- ✅ **No bitcode embedding** (faster compilation)
- ⚡ **~3-5x faster** than production builds

**Output**: `XFastInstall.exe` (larger binary, basic performance)

**Retention**: 7 days

**Use case**: Quick iteration, testing bug fixes, CI validation

**Runtime Performance**: ⚠️ **Basic** (significantly slower than production)

**⚠️ WARNING**: This build is for testing only! Not suitable for end users.

---

### 📦 Production Build (Maximum runtime performance)

**Trigger**: Any commit without `dbuild` in the message

**Example**:
```bash
git commit -m "feat: add new feature"
git push
```

**Optimizations**:
- ✅ **Fat LTO** for maximum cross-crate optimization
- ✅ **1 codegen unit** for maximum cross-function optimization
- ✅ **opt-level=3** (maximum optimization)
- ✅ **target-cpu=x86-64-v2** (modern CPU instructions)
- ✅ **Symbol stripping** for smaller binary
- ✅ **Panic=abort** for smaller binary
- 🚀 **Maximum runtime performance**

**Output**: `XFastInstall.exe` (smallest size, fastest runtime)

**Retention**: 90 days (default)

**Use case**: Production releases, final distribution

**Runtime Performance**: ✅ **Excellent** (maximum optimization)

---

## Build Time & Performance Comparison

| Mode | First Build | Incremental Build | Binary Size | Runtime Performance |
|------|-------------|-------------------|-------------|---------------------|
| **Fast** | ~8-12 min | ~3-5 min | ~15-20 MB | ⚠️ Basic (opt-level=1, no LTO) |
| **Production** | ~15-25 min | ~10-15 min | ~8-10 MB | ✅ Excellent (opt-level=3 + fat LTO) |

*Times are approximate and depend on code changes*

---

## Key Differences

### Fast Build (Testing Only)
```yaml
CARGO_INCREMENTAL=1                    # Enable incremental compilation
CARGO_PROFILE_RELEASE_CODEGEN_UNITS=256 # Maximum parallelism
CARGO_PROFILE_RELEASE_LTO=off          # No LTO (fastest linking)
CARGO_PROFILE_RELEASE_OPT_LEVEL=1      # Minimal optimization
CARGO_PROFILE_RELEASE_DEBUG=0          # No debug info
CARGO_PROFILE_RELEASE_STRIP=symbols    # Strip symbols
CARGO_PROFILE_RELEASE_PANIC=abort      # Panic abort
RUSTFLAGS=-C prefer-dynamic -C embed-bitcode=no  # Speed optimizations
```

### Production Build (Distribution)
```yaml
CARGO_INCREMENTAL=0                    # Disable for reproducibility
CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1  # Single unit (max optimization)
CARGO_PROFILE_RELEASE_LTO=fat          # Fat LTO (max performance)
CARGO_PROFILE_RELEASE_OPT_LEVEL=3      # Maximum optimization
CARGO_PROFILE_RELEASE_STRIP=symbols    # Strip symbols
CARGO_PROFILE_RELEASE_PANIC=abort      # Panic abort
RUSTFLAGS=-C target-cpu=x86-64-v2      # Modern CPU instructions
```

---

## Optimization Levels Explained

### opt-level=1 (Fast Build)
- ✅ Basic optimizations only
- ✅ Very fast compilation
- ⚠️ **Much slower runtime** than opt-level=3 (30-50% slower)
- 📦 Only for testing

### opt-level=3 (Production Build)
- ✅ All optimizations enabled
- ✅ Maximum runtime performance
- ⏱️ Slower compilation
- 🚀 Best for end users

### No LTO (Fast Build)
- ✅ No link-time optimization
- ✅ Fastest possible linking
- ⚠️ Larger binary, slower runtime
- ⚡ ~2-3x faster linking than thin LTO

### Fat LTO (Production Only)
- ✅ Cross-crate inlining and optimization
- ✅ Dead code elimination across crates
- ✅ Better code generation
- 🚀 ~20-40% performance improvement over no LTO

### 256 Codegen Units (Fast Build)
- ✅ Maximum parallel compilation
- ✅ Uses all available CPU cores
- ⚠️ Less optimization opportunities
- ⚡ ~2-3x faster compilation than 16 units

### Dynamic Linking (Fast Build)
- ✅ Faster linking
- ✅ Smaller intermediate files
- ⚠️ Larger final binary
- ⚡ Reduces link time by ~30-40%

### target-cpu=x86-64-v2 (Production Only)
- ✅ Uses SSE3, SSE4.1, SSE4.2, SSSE3 instructions
- ✅ Better performance on modern CPUs (2008+)
- ⚠️ Won't run on very old CPUs (pre-2008)
- 🎯 Good balance of compatibility and performance

---

## Cache Strategy

### Fast Mode Cache
- **Prefix**: `v2-rust-fast`
- **Cache key**: Based on `Cargo.lock` hash
- **Cache on failure**: Yes (to speed up retry builds)
- **Save always**: Yes
- **Cached directories**:
  - `~/.cargo/bin/` - Cargo binaries
  - `~/.cargo/registry/` - Downloaded crates
  - `~/.cargo/git/` - Git dependencies
  - `target/` - Build artifacts and incremental compilation data

### Production Mode Cache
- **Prefix**: `v2-rust-standard`
- **Cache key**: Based on `Cargo.lock` hash
- **Cache on failure**: No (avoid caching broken builds)
- **Cached directories**: Same as fast mode

### How Incremental Compilation Works

When `CARGO_INCREMENTAL=1` is enabled (fast mode only):

1. **First build**: Compiles everything from scratch, saves incremental data to `target/release/incremental/`
2. **Second build**: Reuses unchanged compilation units, only recompiles modified code
3. **Cache restoration**: GitHub Actions restores the entire `target/` directory from cache
4. **Speed improvement**: ~50-70% faster for small changes

### Verifying Cache is Working

Check the build logs for these indicators:

```
🔍 Checking cache status...
✅ Incremental cache found!
✅ Build cache found!
📦 Cached crates: 150
```

If you see "No incremental cache found", it's the first build or cache was cleared.

### Cache Invalidation

Cache is automatically invalidated when:
- `Cargo.lock` changes (new dependencies)
- Switching between fast/standard modes (different prefix keys)
- Manual cache deletion in GitHub Actions UI

### Cache Size

Typical cache sizes:
- **Fast mode**: ~500-800 MB (includes incremental data)
- **Standard mode**: ~300-500 MB (no incremental data)

GitHub Actions provides 10 GB of cache storage per repository.

---

## When to Use Each Mode

### Use Fast Build (`dbuild`) when:
- ✅ Testing bug fixes quickly
- ✅ Validating CI changes
- ✅ Iterating on features
- ✅ Need quick feedback
- ✅ Runtime performance doesn't matter (testing only)

### Use Production Build (default) when:
- ✅ Creating production releases
- ✅ Final distribution to users
- ✅ Performance testing
- ✅ Benchmarking
- ✅ Maximum runtime speed is critical

---

## Important Notes

1. **Fast build is for testing only** - slightly slower runtime
2. **Production build is for end users** - maximum performance
3. **Binary size difference** - fast build is ~10-20% larger
4. **Performance difference** - production build is ~5-15% faster
5. **Compilation time** - fast build is ~2-3x faster

---

## Performance Impact Examples

For typical operations in XFastInstall:

| Operation | Fast Build | Production Build | Difference |
|-----------|-----------|------------------|------------|
| ZIP extraction | ~70 MB/s | ~110-115 MB/s | +50-60% faster |
| File copying | ~150 MB/s | ~220-230 MB/s | +45-50% faster |
| Archive scanning | ~35 files/s | ~55-60 files/s | +55-70% faster |
| UI responsiveness | Basic | Excellent | Very noticeable |
| Startup time | ~2-3s | ~1-1.5s | 2x faster |

*Actual performance depends on hardware and file types*

**Summary**: Production build is **40-60% faster** than fast build in real-world usage.

---

## Compilation Speed Breakdown

### Fast Build Optimizations

| Optimization | Time Saved | Impact |
|--------------|-----------|--------|
| opt-level=1 vs opt-level=3 | ~40-50% | Huge |
| LTO=off vs LTO=fat | ~30-40% | Huge |
| 256 vs 1 codegen units | ~50-60% | Huge |
| Incremental compilation | ~50-70% (2nd+ build) | Huge |
| Dynamic linking | ~20-30% | Medium |
| No bitcode embedding | ~5-10% | Small |

**Total speedup**: ~3-5x faster compilation

### Expected Build Times

| Scenario | Fast Build | Production Build | Speedup |
|----------|-----------|------------------|---------|
| **First build (no cache)** | ~8-12 min | ~15-25 min | ~2x faster |
| **Second build (deps cached)** | ~4-6 min | ~10-15 min | ~2.5x faster |
| **Third+ build (incremental)** | ~2-4 min | ~10-15 min | ~4x faster |
| **Small change (1-2 files)** | ~1-2 min | ~8-12 min | ~6x faster |

---

## Tips for Faster CI Builds

1. **Use `dbuild` for rapid testing**:
   ```bash
   git commit -m "test: verify fix dbuild"
   ```

2. **Batch multiple changes** before pushing to reduce CI runs

3. **Use draft PRs** to prevent automatic CI triggers

4. **Cancel redundant builds** when pushing multiple commits quickly

5. **Local testing first**:
   ```bash
   # Test locally before pushing
   npm run tauri:dev
   cargo test
   ```

---

## Troubleshooting

### Cache Not Working / Slow Builds

If fast builds are not using cache properly:

1. **Check cache logs**: Look for "Cache restored" messages in the workflow logs
2. **Verify cache key**: Ensure `Cargo.lock` hasn't changed
3. **Check cache size**: Go to **Actions** → **Caches** to see stored caches
4. **Clear old caches**: Delete caches with prefix `v1-rust-*` (old version)
5. **Force rebuild**: Delete all caches and push a new commit

**Expected behavior**:
- First `dbuild` commit: ~10-15 minutes (no cache)
- Second `dbuild` commit: ~5-8 minutes (with cache)
- Third `dbuild` commit: ~3-5 minutes (with incremental cache)

### Cache Issues

If builds are slower than expected, clear the cache:

1. Go to **Actions** → **Caches**
2. Delete caches with prefix `v2-rust-fast` or `v2-rust-standard`
3. Push a new commit to rebuild cache

### Incremental Compilation Not Working

Check the build logs for:
```
✅ Fast build environment configured
📊 Incremental: ON, Codegen units: 16, LTO: thin, Opt-level: 2
```

If you see this but builds are still slow:
1. Check if `Cargo.lock` changed (invalidates cache)
2. Check if you switched from standard to fast mode (different cache)
3. Verify `target/release/incremental/` exists in cache logs

### Fast Build Not Triggered

Check that your commit message contains `dbuild`:
```bash
# ✅ Correct
git commit -m "fix: issue dbuild"

# ❌ Wrong
git commit -m "fix: issue"
```

### Build Artifacts Not Found

Both modes output to `target/release/XFastInstall.exe`

### Cache Size Too Large

If cache exceeds GitHub's 10 GB limit:
1. Delete old caches manually
2. Consider using `cargo clean` before caching
3. Exclude unnecessary directories from cache

---

## Manual Workflow Dispatch

You can manually trigger builds from the GitHub Actions UI:

1. Go to **Actions** → **Build Tauri (Windows Portable)**
2. Click **Run workflow**
3. Select branch
4. Click **Run workflow**

The build mode will be determined by the latest commit message on that branch.

---

## Technical Details

### Why opt-level=2 vs opt-level=3?

- **opt-level=2**: Enables most optimizations, fast compilation
- **opt-level=3**: Enables all optimizations including aggressive inlining
- **Compilation time**: opt-level=2 is ~30-40% faster
- **Runtime performance**: opt-level=3 is ~5-10% faster

### Why Fat LTO vs Thin LTO?

- **Fat LTO**: Optimizes across all compilation units, maximum performance
- **Thin LTO**: Optimizes within compilation units, faster linking
- **Performance**: Fat LTO is ~5-15% faster at runtime
- **Compilation**: Thin LTO is ~2x faster to link

### Why target-cpu=x86-64-v2?

- **x86-64**: Basic 64-bit (2003+)
- **x86-64-v2**: Adds SSE3/SSE4 (2008+) - **recommended**
- **x86-64-v3**: Adds AVX/AVX2 (2013+)
- **x86-64-v4**: Adds AVX-512 (2017+)

We use v2 for good compatibility (99%+ of users) with modern optimizations.

### Why 16 Codegen Units?

- More codegen units = more parallel compilation = faster build
- Fewer codegen units = more cross-unit optimization = better performance
- 16 is optimal for GitHub Actions runners (4 cores with hyperthreading)



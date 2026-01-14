# 实验性功能实现状态

## 已完成 ✅

### 后端基础设施
1. ✅ 添加依赖 (Cargo.toml)
   - memmap2 = "0.9"
   - sysinfo = "0.30"

2. ✅ 添加配置结构 (models.rs)
   - ExperimentalConfig
   - ParallelMode enum
   - InstallTask.experimental_config字段

3. ✅ 实现辅助函数 (installer.rs)
   - is_ssd() - 磁盘类型检测
   - calculate_optimal_threads() - 智能线程数计算

4. ✅ 修改函数签名
   - install_content_with_progress
   - extract_archive_with_progress
   - extract_zip_with_progress

## 进行中 🚧

### 后端实现
5. ⏳ 修改extract_zip_with_progress函数体
   需要添加以下逻辑（在2587行之后）：

   ```rust
   // 1. 检查是否使用内存映射
   let archive_size = fs::metadata(archive_path)?.len();
   let use_mmap = experimental_config
       .map(|c| c.memory_mapped_zip && archive_size < 500 * 1024 * 1024)
       .unwrap_or(false);

   // 2. 根据配置打开archive（内存映射 vs 普通文件）
   let mut archive = if use_mmap {
       // 使用memmap2
       let file = fs::File::open(archive_path)?;
       let mmap = unsafe { memmap2::Mmap::map(&file)? };
       let cursor = std::io::Cursor::new(&mmap[..]);
       ZipArchive::new(cursor)?
   } else {
       let file = fs::File::open(archive_path)?;
       ZipArchive::new(file)?
   };

   // 3. 收集entries后，实现并行目录创建
   if experimental_config.map(|c| c.parallel_dir_creation).unwrap_or(false) {
       use rayon::prelude::*;
       let mut dirs: Vec<_> = entries.iter()
           .filter(|(_, _, is_dir, _, _)| *is_dir)
           .collect();
       dirs.sort_by_key(|(_, path, _, _, _)| path.components().count());
       dirs.par_iter().try_for_each(|(_, relative_path, _, _, _)| {
           let outpath = target.join(relative_path);
           fs::create_dir_all(&outpath)
       })?;
   } else {
       // 原有串行逻辑
   }

   // 4. 计算线程数并创建ThreadPool
   let thread_count = if let Some(config) = experimental_config {
       match config.parallel_mode {
           ParallelMode::Auto => {
               calculate_optimal_threads(entries.len(), total_size, target)
           }
           ParallelMode::Manual => {
               num_cpus::get() * config.thread_multiplier as usize
           }
       }
   } else {
       0 // 使用默认
   };

   if thread_count > 0 {
       crate::log_debug!(
           &format!("[TIMING] Using {} threads (mode: {:?})",
               thread_count, experimental_config.map(|c| &c.parallel_mode)),
           "installer_timing"
       );

       let pool = rayon::ThreadPoolBuilder::new()
           .num_threads(thread_count)
           .build()?;

       pool.install(|| {
           entries.par_iter()...
       })?;
   } else {
       // 使用默认rayon配置
       entries.par_iter()...
   }
   ```

## 待完成 📋

### 后端
6. ⬜ 修改其他调用extract_archive_with_progress的地方
   - handle_clean_install_with_progress
   - install_content_with_extraction_chain
   - 传递None作为experimental_config（嵌套archive不使用实验性功能）

### 前端
7. ⬜ 添加状态管理 (src/stores/app.ts)
8. ⬜ 添加UI组件 (src/views/Settings.vue)
9. ⬜ 添加中文翻译 (src/i18n/zh.ts)
10. ⬜ 添加英文翻译 (src/i18n/en.ts)
11. ⬜ 集成前后端 (src/views/Home.vue)

### 测试
12. ⬜ 编译测试
13. ⬜ 功能测试
14. ⬜ 性能测试

## 下一步行动

1. 完成extract_zip_with_progress函数体的修改
2. 修改其他调用点传递experimental_config
3. 编译测试后端
4. 实现前端部分
5. 端到端测试

## 注意事项

- 内存映射需要unsafe代码块
- ThreadPool创建可能失败，需要错误处理
- 并行目录创建需要处理竞态条件
- 需要添加详细的计时日志

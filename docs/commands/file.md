## 4. 软件功能情况
### 4.1. 软件支持的功能清单

| 软件功能                                                             | 原有软件 | Rust 重写后的软件 | 是否自动化测试 |
| -------------------------------------------------------------------- | -------- | ----------------- | -------------- |
| `file ...` 判断文件类型                                              | 支持     | 支持              | 是             |
| `--help` 显示程序的帮助信息                                          | 支持     | \*支持            | 否             |
| `-v, --version` 显示程序的版本信息                                   | 支持     | \*支持            | 否             |
| `-m, --magic-file` 载入指定 magic 文件                               | 支持     | 支持              | 是             |
| `-z, --uncompress` 尝试查看压缩文件                                  | 支持     | 支持              | 是             |
| `-Z, --uncompress-noreport` 尝试查看压缩文件，但不报告关于压缩的信息 | 支持     | 支持              | 是             |
| `-b, --brief` 不输出文件名                                           | 支持     | 支持              | 是             |
| `-c, --checking-printout` 输出 magic 文件解析后的形式                | 支持     | 支持              | 是             |
| `-e, --exclude TEST` 忽略文件类型                                    | 支持     | 支持              | 是             |
| `--exclude-quiet TEST` 忽略文件类型，对未知的 TEST 不报错            | 支持     | 支持              | 是             |
| `-f, --files-from FILE` 从 FILE 中读取文件名                         | 支持     | 支持              | 是             |
| `-F, --separator STRING` 以 STRING 作为分隔符                        | 支持     | 支持              | 是             |
| `-i, --mime`     输出 mime 类型                                      | 支持     | 支持              | 是             |
| `--apple` 输出旧 MacOS 使用的文件类型                                | 支持     | 支持              | 是             |
| `--extension`  输出文件类型的有效扩展名列表                          | 支持     | 支持              | 是             |
| `--mime-type`   同 -i 参数，只输出特定部分                           | 支持     | 支持              | 是             |
| `--mime-encoding`    同 -i 参数，只输出特定部分                      | 支持     | 支持              | 是             |
| `-k, --keep-going`   输出所有可能的匹配类型                          | 支持     | 支持              | 是             |
| `-l, --list`    输出模式列表及其强度                                 | 支持     | 支持              | 是             |
| `-L, --dereference`      跟随符号链接                                | 支持     | 支持              | 是             |
| `-h, --no-dereference`       不跟随符号链接                          | 支持     | 支持              | 是             |
| `-n, --no-buffer`   不缓存输出                                       | 支持     | 支持              | 是             |
| `-N, --no-pad`     不填充对齐文件名                                  | 支持     | 支持              | 是             |
| `-0, --print0`      在文件名后输出 '\0'                              | 支持     | 支持              | 是             |
| `-p, --preserve-date`   尝试不改变文件读取时间戳以假装未访问过文件   | 支持     | 支持              | \*否           |
| `-P, --parameter`    设置参数限制                                    | 支持     | 支持              | 是             |
| `-r, --raw`      不将不可打印字符输出为 `\ooo`                       | 支持     | 支持              | 是             |
| `-s, --special-files`      读取特殊文件                              | 支持     | 支持              | 是             |
| `-S, --no-sandbox`   不启用沙盒功能                                  | 支持     | 支持              | 是             |
| `-C, --compile`     编译产生 magic 文件                              | 支持     | 支持              | 是             |
| `-d, --debug`                   输出内部 debug 信息                  | 支持     | 支持              | 是             |

\*注：

- Rust 版本中 `--help` 参数输出为 clap 统一格式，与原命令不同
- Rust 版本中查询版本的参数为 `-V, --version`，同时输出为 easybox 统一格式，与原命令不同
- 在测试 `-p` 参数是观测到文件时间戳存在些许误差，无法准确对比，暂时略过

### 4.2. 软件自带用例对比验证

测试环境：
- OS: openEuler 23.09 on Windows 10 x86_64
- Kernel: 6.6.30-microsoft-standard-WSL2
- CPU: AMD Ryzen 9 7940H w/ Radeon 780M Graphics (16) @ 3.992GHz

软件自带测试用例为通过对大量不同的文件类型进行测试，判断文件类型是否正确。
由于 Rust 版 file 命令通过调用原命令库进行实现，因此仅需判断是否正确调用。为避免复制大量原项目的测试用例，此处仅需选取少量用例进行测试。

- cmd1.testfile

![cmd1.testfile](https://gitee.com/mizukicry/images/raw/master/OSPP2024/image.png)

- json1.testfile

![json1.testfile](https://gitee.com/mizukicry/images/raw/master/OSPP2024/image-1.png)

如下为使用 easybox 框架进行测试的结果：

```
~/codes/easybox (master) » cargo test test_file
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running unittests src/bin/coreutils.rs (target/debug/deps/easybox-c99683acd27b3c9c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/tests.rs (target/debug/deps/tests-0fb92d10981cd0bf)

running 29 tests
test test_file::test_arg_apple ... ok
test test_file::test_arg_checking_printout ... ok
test test_file::test_arg_compile ... ok
test test_file::test_arg_debug ... ok
test test_file::test_arg_dereference ... ok
test test_file::test_arg_exclude ... ok
test test_file::test_arg_exclude_quiet ... ok
test test_file::test_arg_extension ... ok
test test_file::test_arg_files_from ... ok
test test_file::test_arg_keep_going ... ok
test test_file::test_arg_list ... ok
test test_file::test_arg_magic_file ... ok
test test_file::test_arg_mime ... ok
test test_file::test_arg_mime_encoding ... ok
test test_file::test_arg_mime_type ... ok
test test_file::test_arg_no_buffer ... ok
test test_file::test_arg_no_dereference ... ok
test test_file::test_arg_no_pad ... ok
test test_file::test_arg_parameter ... ok
test test_file::test_arg_preserve_date ... ignored
test test_file::test_arg_print0 ... ok
test test_file::test_arg_raw ... ok
test test_file::test_arg_separator ... ok
test test_file::test_arg_special_files ... ok
test test_file::test_arg_uncompress_and_no_sandbox ... ok
test test_file::test_arg_uncompress_noreport_and_no_sandbox ... ok
test test_file::test_cmd ... ok
test test_file::test_json ... ok
test test_file::test_pgp ... ok
test test_file::test_zstd ... ok

test result: ok. 28 passed; 0 failed; 1 ignored; 0 measured; 311 filtered out; finished in 0.79s
```

测试用例均通过（1个测试因时间戳精度问题被忽略）

### 4.3. 功能对比验证

1. 功能点：测试 `-m` 参数指定使用特定 magic 文件【test_arg_magic_file】

- 测试方法：在测试环境中运行 `easybox file -m rpm.magic rpm.file` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同，成功读取到指定 magic 文件，实现更精准的文件类型判断

2. 功能点：测试 `-z -S` 参数尝试读取压缩文件内部内容【test_arg_uncompress_and_no_sandbox】

- 测试方法：在测试环境中运行 `easybox file -z -S rpm.tar.gz` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同，成功识别压缩文件内容

3. 功能点：测试 `-Z -S` 参数尝试读取压缩文件内部内容，但忽略压缩相关的信息【test_arg_uncompress_noreport_and_no_sandbox】

- 测试方法：在测试环境中运行 `easybox file -Z -S rpm.tar.gz` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同，成功识别压缩文件内容，未输出压缩相关信息

4. 功能点：测试 `-b` 参数，在输出时不输出文件名【test_arg_brief】

- 测试方法：在测试环境中运行 `easybox file -b rpm.file` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同，输出结果时只输出文件类型，不输出文件名

5. 功能点：测试 `-c` 参数输出 magic 文件解析后的形式【test_arg_checking_printout】

- 测试方法：在测试环境中运行 `easybox file -m rpm.magic -c` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同，产生大量 magic 文件解析的内容

6. 功能点：测试 `-e` 参数忽略特定类型的测试集【test_arg_exclude】

- 测试方法：在测试环境中运行 `easybox file -e json json.file` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同，最终产生的文件类型不包括 json

7. 功能点：测试 `--exclude-quiet` 参数【test_arg_exclude_quiet】

- 测试方法：在测试环境中运行 `easybox file --exclude-quiet ??? json.file` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

8. 功能点：测试 `-f` 参数【test_arg_files_from】

- 测试方法：在测试环境中运行 `easybox file -f files.txt` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

9. 功能点：测试 `-F` 参数【test_arg_separator】

- 测试方法：在测试环境中运行 `easybox file -F YuukaC pgp.file` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

10. 功能点：测试 `-i` 参数【test_arg_mime】

- 测试方法：在测试环境中运行 `easybox file -i json.file` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

11. 功能点：测试 `--apple` 参数【test_arg_apple】

- 测试方法：在测试环境中运行 `easybox file --apple json.file` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

12. 功能点：测试 `--extension` 参数【test_arg_extension】

- 测试方法：在测试环境中运行 `easybox file --extension zstd.file` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

13. 功能点：测试 `--mime-type` 参数【test_arg_mime_type】

- 测试方法：在测试环境中运行 `easybox file --mime-type json.file` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

14. 功能点：测试 `--mime-encoding` 参数【test_arg_mime_encoding】

- 测试方法：在测试环境中运行 `easybox file --mime-encoding json.file` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

15. 功能点：测试 `-k` 参数【test_arg_keep_going】

- 测试方法：在测试环境中运行 `easybox file -k json.file` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

16. 功能点：测试 `-l` 参数【test_arg_list】

- 测试方法：在测试环境中运行 `easybox file -l` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

17. 功能点：测试 `-L` 参数【test_arg_dereference】

- 测试方法：在测试环境中运行 `easybox file -L pgp.sl` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

18. 功能点：测试 `-h` 参数【test_arg_no_dereference】

- 测试方法：在测试环境中运行 `easybox file -h pgp.sl` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

19. 功能点：测试 `-n` 参数【test_arg_no_buffer】

- 测试方法：在测试环境中运行 `easybox file -n cmd.file json.file pgp.file pgp.sl rpm.file rpm.magic rpm.tar.gz zstd.file` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

20. 功能点：测试 `-N` 参数【test_arg_no_pad】

- 测试方法：在测试环境中运行 `easybox file -N cmd.file json.file pgp.file pgp.sl rpm.file rpm.magic rpm.tar.gz zstd.file` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

21. 功能点：测试 `-0` 参数【test_arg_print0】

- 测试方法：在测试环境中运行 `easybox file -0 rpm.file`, `easybox file -0 -0 rpm.file` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

22. 功能点：测试 `-P` 参数【test_arg_parameter】

- 测试方法：在测试环境中运行以下命令与原命令结果进行对比
  - `easybox file -P bytes=0 cmd.file`
  - `easybox file -P bytes=1 cmd.file`
  - `easybox file -P bytes=10 cmd.file`
  - `easybox file -P bytes=100 cmd.file`
- 测试结果：Rust 版本结果与原命令相同

23.  功能点：测试 `-r` 参数【test_arg_raw】

- 测试方法：在测试环境中运行 `easybox file \u{2000}`（Unicode 字符串）, `file -r \u{2000}` 进行对比
- 测试结果：两条命令输出不同，参数生效

24. 功能点：测试 `-s` 参数【test_arg_special_files】

- 测试方法：在测试环境中运行 `easybox file /dev/null`, `file -s /dev/null` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

25. 功能点：测试 `-C` 参数【test_arg_compile】

- 测试方法：在测试环境中运行 `easybox file -m rpm.magic -C` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令相同

26. 功能点：测试 `-d` 参数【test_arg_debug】

- 测试方法：在测试环境中运行 `easybox file -d cmd.file` 与原命令结果进行对比
- 测试结果：Rust 版本结果与原命令格式相同

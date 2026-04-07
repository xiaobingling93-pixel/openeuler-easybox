### 软件支持的功能对比

| 软件功能                                                | 原有软件 | Rust重写后的软件 |
| ------------------------------------------------------- | -------- | ---------------- |
| -b, --bytes show output in bytes                        | 支持     | 支持             |
| -c, --count repeat printing N times, then exit          | 支持     | 支持             |
| -g, --gibi show output in gibibytes                     | 支持     | 支持             |
| --giga show output in gigabytes                         | 支持     | 支持             |
| -h, --human show human-readable output                  | 支持     | 支持             |
| --help Print help information                           | 支持     | 支持             |
| -k, --kibi show output in kibibytes                     | 支持     | 支持             |
| --kilo show output in kilobytes                         | 支持     | 支持             |
| -l, --lohi show detailed low and high memory statistics | 支持     | 支持             |
| -m, --mebi show output in mebibytes                     | 支持     | 支持             |
| --mega show output in megabytes                         | 支持     | 支持             |
| --pebi show output in pebibytes                         | 支持     | 支持             |
| --peta show output in petabytes                         | 支持     | 支持             |
| -s, --seconds repeat printing every N seconds           | 支持     | 支持             |
| --si use powers of 1000 not 1024                        | 支持     | 支持             |
| -t, --total show total for RAM + swap                   | 支持     | 支持             |
| --tebi show output in tebibytes                         | 支持     | 支持             |
| --tera show output in terabytes                         | 支持     | 支持             |
| -V, --version Print version information                 | 支持     | 支持             |
| -w, --wide wide output                                  | 支持     | 支持             |

--lohi参数由于在64位系统中并无高内存与低内存之分，显示逻辑改为和实际程序一样显示实际内存使用总量

### 软件自带测试用例

软件测试实现了以下测试点

1. test_free - 测试无参数输出，检测输出是否与原有软件一致
2. test_free_t - 测试 -t 参数，测试输出是否与原有软件一致
3. test_free_h - 测试 -h 参数，测试输出是否与原有软件一致
4. test_free_s - 测试 -s -c 参数，测试输出是否与原有软件一样输出多次
5. test_free_l - 测试 -l 参数，测试输出是否与原有软件一样输出多次
6. test_free_muti_unit - 测试 -b，-k 组合单位参数，测试是否报错
7. test_free_units - 测试所有单个单位选项，测试是否与原有软件一致
8. test_free_invalid_unit - 测试非法参数，测试软件是否报错
9. test_free_w - 测试 -w 参数，测试宽输出格式是否正确

![测试](https://foruda.gitee.com/images/1714374959027206123/fab9e6ef_10135006.png)

### 功能对比验证

1. 功能点：无参数输出
   - 测试方法：在测试环境中运行`easybox free`
   - 测试结果：显示系统内存的总和、已使用和空闲的内存量。
2. 功能点：-b 参数输出
   - 测试方法：在测试环境中运行`easybox free -b`
   - 测试结果：以字节为单位显示内存和交换空间的使用情况。
3. 功能点：-c 参数输出
   - 测试方法：在测试环境中运行`easybox free -c 5`
   - 测试结果：重复打印内存状态5次，然后退出。
4. 功能点：-g 参数输出
   - 测试方法：在测试环境中运行`easybox free -g`
   - 测试结果：以吉字节（GiB）为单位显示内存和交换空间的使用情况。
5. 功能点：-h 参数输出
   - 测试方法：在测试环境中运行`easybox free -h`
   - 测试结果：以人类可读的格式显示内存和交换空间的使用情况。
6. 功能点：-k 参数输出
   - 测试方法：在测试环境中运行`easybox free -k`
   - 测试结果：以千字节（KiB）为单位显示内存和交换空间的使用情况。
7. 功能点：-l 参数输出
   - 测试方法：在测试环境中运行`easybox free -l`
   - 测试结果：显示详细的低和高内存统计信息。
8. 功能点：-m 参数输出
   - 测试方法：在测试环境中运行`easybox free -m`
   - 测试结果：以兆字节（MiB）为单位显示内存和交换空间的使用情况。
9. 功能点：-s 参数输出
   - 测试方法：在测试环境中运行`easybox free -s 2`
   - 测试结果：每2秒重复打印内存状态。
10. 功能点：-t 参数输出
    - 测试方法：在测试环境中运行`easybox free -t`
    - 测试结果：显示RAM和交换空间的总和。
11. 功能点：--si 参数输出
    - 测试方法：在测试环境中运行`easybox free --si`
    - 测试结果：使用1000为底数的幂而不是1024来显示内存和交换空间的使用情况。
12. 功能点：-V 参数输出
    - 测试方法：在测试环境中运行`easybox free -V`
    - 测试结果：打印easybox free命令的版本信息。
13. 功能点：-w 参数输出
    - 测试方法：在测试环境中运行`easybox free -w`
    - 测试结果：以宽格式显示内存和交换空间的使用情况。
14. 功能点：--help参数输出
    - 测试方法：在测试环境中运行`easybox free --help`
    - 测试结果：打印free指令的使用指南
15. 功能点：-c参数输出
    - 测试方法：在测试环境中运行`easybox free -c 10`
    - 测试结果：间隔1秒打印内存状态打印10次
16. 功能点：-c -s参数输出
    - 测试方法：在测试环境中运行`easybox free -c 10 -s 4`
    - 测试结果：间隔4秒打印内存状态打印10次

### 软件自带用例对比验证
将软件源码的测试用例转写为shell脚本如下

```bash
#!/bin/bash
shopt -s expand_aliases
alias free="$@"
```

#### 测试 free 命令的基本功能
free
echo "free -b"
free -b  # 以字节为单位显示内存使用情况
echo "free -k"
free -k  # 以千字节为单位显示内存使用情况
echo "free -m"
free -m  # 以兆字节为单位显示内存使用情况
echo "free -g"
free -g  # 以吉字节为单位显示内存使用情况

echo "free --bytes"
free --bytes  # 以字节为单位显示内存使用情况
echo "free --kilo"
free --kilo  # 以千字节为单位显示内存使用情况
echo "free --mega"
free --mega  # 以兆字节为单位显示内存使用情况
echo "free --giga"
free --giga  # 以吉字节为单位显示内存使用情况
echo "free --tera"
free --tera  # 以太字节为单位显示内存使用情况
echo "free --peta"
free --peta  # 以拍字节为单位显示内存使用情况

echo "free --kibi"
free --kibi  # 以 kibibyte 为单位显示内存使用情况
echo "free --mebi"
free --mebi  # 以 mebibyte 为单位显示内存使用情况
echo "free --gibi"
free --gibi  # 以 gibibyte 为单位显示内存使用情况
echo "free --tebi"
free --tebi  # 以 tebibyte 为单位显示内存使用情况
echo "free --pebi"
free --pebi  # 以 pebibyte 为单位显示内存使用情况

#### 测试不同的刷新间隔
echo "free -s 1 -c 3"
free -s 1 -c 3  # 每2秒刷新一次，持续5次

#### 测试不同的输出格式
echo "free -w"
free -w  # 宽输出格式
echo "free -t"
free -t  # 显示总计行
echo "free -h"
free -h  # 人性化显示
echo "free -h -t"
free -h -t  # 人性化显示，显示总计行
echo "free -l"
free -l  # 高低地址内存使用情况

echo "free -k --si"
free -k --si  # 以千字节为单位显示内存使用情况，使用国际单位制
echo "free --kilo "
free --kilo   # 以 kibibyte 为单位显示内存使用情况，使用国际单位制

free -V

## 功能对比验证模版
### 软件支持的功能清单
| 软件功能 | 原有软件 | Rust重写后的软件 |
| --- | --- | --- |
| sysctl [options] [variable[=value] ...] | 支持 | 支持 |
| -a, --all, -A, -X 显示当前有效的所有变量和值 | 支持 | 支持 |
| --deprecated 使用-a选项时显示弃用的变量 | 支持 | 支持 |
| --dry-run 显示变量的键和值，但不写入 | 支持 | 支持 |
| -b, --binary 显示值而不换行 | 支持 | 支持 |
| -e, --ignore 忽略未知变量错误 | 支持 | 支持 |
| -N, --names 打印不带值的变量名称 | 支持 | 支持 |
| -n, --values 仅打印给定变量的值 | 支持 | 支持 |
| -p, --load[=\<file\>], -f 从文件读取值 | 支持 | 支持 |
| --system 从所有系统目录读取值 | 支持 | 支持 |
| -r, --pattern \<expression\> 选择与表达式匹配的设置 | 支持 | 支持 |
| -q, --quiet 不回显变量集合 | 支持 | 支持 |
| -w, --write 允许将值写入变量 | 支持 | 支持 |
| -o, -x 不进行任何操作 | 支持 | 支持 |
| -h, --help, -d 显示帮助信息 | 支持 | 支持 |
| -V, --version 显示版本信息 | 支持 | 支持 |

### 软件自带用例对比验证
软件具有自带测试用例，以下使用sysctl的最新版本进行测试（Commit ID：92686791）：

#### 测试无参数时输出帮助信息

Rust语言程序和C程序运行结果如下：

![sysctl-2](https://foruda.gitee.com/images/1718638708228893013/2d39607e_7603128.png "sysctl-2.png")

![sysctl-3](https://foruda.gitee.com/images/1718638729855668261/9fd71e50_7603128.png "sysctl-3.png")

#### 测试使用`/`作为分隔符读取变量

Rust语言程序和C程序运行结果如下：

![sysctl-4](https://foruda.gitee.com/images/1718638745781818676/4149040c_7603128.png "sysctl-4.png")

#### 测试使用`.`作为分隔符读取变量

Rust语言程序和C程序运行结果如下：

![sysctl-5](https://foruda.gitee.com/images/1718638761235762728/26954ecd_7603128.png "sysctl-5.png")

#### 测试只读取变量值

Rust语言程序和C程序运行结果如下：

![sysctl-6](https://foruda.gitee.com/images/1718638777597064914/c5b90446_7603128.png "sysctl-6.png")

#### 测试只读取变量名

Rust语言程序和C程序运行结果如下：

![sysctl-7](https://foruda.gitee.com/images/1718638792812097593/af4d47e3_7603128.png "sysctl-7.png")

#### 测试使用路径穿越读取

Rust语言程序和C程序运行结果如下：

![sysctl-8](https://foruda.gitee.com/images/1718638807400223631/5ae4546f_7603128.png "sysctl-8.png")

#### 测试通过命令行、使用`.`作为分隔符写入变量

Rust语言程序和C程序运行结果如下：

![sysctl-9](https://foruda.gitee.com/images/1718638846969937797/7d184943_7603128.png "sysctl-9.png")

#### 测试通过命令行、使用`/`作为分隔符写入变量

Rust语言程序和C程序运行结果如下：

![sysctl-10](https://foruda.gitee.com/images/1718638826341173922/260113db_7603128.png "sysctl-10.png")

#### 测试通过配置文件写入变量

Rust语言程序和C程序运行结果如下：

![sysctl-11](https://foruda.gitee.com/images/1718638889284509749/159eeb1b_7603128.png "sysctl-11.png")

#### 测试通过带`/`分隔符的配置文件写入变量

Rust语言程序和C程序运行结果如下：

![sysctl-12](https://foruda.gitee.com/images/1718638904087129260/9596f8b2_7603128.png "在这里输入图片标题")

#### 测试写入不可写的文件

Rust语言程序和C程序运行结果如下：

![sysctl-13](https://foruda.gitee.com/images/1718638917155003798/3cd3fb41_7603128.png "sysctl-13.png")

#### 测试写入不可写的文件并忽略错误

Rust语言程序和C程序运行结果如下：

![sysctl-14](https://foruda.gitee.com/images/1718638933439719462/c14bc538_7603128.png "sysctl-14.png")

#### 测试写入非`/proc`下的文件

Rust语言程序和C程序运行结果如下：

![sysctl-15](https://foruda.gitee.com/images/1718638951070843998/9747d8fc_7603128.png "sysctl-15.png")

以上测试已集成至测试代码中，测试代码执行结果如下：

![sysctl-1](https://foruda.gitee.com/images/1718638965603790278/d7d4e4ef_7603128.png "sysctl-1.png")

### 功能对比验证
1. 功能点1：查看系统变量
    - 测试方法：在测试环境中运行`easybox sysctl kernel.hostname`
    - 测试结果：输出本机的hostname
2. 功能点2：-a, --all, -A, -X 显示当前有效的所有变量和值
    - 测试方法：在测试环境中运行`easybox sysctl -a`
    - 测试结果：输出本机有效的所有变量和值
3. 功能点3：--deprecated 使用-a选项时显示弃用的变量
    - 测试方法：在测试环境中运行`easybox sysctl -a --deprecated`
    - 测试结果：输出本机所有变量和值（包含base_reachable_time等已弃用的变量）
4. 功能点4：--dry-run 显示变量的键和值，但不写入
    - 测试方法：在测试环境中运行`easybox sysctl --dry-run kernel.hostname=procps-test`
    - 测试结果：输出kernel.hostname=procps-test，但不写入
5. 功能点5：-b, --binary 显示值而不换行
    - 测试方法：在测试环境中运行`easybox sysctl -b kernel.hostname`
    - 测试结果：输出本机的hostname值但不换行
6. 功能点6：-e, --ignore 忽略未知变量错误
    - 测试方法：在测试环境中运行`easybox sysctl -w -e kernel.notexistfile=1`
    - 测试结果：无输出
7. 功能点7：-N, --names 打印不带值的变量名称
    - 测试方法：在测试环境中运行`easybox sysctl -N kernel.hostname`
    - 测试结果：输出kernel.hostname
8. 功能点8：-n, --values 仅打印给定变量的值
    - 测试方法：在测试环境中运行`easybox sysctl -n kernel.hostname`
    - 测试结果：输出本机的hostname值
9. 功能点9：-p, --load[=\<file\>], -f 从文件读取值
    - 测试方法：在测试环境中运行`echo "kernel/hostname = procps-test" > test.conf && easybox sysctl --dry-run -f test.conf`
    - 测试结果：输出kernel.hostname = procps-test
10. 功能点10：--system 从所有系统目录读取值
    - 测试方法：在测试环境中运行`easybox sysctl --dry-run --system`
    - 测试结果：输出系统目录中的配置值
11. 功能点11：-r, --pattern \<expression\> 选择与表达式匹配的设置
     - 测试方法：在测试环境中运行`easybox sysctl -a --pattern forward$`
     - 测试结果：输出系统所有变量中以forward结尾的变量
12. 功能点12：-q, --quiet 不回显变量集合
     - 测试方法：在测试环境中运行`easybox sysctl -q --system`
     - 测试结果：不输出Applying信息
13. 功能点13：-w, --write 允许将值写入变量
     - 测试方法：在测试环境中运行`easybox sysctl -w --dry-run kernel.hostname=procps-test`
     - 测试结果：输出kernel.hostname = procps-test
14. 功能点14：-o, -x 不进行任何操作
     - 测试方法：在测试环境中运行`easybox sysctl -x -o`
     - 测试结果：提示参数不足
15. 功能点15：-h, --help, -d 显示帮助信息
     - 测试方法：在测试环境中运行`easybox sysctl -h`
     - 测试结果：显示帮助信息
16. 功能点16：-V或--version 显示版本信息
     - 测试方法：在测试环境中运行`easybox setsid -V`
     - 测试结果：显示sysctl的版本信息

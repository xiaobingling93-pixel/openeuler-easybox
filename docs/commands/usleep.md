## 功能对比验证模版
### 软件支持的功能清单
| 软件功能 | 原有软件 | Rust重写后的软件 | 是否自动化测试 |
| --- | --- | --- | --- |
| usleep [-vo?] [-v\|--version] [-o\|--oot] [-?\|--help] [--usage] [microseconds] | 支持 | 支持 | 是 |
| -v, --version 显示版本信息 | 支持 | 支持 | 否 |
| -o, --oot 显示字符串"oot says hey!" | 支持 | 支持 | 是 |
| -?, --help 显示帮助信息 | 支持 | 支持 | 否 |
| --usage 显示简短的使用方法信息 | 支持 | 支持 | 是 |
| [microseconds] 指定睡眠的微秒数（支持十进制、八进制、十六进制） | 支持 | 支持 | 是 |

### 软件自带用例对比验证
软件没有自带测试用例，在Rust测试环境中添加4个测试用例，下面是测试情况：

running 4 tests
test test_usleep::test_usleep_1000000 ... ok
test test_usleep::test_usleep_oot ... ok
test test_usleep::test_usleep_no_arg ... ok
test test_usleep::test_usleep_extra_operand ... ok

#### 测试睡眠功能

分别以1000000、03641100、0xf4240、0XF4240为参数测试usleep是否能正确解析并睡眠用不同进制表示的1秒，Rust语言程序和C程序运行结果如下：

![usleep-1](https://foruda.gitee.com/images/1715360055956722628/5841d642_7603128.png "usleep-1.png")

![usleep-2](https://foruda.gitee.com/images/1715360077248081816/ccd22fac_7603128.png "usleep-2.png")

![usleep-3](https://foruda.gitee.com/images/1715360090724818080/78a65f29_7603128.png "usleep-3.png")

![usleep-4](https://foruda.gitee.com/images/1715360112922079721/a11937eb_7603128.png "usleep-4.png")

#### 测试-o和--oot功能

Rust语言程序和C程序运行结果如下：

![usleep-5](https://foruda.gitee.com/images/1715360146993706548/76299e5f_7603128.png "usleep-5.png")

#### 测试没有参数时程序的功能

Rust语言程序和C程序运行结果如下：

![usleep-6](https://foruda.gitee.com/images/1715360158022746501/5bf98461_7603128.png "usleep-6.png")

#### 测试多个参数时程序的功能

Rust语言程序和C程序运行结果如下：

![usleep-7](https://foruda.gitee.com/images/1715360169142499715/37e270a2_7603128.png "usleep-7.png")

以上测试已集成至测试代码中，测试代码执行结果如下：

![usleep-8](https://foruda.gitee.com/images/1715360191572738641/02db216e_7603128.png "usleep-8.png")

### 功能对比验证
1. 功能点1：睡眠一段时间
- 测试方法：在测试环境中运行`time easybox usleep 1000000`
- 测试结果：usleep的运行时间约为1秒
2. 功能点2：-v或--version 显示版本信息
- 测试方法：在测试环境中运行`easybox usleep -v`或`easybox usleep --version`
- 测试结果：显示usleep的版本信息
3. 功能点3：-o或--oot 显示字符串“oot says hey!”
- 测试方法：在测试环境中运行`easybox usleep -o`或`easybox usleep --oot`
- 测试结果：显示字符串“oot says hey!”
4. 功能点4：-?或--help 显示帮助信息
- 测试方法：在测试环境中运行`easybox usleep -?`或`easybox usleep --help`
- 测试结果：显示帮助信息
5. 功能点5：--usage 显示简短的使用方法信息
- 测试方法：在测试环境中运行`easybox usleep --usage`
- 测试结果：显示简短的使用方法信息

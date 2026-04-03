## 功能对比验证模版
### 软件支持的功能清单
| 软件功能 | 原有软件 | Rust重写后的软件 |
| --- | --- | --- |
| setsid [options] \<program\> [arguments ...] | 支持 | 支持 |
| -c, --ctty 将控制终端设置为当前终端 | 支持 | 支持 |
| -f, --fork 总是fork | 支持 | 支持 |
| -w, --wait 等待程序退出，使用与程序相同的返回值 | 支持 | 支持 |
| -h, --help 显示帮助信息 | 支持 | 支持 |
| -V, --version 显示版本信息 | 支持 | 支持 |

### 软件自带用例对比验证
软件没有自带测试用例，设计如下测试用例：

#### 测试在新的session运行程序的功能

分别在非fork和fork情况下使用setsid命令运行sleep程序，观察sleep进程的pid、ppid和sid：

- 在非fork情况下，sleep进程由setsid进程调用execvp得到，因此sleep进程与setsid进程的pid应相同，可以间接通过sleep进程的ppid为bash确认setsid没有执行fork，且sleep进程的sid与pid相同，说明sleep在一个新的session运行。Rust语言程序和C程序运行结果如下：

![setsid-1](https://foruda.gitee.com/images/1717258089344524313/7cb5886a_7603128.png "setsid-1.png")

![setsid-2](https://foruda.gitee.com/images/1717258114932373111/4b6b0755_7603128.png "setsid-2.png")

- 在fork情况下，sleep进程由setsid进程调用fork得到，若不启用--wait选项，setsid进程将会立即退出，因此sleep进程的ppid被设置为1，可以通过sleep进程的ppid为1确认setsid执行了fork，且sleep进程的sid与pid相同，说明sleep在一个新的session运行。Rust语言程序和C程序运行结果如下：

![setsid-3](https://foruda.gitee.com/images/1717258137294307141/127490b8_7603128.png "setsid-3.png")

![setsid-4](https://foruda.gitee.com/images/1717258151383777299/72415582_7603128.png "setsid-4.png")

#### 测试程序回显及返回值

Rust语言程序和C程序运行结果如下：

#### 测试-c和--ctty功能

若不启用-c选项，则程序的控制终端不是当前终端，通过ps指令查看ping进程对应的tty为空，无法在当前终端通过ctrl c中断ping进程。Rust语言程序和C程序运行结果如下：

![setsid-5](https://foruda.gitee.com/images/1717258178040231975/2b32e8c8_7603128.png "setsid-5.png")

![setsid-6](https://foruda.gitee.com/images/1717258193350723302/7d025d47_7603128.png "setsid-6.png")

若启用-c选项，则程序的控制终端被设置为当前终端，通过ps指令查看ping进程对应的tty当前终端，可以在当前终端通过ctrl c中断ping进程。Rust语言程序和C程序运行结果如下：

![setsid-7](https://foruda.gitee.com/images/1717258215760189849/0469ff94_7603128.png "setsid-7.png")

![setsid-8](https://foruda.gitee.com/images/1717258233857517246/8d2fbfbb_7603128.png "setsid-8.png")

#### 测试-f和--fork功能

若启用-f选项，则强制启用fork。Rust语言程序和C程序运行结果如下：

![setsid-9](https://foruda.gitee.com/images/1717258251570876929/2123a8c3_7603128.png "setsid-9.png")

![setsid-10](https://foruda.gitee.com/images/1717258269233686143/bec4e15f_7603128.png "setsid-10.png")

#### 测试-w和--wait功能

若启用-w选项，在fork的情况下，setsid进程将等待程序退出，并使用与程序相同的返回值。Rust语言程序和C程序运行结果如下：

![setsid-11](https://foruda.gitee.com/images/1717258286283931869/6722db95_7603128.png "setsid-11.png")

![setsid-12](https://foruda.gitee.com/images/1717258351586018910/56418584_7603128.png "setsid-12.png")

![setsid-13](https://foruda.gitee.com/images/1717258368594296424/b67b3659_7603128.png "setsid-13.png")

![setsid-14](https://foruda.gitee.com/images/1717258392916223469/3b6e7922_7603128.png "setsid-14.png")

#### 测试没有参数时程序的功能

Rust语言程序和C程序运行结果如下：

![setsid-15](https://foruda.gitee.com/images/1717258410253247373/2056636e_7603128.png "setsid-15.png")

以上测试已集成至测试代码中，测试代码执行结果如下：

![setsid-16](https://foruda.gitee.com/images/1717258428542724219/6a71bdce_7603128.png "setsid-16.png")

### 功能对比验证
1. 功能点1：在新的session运行程序
- 测试方法：在测试环境中运行`easybox setsid sleep 10.03270904`
- 测试结果：使用ps指令查看sleep进程的sid与pid相同
2. 功能点2：-c或--ctty 将控制终端设置为当前终端
- 测试方法：在测试环境中运行`sudo easybox setsid -c ping openeuler.org`或`sudo easybox setsid --ctty ping openeuler.org`
- 测试结果：ping进程的控制终端为当前终端
3. 功能点3：-f或--fork 总是fork
- 测试方法：在测试环境中运行`echo 1 | easybox setsid -f sleep 10.03270904 `或`echo 1 | easybox setsid --fork sleep 10.03270904`
- 测试结果：使用ps指令查看sleep进程的父进程为/init进程或1号进程
4. 功能点4：-w或--wait 等待程序退出，使用与程序相同的返回值
- 测试方法：在测试环境中运行`time easybox setsid -w sleep 2`或`time easybox setsid --wait sleep 2`
- 测试结果：程序运行时间约为2秒
5. 功能点5：-h或--help 显示帮助信息
- 测试方法：在测试环境中运行`easybox setsid -h`或`easybox setsid --help`
- 测试结果：显示帮助信息
6. 功能点6：-V或--version 显示版本信息
- 测试方法：在测试环境中运行`easybox setsid -V`或`easybox setsid --version`
- 测试结果：显示setsid的版本信息

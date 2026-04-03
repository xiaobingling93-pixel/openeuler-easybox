## 功能对比验证
### 软件支持的功能清单


| 软件功能                                              | 原有软件 | Rust重写后的软件 |
| ----------------------------------------------------- | -------- | ---------------- |
| pidof [options] [program [...]]                       | 支持     | 支持             |
| -s, --single-shot：每个进程名只返回一个PID            | 支持     | 支持             |
| -c, --check-root：忽略不同根目录的进程                | 支持     | 支持             |
| -q：安静模式，仅设置退出代码                          | 支持     | 支持             |
| -w, --with-workers：显示内核工作线程                  | 支持     | 支持             |
| -x：查找运行指定脚本的shell                           | 支持     | 支持             |
| -o, --omit-pid \<omit-pid>... ：忽略具有指定PID的进程 | 支持     | 支持             |
| -S, --separator \<separator>...：指定分隔符           | 支持     | 支持             |
| -h, --help：打印帮助信息                              | 支持     | 支持             |
| -t, --lightweight：列出线程                           | 支持     | 支持             |
| -V, --version：打印版本信息                           | 支持     | 支持             |

### 软件自带用例对比验证

| 测试用例 | 测试目的 | 已集成至测试代码 |
| --- | --- | :---: |
| pidof easybox | 基础功能 | √ |
| pidof systemd | 单个程序查询 | √ |
| pidof bash | 单个程序查询 | √ |
| pidof /usr/bin/bash | 带路径查询 | √ |
| pidof /bin/bash | 带路径查询 | √ |
| pidof systemd bash | 多个程序查询 | √ |
| pidof zsh ssh sshd bash | 多个程序查询 | √ |
| pidof -s bash systemd sh | -s/--single-shot选项 | √ |
| pidof -q systemd | -q安静模式 | √ |
| pidof -w kthreadd | -w/--with-workers选项 | √ |
| pidof -o 1 systemd | -o/--omit-pid选项 | √ |
| pidof -o 1,2 systemd kthreadd -w | 多个omit-pid | √ |
| pidof -S - systemd sleep bash | -S/--separator选项 | √ |
| pidof -x test_sleep.sh | -x脚本匹配选项 | √ |
| pidof -w -s systemd | 混合选项 | √ |
| pidof -w -q -o 1 -S "," bash sh | 混合选项 | √ |
| pidof -q -w -o 1 -x systemd node sh | 混合选项 | √ |
| pidof kswapd0 -w -q | 混合选项 | √ |

软件暂无自带测试用例，下面将通过与原版程序交叉验证来测试软件的主要功能，测试方法如下：

**测试pidof功能**
![1](https://foruda.gitee.com/images/1716539661349966741/f28675a7_14024037.png "1.png")

**测试-s功能**
![2](https://foruda.gitee.com/images/1716539701052872554/9cee9e4a_14024037.png "2.png")

**测试-c功能**
![输入图片说明](https://foruda.gitee.com/images/1716539716197071930/c1c5f5d7_14024037.png "3.png")

**测试-q功能**
![4](https://foruda.gitee.com/images/1716539767532788530/2c43342e_14024037.png "4.png")

**测试-w功能**
![5](https://foruda.gitee.com/images/1716539787921683628/278919d0_14024037.png "5.png")

**测试-x功能**
![6](https://foruda.gitee.com/images/1716539803775768745/054a8102_14024037.png "6.png")
**测试-S功能**
![7](https://foruda.gitee.com/images/1716539830316949331/9dc6cdc8_14024037.png "7.png")
**测试-o功能**
![输入图片说明](https://foruda.gitee.com/images/1716539980400641574/00e02c53_14024037.png "1b71c1c1c3c526c15163e93edc3be5c.png")
**测试-h功能**
![输入图片说明](https://foruda.gitee.com/images/1716539844887666312/d11d3d7e_14024037.png "8.png")
**测试-t功能**
![9](https://foruda.gitee.com/images/1716539877965441054/3769d1bf_14024037.png "9.png")
**测试-V功能**
![10](https://foruda.gitee.com/images/1716539902527956643/f446f252_14024037.png "10.png")



### 功能对比验证
1. 功能点1：打印程序的进程号
- 测试方法：在测试环境中运行命令`pidof bash`
- 测试结果：命令运行结果、返回值与原有C程序保持一致。
2. 功能点2：`-s, --single-shot`，每个进程名只返回一个PID
- 测试方法：在测试环境中运行命令`pidof bash -s`
- 测试结果：运行成功，只显示一个进程名。

3. 功能点3：`-c, --check-root`，忽略不同根目录的进程

- 测试方法：在测试环境中运行命令`pidof bash`
- 测试结果：命令运行结果、返回值与原有C程序保持一致。

4. 功能点4：`-q`，安静模式，仅设置退出代码

- 测试方法：在测试环境中运行命令`pidof bash -q`
- 测试结果：运行成功，不打印进程号，且返回值为0。
5. 功能点5：`-w, --with-workers`，显示内核工作线程

- 测试方法：在测试环境中运行命令`pidof -w kthreadd`
- 测试结果：运行成功，打印出内核进程`kthreadd`的进程号2。
6. 功能点6：`-x`，查找运行指定脚本的shell

- 测试方法：先执行命令`tests/fixtures/pidof/bin/test_sleep.sh &`，在后台运行脚本；运行命令`pidof -x test_sleep.sh`
- 测试结果：运行成功，打印出执行脚本的进程号。

7. 功能点7：`-o, --omit-pid <omit-pid>...`，忽略具有指定PID的进程

- 测试方法：在测试环境中运行命令`pidof systemd -o 1`
- 测试结果：运行成功，PID 1 被忽略。

8. 功能点8：`-t, --lightweight`，列出线程

- 测试方法：在测试环境中运行命令`pidof -w kthreadd`
- 测试结果：运行成功，打印出线程号。

9. 功能点9：`-S, --separator <separator>...`，指定分隔符

- 测试方法：在测试环境中运行命令`pidof bash -S -`
- 测试结果：运行成功，每个PID之间用`-`分隔开。

10. 功能点10：打印版本信息和帮助信息

- 测试方法：在测试环境中运行命令`pidof -h`和`pidof -v`
- 测试结果：运行成功，打印出版本信息和帮助信息。

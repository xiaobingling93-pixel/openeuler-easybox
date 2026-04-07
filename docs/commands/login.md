## login功能对比验证报告

### 软件支持的功能清单

| 软件功能 | 原有软件 | Rust重写后的软件 | 是否自动化测试 |
| --- | --- | --- | --- |
| login [选项] [-h 主机名] [用户名] | 支持  | 支持  | 是 |
| -p 不要清空环境信息 | 支持  | 支持  | 是 |
| -f 跳过登录认证 | 支持  | 支持  | 是 |
| -h 用于在utmp中记录的主机名 | 支持  | 支持  | 是 |
| -H 在登录提示符中不显示主机名 | 支持  | 支持  | 是 |
| --help 显示程序的帮助信息 | 支持 | 支持  | 否* |
| -V, --version 显示程序的版本信息 | 支持 | 支持  | 否* |

注：`--help，--version，-V` 选项输出的帮助信息带有版权信息、不同的版本号输出，无法直接进行对比。

### 软件自带用例对比验证

util-linux 软件包中有一份关于 login 配置文件解析的测试用例，下面介绍该样例分别在 util-linux 与 easybox 上分别测试的方法和结果：

1. 给定如下 logindefs.data 作为 login 配置文件测试用例：
```
#
# this is /etc/login.defs sample
#

HELLO_WORLD	"hello world!"
STRING		this_is_string		# another comment
NUMBER		123456
BOOLEAN		yEs

CRAZY1 = "this is crazy format"
CRAZY2=fooBar
CRAZY3    FoooBaaar

EMPTY

END	"the is end"
```
2. 在 util-linux 编译输出目录下运行`./test_logindefs logindefs.data`
![图1](https://gitee.com/xjl12/easybox/raw/wiki/img/login_img1.png)
3. 在 easybox 中 login 对应的测试项目中新增 test_logindefs_parse 函数，直接内部调用 login 模块的 load_defaults() 及 dump_list() 函数，进行对比：
<img src="https://gitee.com/xjl12/easybox/raw/wiki/img/login_img2.png" alt="图2" style="zoom: 67%;" />

经比对，二者输出结果除顺序外，其余数据均一致。

### 功能对比验证

考虑到 login 命令需要用户交互才能正常执行各种功能，所以使用 `expect` 脚本来进行交互控制。以下所有操作均以 root 用户执行。

在测试前，需要先执行`useradd login_test -MN -d / -p bIgJASE9rgy2g`命令，创建 login_test 用户，并设置密码为 123。

每个功能点测试时，均分别运行`/usr/bin/login`和`easybox login`，在去除时间等动态变化的内容后对输出进行比较，若两个程序输出和返回值保持一致，则测试通过。测试用例清单如下：

| 测试名称                               | 测试功能                        | 测试情况 |
| -------------------------------------- | ------------------------------- | -------- |
| test_normal_login_procedure            | 正常登录并获取环境变量          | 通过     |
| test_login_failed_procedure            | 空密码登录，登录失败            | 通过     |
| test_login_specific_username_procedure | `username` 登录指定用户         | 通过     |
| test_login_keep_environment            | `-p` 登录时保留环境变量         | 通过     |
| test_login_skip_auth                   | `-f` 登录时跳过认证             | 通过     |
| test_login_supress_hostname            | `-H` 在登录提示符中不显示主机名 | 通过     |
| test_login_utmp_hostname               | `-h` 登录时指定主机名           | 通过     |
| test_logindefs_parse                   | login 配置文件解析功能          | 通过     |

1. 功能点：`空选项`  正常登录并获取环境变量

- 测试方法：在测试环境中分别以`/usr/bin/login`和`easybox login`为参数运行如下`expect`脚本：
    ```
    set timeout 60
    if {$argc == 1} {
        spawn [lindex $argv 0]
    } else {
        spawn [lindex $argv 0] [lindex $argv 1]
    }
    exec sleep 1
    expect "login:"
    send "login_test\r"
    expect "Password:"
    send "123\r"
    expect "login_test"
    exec sleep 0.5
    send "env && exit\r"
    expect eof
    ```
- 测试结果：命令成功运行，用户成功登录并输出环境变量。
  
2. 功能点：`空选项` 空密码登录，测试登录场景

- 测试方法：在测试环境中分别以`/usr/bin/login`和`easybox login`为参数运行如下`expect`脚本：
    ```
    set timeout 60
    if {$argc == 1} {
        spawn [lindex $argv 0]
    } else {
        spawn [lindex $argv 0] [lindex $argv 1]
    }
    
    exec sleep 1
    expect "login:"
    send "login_test\r"
    expect "Password:"
    send "\r"
    ```
- 测试结果：用户无法登录，login 报错退出。
3. 功能点：`username` 登录指定用户
- 测试方法：在测试环境中分别以`/usr/bin/login`和`easybox login`为参数运行如下`expect`脚本：
    ```
    set timeout 60
    if {$argc == 1} {
        spawn [lindex $argv 0] login_test
    } else {
        spawn [lindex $argv 0] [lindex $argv 1] login_test
    }
    
    exec sleep 1
    expect "Password:"
    send "123\r"
    
    expect "login_test"
    exec sleep 0.5
    send "exit\r"
    expect eof
    ```
- 测试结果：命令成功运行，指定的用户成功登录。
4. 功能点：`-p` 登录时保留环境变量
- 测试方法：在测试环境中分别以`/usr/bin/login`和`easybox login`为参数运行如下`expect`脚本：
    ```
    set timeout 60
    if {$argc == 1} {
        spawn [lindex $argv 0] -p
    } else {
        spawn [lindex $argv 0] [lindex $argv 1] -p
    }
    exec sleep 1
    expect "login:"
    send "login_test\r"
    expect "Password:"
    send "123\r"
    expect "login_test"
    exec sleep 0.5
    send "env && exit\r"
    expect eof
    ```
- 测试结果：命令成功运行，登录后可观察到系统所有环境变量均已保留。
5. 功能点：`-f` 登录时跳过认证
- 测试方法：在测试环境中分别以`/usr/bin/login`和`easybox login`为参数运行如下`expect`脚本：
    ```
    set timeout 60
    if {$argc == 1} {
        spawn [lindex $argv 0] -f login_test
    } else {
        spawn [lindex $argv 0] [lindex $argv 1] -f login_test
    }
    exec sleep 1
    expect "login_test"
    exec sleep 0.5
    send "whoami && exit\r"
    expect eof
    ```
- 测试结果：命令成功运行，指定的用户无需输入密码即可登录。
6. 功能点：`-H` 在登录提示符中不显示主机名
- 测试方法：在测试环境中分别以`/usr/bin/login`和`easybox login`为参数运行如下`expect`脚本：
    ```
    set timeout 60
    if {$argc == 1} {
        spawn [lindex $argv 0] -H
    } else {
        spawn [lindex $argv 0] [lindex $argv 1] -H
    }
    exec sleep 1
    expect "login:"
    send "login_test\r"
    expect "Password:"
    send "123\r"
    expect "login_test"
    exec sleep 0.5
    send "whoami && exit\r"
    expect eof
    ```
- 测试结果：命令成功运行，login登录提示符未显示主机名。
7. 功能点：`-h` 登录时指定主机名
- 测试方法：在测试环境中分别以`/usr/bin/login`和`easybox login`为参数运行如下`expect`脚本：
    ```
    set timeout 60
    if {$argc == 1} {
        spawn [lindex $argv 0] -h test.easybox.host
    } else {
        spawn [lindex $argv 0] [lindex $argv 1] -h test.easybox.host
    }
    exec sleep 1
    expect "login:"
    send "login_test\r"
    expect "Password:"
    send "123\r"
    expect "login_test"
    exec sleep 0.5
    send "who | grep test.easybox.host > /dev/null && echo found\r"
    expect "login_test"
    exec sleep 0.5
    send "exit\r"
    expect eof
    ```
- 测试结果：命令成功运行，登录后运行`who`命令，检查当前登录的远程主机名，可以从输出中找到`test.easybox.host`值。
8. 功能点：`内部测试` login 配置文件解析功能
- 测试方法：解析一段 login.defs 文件样例。详见上节"软件自带用例对比验证"。
- 测试结果：解析成功，得到的数据正确。

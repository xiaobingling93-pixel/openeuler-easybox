## 功能对比验证报告
### 软件支持的功能清单

| 软件功能 | 原有软件 | Rust重写后的软件 |
| --- | --- | --- |
| flock \[选项\] <文件\|目录> <命令> \[<参数>...\]| 支持 | 支持 |
| flock \[选项\] <文件\|目录> -c <命令> | 支持 | 支持 |
| flock \[选项] <文件描述符号码> | 支持 | 支持 |
| -s, --shared 获取共享锁 | 支持 | 支持 |
| -x, --exclusive 获取排他锁 | 支持 | 支持 |
| -u, --unlock 移除锁 | 支持 | 支持 |
| -n, --nonblock 获取锁失败不等待 | 支持 | 支持 |
| -w, --timeout <秒> 等待限定的时间 | 支持 | 支持 |
| -E, --conflict-exit-code <数字> 冲突或超时后的退出代码 | 支持 | 支持 |
| -o, --close 运行命令前关闭文件描述符 | 支持 | 支持 |
| -c, --command <命令> 通过shell运行单个命令字符串 | 支持 | 支持 |
| -F, --no-fork 执行命令时不fork | 支持 | 支持 |
| --verbose 增加详尽程度 | 支持 | 支持 |
| -h, --help 显示帮助 | 支持 | 支持 |
| -V, --version 显示版本 | 支持 | 支持 |


### 软件自带用例对比验证
软件自带的flock测试用例清单如下:
先进行一次flock操作，获取测试文件的共享锁并持有三秒钟，等待其它功能参数的测试
测试--non-block参数，由于测试文件共享锁被持有，因此在锁不能被立刻获取的情况下直接退出，并返回对应的退出代码
测试--no-fork参数，不fork进程执行命令，由于测试文件共享锁被持有，因此在锁不能被立刻获取的情况下直接退出，并返回对应的退出代码
测试--shared参数，请求测试文件的共享锁，并成功获取
测试--exclusive参数，请求独占锁，由于测试文件共享锁被持有，因此在锁不能被立刻获取的情况下直接退出，并返回对应的退出代码
测试通过文件描述符持有文件锁，由于rust测试环境缺少将测试文件绑定文件描述符的方法，rust测试样例中不测试这一部分
测试--timeout参数，等待最多5秒后，成功获取文件锁。检测等待时间应该为3秒钟。
以上测试需要连续执行，因此在一个测试用例中实现

测试用例均通过

easybox测试框架下的测试结果：

```
running 1 test
test test_flock::test_lock ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 338 filtered out; finished in 0.06s
```

### 功能对比验证
1. 功能点1：共享锁上锁测试
- 测试方法：在测试环境中运行`flock --shared --conflict-exit-code 123 lockfile.txt -c echo \"Locking\"; sleep 3 ;echo \"Unlocking\" > outputfile.txt 2>&1 &`
- 测试结果：文件成功持有共享锁并持续3秒，后续功能点2-6测试在功能点1测试占有文件锁的时间段中进行连续进行

2. 功能点2：测试nonblock参数
- 测试方法：在测试环境中运行`flock --nonblock --conflict-exit-code 123 lockfile.txt echo "you will never see this"`
- 测试结果：无法持有锁，立即退出并返回错误码123

3. 功能点3：测试no-fork参数
- 测试方法：在测试环境中运行`flock --no-fork --nonblock --conflict-exit-code 123 lockfile.txt echo "you will never see this"`
- 测试结果：无法持有锁，立即退出并返回错误码123

4. 功能点4：测试shared参数
- 测试方法：在测试环境中运行`flock --shared lockfile.txt echo "Have shared lock"`
- 测试结果：成功获取文件的共享锁

5. 功能点5：测试exclusive参数
- 测试方法：在测试环境中运行`flock --nonblock --exclusive --conflict-exit-code 123 lockfile.txt echo "you will never see this"`
- 测试结果：无法持有锁，立即退出并返回错误码123

6. 功能点6：测试timeout参数
- 测试方法：在测试环境中运行`flock --timeout 5 --conflict-exit-code 5 lockfile.txt echo "After timeout"`，并计算测试用例执行时间。
- 测试结果：在等待3秒功能1释放文件锁后，成功获取锁

7. 功能点7：锁定fd文件
- 测试方法：在测试环境中运行`exec 5<>lockfile.txt`对文件绑定文件描述符，再执行`flock 5`
- 测试结果：成功获取文件描述符锁

8. 功能点8：测试unlock参数
- 测试方法：在测试环境中运行`flock lockfile.txt sleep 3`获取3秒钟的锁。同时在测试环境中执行另一指令`flock --unlock text.txt echo test`，查看是否成功获取锁
- 测试结果：在另一文件锁定的情况下，unlock参数解锁文件并获取锁。

9. 功能点9：测试help和version参数
- 测试方法：在测试环境中运行`flock --help`和`flock --version`。
- 测试结果：成功获取帮助信息与版本信息。

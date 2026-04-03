## 功能对比验证模版
### 软件支持的功能清单
| 软件功能 | 原有软件 | Rust重写后的软件 |
| --- | --- | --- |
| pgrep [options] \<pattern\> | 支持 | 支持 |
| -d, --delimiter \<string\> 指定输出分隔符 | 支持 | 支持 |
| -l, --list-name 列出PID和进程名称 | 支持 | 支持 |
| -a, --list-full 列出PID和完整命令行指令 | 支持 | 支持 |
| -v, --inverse 反转匹配结果 | 支持 | 支持 |
| -w, --lightweight 列出所有TID | 支持 | 支持 |
| -c, --count 统计匹配的进程数量 | 支持 | 支持 |
| -f, --full 使用完整的进程名称来匹配 | 支持 | 支持 |
| -g, --pgroup \<PGID,...\> 匹配RGID | 支持 | 支持 |
| -G, --group \<GID,...\> 匹配GID | 支持 | 支持 |
| -i, --ignore-case 匹配时忽略大小写 | 支持 | 支持 |
| -n, --newest 得到开始时间最晚的匹配进程 | 支持 | 支持 |
| -o, --oldest 得到开始时间最早的匹配进程 | 支持 | 支持 |
| -O, --older \<seconds\> 选择比指定秒数更老的进程 | 支持 | 支持 |
| -P, --parent \<PPID,...\> 仅匹配给定父进程的子进程 | 支持 | 支持 |
| -s, --session \<SID,...\> 匹配SID | 支持 | 支持 |
| --signal \<sig\> 匹配信号 | 支持 | 支持 |
| -t, --terminal \<tty,...\> 匹配控制终端 | 支持 | 支持 |
| -u, --euid \<ID,...\> 匹配EUID | 支持 | 支持 |
| -U, --uid \<ID,...\> 匹配UID | 支持 | 支持 |
| -x, --exact 与命令名称完全匹配 | 支持 | 支持 |
| -F, --pidfile \<file\> 从文件中读取PID | 支持 | 支持 |
| -L, --logpidfile 如果PID文件未锁定则失败 | 支持 | 支持 |
| -r, --runstates \<state\> 匹配运行状态[D,S,Z,...] | 支持 | 支持 |
| -A, --ignore-ancestors 将祖先进程排除在结果之外 | 支持 | 支持 |
| --cgroup \<grp,...\> 匹配cgroup v2名称 | 支持 | 支持 |
| --ns \<PID\> 匹配与PID属于同一命名空间的进程 | 支持 | 支持 |
| --nslist \<ns,...\> 指定--ns选项使用的命名空间 | 支持 | 支持 |
| --env \<name=val,...\> 匹配环境变量 | 支持 | 支持 |
| -h, --help 显示帮助信息 | 支持 | 支持 |
| -V, --version 显示版本信息 | 支持 | 支持 |

### 软件自带用例对比验证
软件具有自带测试用例，以下使用pgrep的最新版本进行测试（Commit ID：21f6017b）：

| 测试用例                          | 测试目的 | 为自带测试用例 | 已集成至测试代码 | 版本要求 |
| --------------------------------- | -------- | :----------------: | :------------------: | --------------------------------- |
| pgrep | 错误处理 | √ | √ |  |
| pgrep \<test_proc_comm\> | 基础功能 |         √          |          √           |                     |
| pgrep -c \<test_proc_comm\> | -c选项   |         √          |          √           |                     |
| pgrep -d : \<test_proc_comm\> | -d选项   |         √          |          √           |                     |
| pgrep -f \<test_proc_full\> | -f选项   |         √          |          √           |                     |
| pgrep -G \<gid\> \<test_proc_comm\> | -G选项 |         √          |          √           |                     |
| pgrep -l \<test_proc_comm\> | -l 选项 |         √          |          √           |                     |
| pgrep -af \<test_proc_path> | -a/-f选项 |         √          |          √           |                     |
| pgrep -n \<test_proc_comm\> | -n选项 |         √          |          √           |                     |
| pgrep -o \<test_proc_comm\> | -o选项 |         √          |          √           |                     |
| pgrep -P \<my_pid\> \<test_proc_comm\> | -P选项 |         √          |          √           |                     |
| pgrep -P \<not_ppid\> \<test_proc_comm\> | -P选项 |         √          |          √           |                     |
| pgrep -s \<test_proc_1_sid\> \<test_proc_comm\> | -s选项 |         √          |          √           |                     |
| pgrep -s \<not_test_proc_1_sid\> \<test_proc_comm\> | -s选项 |         √          |          √           |                     |
| pgrep -t \<tty\> \<test_proc_comm\> | -t选项 |         √          |                     |                     |
| pgrep -t glass \<test_proc_comm\> | -t选项 |         √          |          √           |                     |
| pgrep -u \<uid\> \<test_proc_comm\> | -u选项 |         √          |          √           |                     |
| pgrep -u \<not_uid\> \<test_proc_comm\> | -u选项 |         √          |          √           |                     |
| pgrep -U \<uid\> \<test_proc_comm\> | -U选项 |         √          |          √           |                     |
| pgrep -U \<not_uid\> \<test_proc_comm\> | -U选项 |         √          |          √           |                     |
| pgrep \<test_proc_trim\> | 基础功能 | √ | √ |  |
| pgrep -x \<test_proc_comm\> | -x选项 | √ | √ |  |
| pgrep -x \<test_proc_trim\> | -x选项 | √ | √ |  |
| pgrep gnome-session-bi | 错误处理 | √ | √ | >=4.0.0 |
| pgrep -c no_matching | -c选项 |                    | √ |                      |
| pgrep -d : -d _ \<test_proc_comm\> | -d选项 |                    | √ |                      |
| pgrep -d _ -d : \<test_proc_comm\> | -d选项 |                    | √ |                      |
| pgrep -g \<test_proc_1_pgid\> \<test_proc_comm\> | -g选项 |                    | √ |                      |
| pgrep -g \<not_test_proc_1_pgid\> \<test_proc_comm\> | -g选项 |                    | √ |                      |
| pgrep -g \<multiple_pgids\> \<test_proc_comm\> | -g选项 |                    | √ |                      |
| pgrep -G \<multiple_gids\> \<test_proc_comm\> | -G选项 |                    | √ |                      |
| pgrep -i \<test_proc_upper\> | -i选项 |                    | √ |                      |
| pgrep -n no_matching | -n选项 |                    | √ |                      |
| pgrep -o no_matching | -o选项 |                    | √ |                      |
| pgrep -O \<test_proc_comm\> | -O选项 |                    | √ |                      |
| pgrep -O no_matching | -O选项 |                    | √ |                      |
| pgrep -P \<multiple_ppids\> \<test_proc_comm\> | -P选项 |                    | √ |                      |
| pgrep -r D \<test_proc_comm\> | -r选项 |                    | √ |                      |
| pgrep -r S \<test_proc_comm\> | -r选项 |                    | √ |                      |
| pgrep -r DS \<test_proc_comm\> | -r选项 |                    | √ |                      |
| pgrep -r SD \<test_proc_comm\> | -r选项 | | √ | |
| pgrep -s \<multiple_sids\> \<test_proc_comm\> | -s选项 | | √ | |
| pgrep -u \<multiple_uids\> \<test_proc_comm\> | -u选项 | | √ | |
| pgrep -U \<multiple_uids\> \<test_proc_comm\> | -U选项 | | √ | |
| pgrep -U pattern1 pattern2 | 错误处理 | | √ | |
| pgrep -A init | -A选项 | | √ | >=4.0.3 |
| pgrep -O 0 | -O选项 | | √ | |
| pgrep -t pts/10 -v | -t选项 | | √ | |
| pgrep -t tty10 -v | -t选项 | | √ | |
| pgrep -t ttyS10 -v | -t选项 | | √ | |
| pgrep -t pts/10,pts/11 -v | -t选项 | | √ | |
| pgrep -t ? -v | -t选项 | | √ | |
| pgrep -v -P \<my_pid\> \<test_proc_comm\> | -v选项 | | √ | |
| pgrep --cgroup / -v | --cgroup选项 | | √ | >=4.0.0 |
| pgrep --cgroup /init.scope -v | --cgroup选项 | | √ | >=4.0.0 |
| pgrep --env SHELL=/bin/bash -v | --env选项 | | √ | >=4.0.0 |
| pgrep -g invalid_pgid | 错误处理 | | √ | |
| pgrep -G invalid_gid | 错误处理 | | √ | |
| pgrep -P invalid_ppid | 错误处理 | | √ | |
| pgrep -r invalid_state | 错误处理 | | √ | |
| pgrep -s invalid_sid | 错误处理 | | √ | |
| pgrep -t invalid_terminal | 错误处理 | | √ | |
| pgrep -t /dev/pts/1 | 错误处理 | | √ | |
| pgrep -u invalid_id | 错误处理 | | √ | |
| pgrep -U invalid_id | 错误处理 | | √ | |
| pgrep -O invalid_seconds | 错误处理 | | √ | |
| pgrep -F - \<test_proc_comm\> | -F选项 | | √ | >4.0.4 |
| pgrep -F \<pidfile\> \<test_proc_comm\> | -F选项 | | √ | |

以上测试已集成至测试代码中，测试代码执行结果如下：

![pgrep](https://foruda.gitee.com/images/1724195082078672475/29f8eac0_7603128.png "pgrep.png")

### 功能对比验证
1. 功能点1：基础功能
    - 测试方法：在测试环境中运行`sleep 1 & easybox pgrep sleep`
    - 测试结果：输出sleep进程的PID
2. 功能点2：-d, --delimiter \<string\> 指定输出分隔符
    - 测试方法：在测试环境中运行`sleep 1 & sleep 1 & easybox pgrep -d , sleep`
    - 测试结果：输出用,分割的sleep进程的PID
3. 功能点3：-l, --list-name 列出PID和进程名称
    - 测试方法：在测试环境中运行`sleep 1 & easybox pgrep -l sleep`
    - 测试结果：输出sleep进程的PID和进程名称
4. 功能点4：-a, --list-full列出PID和完整命令行指令
    - 测试方法：在测试环境中运行`sleep 1 & easybox pgrep -a sleep`
    - 测试结果：输出sleep进程的PID和完整命令行指令
5. 功能点5：-v, --inverse 反转匹配结果
    - 测试方法：在测试环境中运行`sleep 1 & easybox pgrep -v sleep`
    - 测试结果：输出结果中不包含sleep的PID
6. 功能点6：-c, --count 统计匹配的进程数量
    - 测试方法：在测试环境中运行`sleep 1 & easybox pgrep -c sleep`
    - 测试结果：输出为1
7. 功能点7：-f, --full 使用完整的进程名称来匹配
    - 测试方法：在测试环境中运行`sleep 1 & easybox pgrep -f sleep`
    - 测试结果：输出sleep进程的PID
8. 功能点8：-g, --pgroup \<PGID,...\>
    - 测试方法：在测试环境中运行`sleep 1 & easybox pgrep -g $(ps --no-headers -o pgid $(pgrep sleep)) sleep`
    - 测试结果：输出sleep进程的PID
9. 功能点9：-G, --group \<GID,...\> 匹配GID
    - 测试方法：在测试环境中运行`sleep 1 & easybox pgrep -G $(ps --no-headers -o gid $(pgrep sleep)) sleep`
    - 测试结果：输出sleep进程的PID
10. 功能点10：-i, --ignore-case 匹配时忽略大小写
    - 测试方法：在测试环境中运行`sleep 1 & easybox pgrep -i SLEEP`
    - 测试结果：输出sleep进程的PID
11. 功能点11：-n, --newest 得到开始时间最晚的匹配进程
     - 测试方法：在测试环境中运行`sleep 1 & sleep 1 & easybox pgrep -n sleep`
     - 测试结果：输出第二个sleep进程的PID
12. 功能点12：-o, --oldest 得到开始时间最早的匹配进程
     - 测试方法：在测试环境中运行`sleep 1 & sleep 1 & easybox pgrep -o sleep`
     - 测试结果：输出第一个sleep进程的PID
13. 功能点13：-P, --parent \<PPID,...\> 仅匹配给定父进程的子进程
     - 测试方法：在测试环境中运行`setsid -fw sleep 1 & easybox pgrep -P $(pgrep setsid) sleep`
     - 测试结果：输出sleep进程的PID
14. 功能点14：-s, --session \<SID,...\> 匹配SID
     - 测试方法：在测试环境中运行`sleep 1 & easybox pgrep -s $(ps --no-headers -o sid $(pgrep sleep)) sleep`
     - 测试结果：输出sleep进程的PID
15. 功能点15：-t, --terminal \<tty,...\> 匹配控制终端
     - 测试方法：在测试环境中运行`sleep 1 & easybox pgrep -t $(ps --no-headers -o tty $(pgrep sleep)) sleep`
     - 测试结果：输出sleep进程的PID
16. 功能点16：-u, --euid \<ID,...\> 匹配EUID
     - 测试方法：在测试环境中运行`sleep 1 & easybox pgrep -u $(ps --no-headers -o euid $(pgrep sleep)) sleep`
     - 测试结果：输出sleep进程的PID
17. 功能点17：-U, --uid \<ID,...\> 匹配UID
     - 测试方法：在测试环境中运行`sleep 1 & easybox pgrep -U $(ps --no-headers -o uid $(pgrep sleep)) sleep`
     - 测试结果：输出sleep进程的PID
18. 功能点18：-x, --exact 与命令名称完全匹配
     - 测试方法：在测试环境中运行`sleep 1 & easybox pgrep -x slee`
     - 测试结果：无输出
19. 功能点19：-F, --pidfile \<file\> 从文件中读取PID
     - 测试方法：在测试环境中运行`easybox pgrep -F - init`并输入2
     - 测试结果：输出2（/init进程的PID）
20. 功能点20：-r, --runstates \<state\> 匹配运行状态[D,S,Z,...]
     - 测试方法：在测试环境中运行`sleep 1 & easybox pgrep -r $(ps --no-headers -o stat $(pgrep sleep)) sleep`
     - 测试结果：输出sleep进程的PID
21. 功能点21：-A, --ignore-ancestors 将祖先进程排除在结果之外
     - 测试方法：在测试环境中运行`easybox pgrep -A init`
     - 测试结果：结果中不包含2（/init进程的PID）
22. 功能点22：--cgroup \<grp,...\> 匹配cgroup v2名称
     - 测试方法：在测试环境中运行`easybox pgrep --cgroup /init.scope`
     - 测试结果：输出1
23. 功能点23：--env \<name=val,...\> 匹配环境变量
     - 测试方法：在测试环境中运行`export TEST_ENV="TEST"`和`sleep 1 & easybox pgrep --env TEST_ENV=TEST sleep`
     - 测试结果：输出sleep进程的PID
24. 功能点24：-h --help 显示帮助信息
     - 测试方法：在测试环境中运行`easybox pgrep -h`
     - 测试结果：显示帮助信息
25. 功能点25：-V --version 显示版本信息
     - 测试方法：在测试环境中运行`easybox pgrep -V`
     - 测试结果：显示setsid的版本信息

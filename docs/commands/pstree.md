# 功能验证对比报告
## 软件支持的功能清单

| 软件功能 | 原有软件 | Rust重写后的软件 |
| ---- | ---- | ---- |
| easybox pstree \[-acglpsStTuZ\] \[ -h \| -H PID \] \[ -n \| -N type \] \[ -A \| -G \| -U \] \[ PID \| USER \] | 支持 | 支持 |
| easybox pstree -V | 支持 | 支持 |
| -a, --arguments  显示命令行参数 | 支持 | 支持 |
| -A, --ascii  使用 ASCII 行绘制字符 | 支持 | 支持 |
| -c, --compact-not  不要对完全相同的子树进行压缩 | 支持 | 支持 |
| -C, --color=类型\<age\>  按照指定属性对进程上色 | 支持 | 支持 |
| -g, --show-pgids  显示进程组 ID；隐含启用 -c 选项 | 支持 | 支持 |
| -G, --vt100  使用 VT100 行绘制字符 | 支持 | 支持 |
| -h, --highlight-all  高亮显示当前进程和其所有祖先 | 支持 | 支持 |
| -H PID, --highlight-pid=PID  高亮显示指定 PID 对应的进程和其所有祖先 | 支持 | 支持 |
| -l, --long  不要截断长行 | 支持 | 支持 |
| -n, --numeric-sort  按照 PID 对输出进行排序 | 支持 | 支持 |
| -N 类型, --ns-sort=类型\<cgroup, ipc, mnt, net, pid, time, user, uts\>  按照指定命名空间类型对输出进行排序 | 支持 | 支持 |
| -p, --show-pids  显示 PID；隐含启用 -c 选项 | 支持 | 支持 |
| -s, --show-parents  显示所选进程的父进程 | 支持 | 支持 |
| -S, --ns-changes  显示命名空间的变化 | 支持 | 支持 |
| -t, --thread-names  显示完整线程名称 | 支持 | 支持 |
| -T, --hide-threads  隐藏线程，只显示进程 | 支持 | 支持 |
| -u, --uid-changes  显示用户 ID（UID）的变化 | 支持 | 支持 |
| -U, --unicode  使用 UTF-8（Unicode）的行绘制字符 | 支持 | 支持 |
| -Z, --security-context  显示安全属性 | 支持 | 支持 |
| -h, --help 显示帮助 | 支持 | 支持 |
| -V, --version 显示版本 | 支持 | 支持 |
## 软件自带用例对比验证
软件没有自带测试用例，在Rust测试环境中添加如下测试用例：
测试pstree基本功能，检测输出是否有easybox进程
测试pstree打印PID 1的进程及其子进程
测试pstree --arguments参数输出
测试pstree --compact-not参数输出
测试pstree --ascii参数输出
测试pstree --color=age参数输出
测试pstree --show-pgids参数输出
测试pstree --vt100参数输出
测试pstree --highlight-all参数输出
测试pstree --highlight-pid=1参数输出
测试pstree --long参数输出
测试pstree --numeric-sort参数输出
测试pstree --ns-sort=cgroup参数输出
测试pstree --show-pids参数输出
测试pstree --show-parents参数输出
测试pstree --ns-changes参数输出
测试pstree --thread-names参数输出
测试pstree --hide-threads参数输出
测试pstree --uid-changes参数输出
测试pstree --unicode参数输出
测试pstree --security-context参数输出

## 功能对比验证
1. 功能点1：--arguments显示命令行参数
- 测试方法：在测试环境中运行`easybox pstree --arguments`
- 测试结果：进程树中的所有进程会显示该进程启动时的命令行参数

2. 功能点2：--compact-not不对完全相同的子树进行压缩
- 测试方法：在测试环境中运行`easybox pstree --compact-not`
- 测试结果：进程树中的会显示完全相同的子树节点

3. 功能点3：--ascii使用 ASCII行绘制字符
- 测试方法：在测试环境中运行`easybox pstree --ascii`
- 测试结果：进程树使用ASCII字符的格式进行绘制

4. 功能点4：--color=age按照运行时间属性对进程上色
- 测试方法：在测试环境中运行`easybox pstree --color=age`
- 测试结果：进程树中的进程根据运行时间长短以红黄绿色进行显示
  
5. 功能点5：--show-pgids显示进程组ID
- 测试方法：在测试环境中运行`easybox pstree --show-pgids`
- 测试结果：进程树中的进程显示进程组ID

6. 功能点6：--vt100使用VT100行绘制字符
- 测试方法：在测试环境中运行`easybox pstree --vt100`
- 测试结果：进程树使用VT100行绘制字符

7. 功能点7：--highlight-all高亮显示当前进程和其所有祖先
- 测试方法：在测试环境中运行`easybox pstree --highlight-all`
- 测试结果：进程树高亮显示当前进程和其所有祖先

8. 功能点8：--highlight-pid=PID高亮显示指定 PID 对应的进程和其所有祖先
- 测试方法：在测试环境中运行`easybox pstree --highlight-pid=1`
- 测试结果：进程树 高亮显示指定 PID 对应的进程和其所有祖先

9. 功能点9：--long不截断长行
- 测试方法：在测试环境中运行`easybox pstree --long`
- 测试结果：进程树不根据窗口大小截断长行

10. 功能点10：--numeric-sort按照 PID 对输出进行排序
- 测试方法：在测试环境中运行`easybox pstree --numeric-sort`
- 测试结果：进程树根据PID大小排序显示进程

11. 功能点11：--ns-sort=类型 按照指定命名空间类型对输出进行排序
- 测试方法：在测试环境中运行`easybox pstree --ns-sort=cgroup`
- 测试结果：进程树显示指定命名空间类型

12. 功能点12：--show-pids显示 PID
- 测试方法：在测试环境中运行`easybox pstree --show-pids`
- 测试结果：进程树显示进程PID

13. 功能点13：--show-parents显示所选进程的父进程
- 测试方法：在测试环境中运行`easybox pstree 2 --show-parents`
- 测试结果：进程树显示所指定进程号的父进程

14. 功能点14：--ns-changes显示命名空间的变化
- 测试方法：在测试环境中运行`easybox pstree --ns-changes`
- 测试结果：进程树显示进程命名空间的变化

15. 功能点15：--thread-names显示完整线程名称
- 测试方法：在测试环境中运行`easybox pstree --thread-names`
- 测试结果：进程树显示进程的完整线程名称

16. 功能点16：--hide-threads隐藏线程，只显示进程
- 测试方法：在测试环境中运行`easybox pstree --hide-threads`
- 测试结果：进程树隐藏线程，只显示进程

17. 功能点17：--uid-changes显示用户 ID（UID）的变化
- 测试方法：在测试环境中运行`easybox pstree --uid-changes`
- 测试结果：进程树显示用户 ID（UID）的变化

18. 功能点18：--unicode显示用户 ID（UID）的变化
- 测试方法：在测试环境中运行`easybox pstree --unicode`
- 测试结果：进程使用UTF-8行绘制进程树

19. 功能点19：--security-context显示安全属性
- 测试方法：在测试环境中运行`easybox pstree --security-context`
- 测试结果：进程树显示进程的安全属性信息

20. 功能点20：测试pstree基本输出
- 测试方法：在测试环境中运行`easybox pstree`
- 测试结果：正确显示进程树

21. 功能点21：测试pstree \<pid\>输出
- 测试方法：在测试环境中运行`easybox pstree 1`
- 测试结果：进程树显示pid 1的进程及其子进程

22. 功能点22：--version显示pstree版本信息
- 测试方法：在测试环境中手动运行`easybox pstree --version`
- 测试结果：正确显示pstree的版本信息

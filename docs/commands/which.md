### 软件支持的功能清单

| 软件功能                                           | 原有软件 | Rust重写后的软件 | 是否自动化测试 |
| ---------------------------------------------- | ---- | ---------- | -------------- |
| which [选项]  [--] 命令 [参数]                       | 支持   | 支持         | 是             |
| -a, --all 打印PATH变量中的所有匹配项，而不只是第一个              | 支持   | 支持         | 是             |
| -i, --read-alias 从标准输入读取alias列表                | 支持   | 支持         | 是             |
| --read-functions 从标准输入读取shell函数                | 支持   | 支持         | 是             |
| --show-dot 在输出中不展开当前目录的点                       | 支持   | 支持         | 是             |
| --show-tilde 输出非根目录下家目录的`~`符号                  | 支持   | 支持         | 是             |
| --skip-alias 忽略选项 --read-alias，不读取标准输入         | 支持   | 支持         | 是             |
| --skip-dot 跳过PATH变量中以点开头的目录                    | 支持   | 支持         | 是             |
| --skip-functions 忽略选项 --read-functions，不读取标准输入 | 支持   | 支持         | 是             |
| --skip-tilde 跳过PATH变量中以`~`开头的目录                | 支持   | 支持         | 是             |
| --tty-only 如果程序不运行在终端上，则停止处理右侧的选项              | 支持   | 支持         | 是             |
| -h, --help 显示程序的帮助信息                           | 支持   | 支持         | 否             |
| -V, --version 显示程序的版本信息                        | 支持   | 支持         | 否             |

### 软件自带用例对比验证

软件源码中附带有EXAMPLES文件，该文件含有一系列测试命令，用以测试which的基本功能。在Rust测试环境中添加11个测试用例，下面是测试情况：

running 11 tests
test test_which::test_split_command_options ... ok
test test_which::test_show_tilde_show_dot ... ok
test test_which::test_show_dot_skip_dot ... ok
test test_which::test_error_handle ... ok
test test_which::test_absolute_path_show_dot ... ok
test test_which::test_all_multi_commands ... ok
test test_which::test_permission_filter ... ok
test test_which::test_read_alias_skip_alias ... ok
test test_which::test_skip_tilde ... ok
test test_which::test_read_functions_skip_functions ... ok

将EXAMPLES文件中的测试命令转写为如下脚本程序：

```bash
#! /bin/bash
umask 022
shopt -s expand_aliases
export PATH=".:~/bin:/bin:/usr/bin"
export HOME=`(cd ~; pwd)`
alias which="$@"

touch cat; chmod 755 cat
alias

which --version
which -- --version

which cat
which --show-tilde cat
which --show-dot cat
which --show-tilde --show-dot cat
which --skip-dot cat

(cd /bin; which cat)
(cd /bin; which --show-dot cat)
(cd /bin; PATH=".:/bin:/usr/bin" which --show-dot cat)
(cd /bin; PATH="/bin:.:/usr/bin" which --show-dot cat)
(cd /bin; PATH=".:/bin:/usr/bin" which --skip-dot --show-dot cat)

which ls
which xxx
which ./ls

(cd /; which bin/ls)
(cd /; which --show-dot bin/ls)
(cd /; which --show-dot /bin/ls)
(cd /; which --show-dot bin/xxx)
(cd /; which --show-dot /bin/xxx)

which --all cat

touch xxx
which ./xxx

chmod 711 xxx
which ./xxx

chmod 655 cat
which cat

sudo chown root cat
which cat

sudo chmod 545 cat
which cat

sudo chgrp bin cat
which cat

sudo chmod 541 cat
which cat
sudo rm -f cat

sudo chown root xxx
which ./xxx

sudo chmod 700 xxx
which ./xxx

ls -l xxx
sudo chmod 750 xxx
sudo chgrp $GROUPS xxx
which ./xxx

sudo chgrp bin xxx
which ./xxx
sudo rm xxx

alias which="alias | $@ --tty-only --read-alias --show-tilde --show-dot"
alias test1='test1'
alias test2='echo "test2" | cat | sort'
alias test3='  echo "test2"|cat&sort'
alias test4='    ls &&sort ||/usr/bin/which || exit'

which which
which test1
which test2
which test3
which test4
```

在该脚本中，待测试的which程序由脚本参数决定，分别以C编写的原版which和Rust改写的which程序作为参数，二者运行对比如下图所示：

![截图1](https://foruda.gitee.com/images/1713494069897067737/e50671aa_13796108.png "截屏2024-04-18 19.46.14.png")

除version信息以及由于alias值不同（`alias which="alias | $@ --tty-only --read-alias --show-tilde --show-dot"`）导致的输出不一致外，其余输出一致。

### 功能对比验证

基于上述自带用例，编写Rust测试用例，并额外补充了一些自带测试用例尚未提及的选项，测试用例具体如下。

如无特殊声明，以下测试用例中PATH环境变量均为`.:~/bin:/bin:/usr/bin`。

1. 功能点：`--`分割参数与命令
- 测试方法：在测试环境中运行命令`which -- --version`

- 测试结果：命令运行结果、返回值与原有C程序保持一致。
2. 功能选项：`--show-tilde --show-dot`
- 测试方法：在测试环境中运行命令： 

- ```
  touch cat; chmod 755 cat
  which cat
  which --show-tilde cat
  which --show-dot cat
  which --show-tilde --show-dot cat
  which --skip-dot cat
  ```

- 测试结果：命令运行结果、返回值与原有C程序保持一致。
3. 功能选项：`--show-dot --skip-dot`
- 测试方法：在测试环境中运行命令： 

- ```
  (cd /bin; which cat)
  (cd /bin; which --show-dot cat)
  (cd /bin; PATH=".:/bin:/usr/bin" which --show-dot cat)
  (cd /bin; PATH="/bin:.:/usr/bin" which --show-dot cat)
  (cd /bin; PATH=".:/bin:/usr/bin" which --skip-dot --show-dot cat)
  ```

- 测试结果：命令运行结果、返回值与原有C程序保持一致。
4. 功能点：错误处理
- 测试方法：在测试环境中运行命令： 

- ```
  which ls
  which xxx
  which ./ls
  ```
- 测试结果：命令运行结果、返回值与原有C程序保持一致。
5. 功能点：绝对路径
- 测试方法：在测试环境中运行命令： 

- ```
  (cd /; which bin/ls)
  (cd /; which --show-dot bin/ls)
  (cd /; which --show-dot /bin/ls)
  (cd /; which --show-dot bin/xxx)
  (cd /; which --show-dot /bin/xxx)
  ```

- 测试结果：命令运行结果、返回值与原有C程序保持一致。
6. 功能选项：`--all` 与 多命令查找
- 测试方法：在测试环境中运行命令： 

- ```
  touch cat; chmod 755 cat
  which --all cat ls xxx yyy
  ```
- 测试结果：命令运行结果、返回值与原有C程序保持一致。
7. 功能点：文件权限过滤
- 测试方法：在测试环境中运行命令： 

- ```
  touch xxx cat
  chmod 755 cat
  chmod 
  
  which ./xxx
  
  chmod 711 xxx
  which ./xxx
  
  chmod 655 cat
  which cat
  
  sudo chown root cat
  which cat
  
  sudo chmod 545 cat
  which cat
  
  sudo chgrp bin cat
  which cat
  
  sudo chmod 541 cat
  which cat
  
  sudo chown root xxx
  which ./xxx
  
  sudo chmod 700 xxx
  which ./xxx
  
  sudo chmod 750 xxx
  sudo chgrp $GROUPS xxx
  which ./xxx
  
  sudo chgrp bin xxx
  which ./xxx
  ```
- 测试结果：命令运行结果、返回值与原有C程序保持一致。
8. 功能选项：`--tty-only --read-alias --show-tilde --show-dot` 以及 `--skip-alias`
- 测试方法：在测试环境中运行命令： 

- ```
  alias which="alias | which --tty-only --read-alias --show-tilde --show-dot"
  alias test1='test1'
  alias test2='echo "test2" | cat | sort'
  alias test3='  echo "test2"|cat&sort'
  alias test4='    ls &&sort ||/usr/bin/which || exit'
  
  which which
  which test1
  which test2
  which test3
  which test4
  which --skip-alias test4
  ```
- 测试结果：命令运行结果、返回值与原有C程序保持一致。
9. 功能选项：`--skip-tilde`
- 测试方法：在测试环境中运行命令：

- ```
  mkdir ~/bin
  touch ~/bin/aabb
  chmod 755 ~/bin/aabb
  which aabb
  which --skip-tilde aabb
  ```
- 测试结果：命令运行结果、返回值与原有C程序保持一致。
10. 功能选项：`--read-functions` 和 `--skip-functions`
- 测试方法：在测试环境中运行命令：

- ```
  cat > test_read_functions.in << EOF
  // bash version 2.0.5a and older output a pattern for `str' like
  declare -fx test1 ()
  {
      echo Hello world!
      echo Hello world!
      echo I am test1
  }
  
  // bash version 2.0.5b and later output a pattern for `str' like
  test2 ()
  {
      echo Hello world!
      echo Hello world!
      echo I am test2
  }
  
  // Add some zsh support here.
  // zsh does output a pattern for `str' like
  test3 () {
      echo Hello world!
      echo Hello world!
      echo I am test3
  }
  EOF
  which --read-functions test1 < test_read_functions.in
  which --read-functions test2 < test_read_functions.in
  which --read-functions test3 < test_read_functions.in
  which --read-functions --skip-functions test1 < test_read_functions.in
  ```
- 测试结果：命令运行结果、返回值与原有C程序保持一致。
11. 功能点：测试help和version参数
- 测试方法：在测试环境中运行`which --help`和`which --version`。
- 测试结果：命令运行成功，程序输出帮助信息与版本信息。


### 软件支持的功能清单

| 软件功能 | 原有软件 | Rust 重写后的软件 | 是否自动化测试 |
|---|---|---|---|
|`find [options] [starting_points] [expressions]`|支持|支持|是|
|`-H` 只在处理命令行参数时跟随符号链接|支持|支持|是|
|`-L` 跟随符号链接|支持|支持|是|
|`-P` 不跟随符号链接|支持|支持|是|
|`n \| +n \| -n` 检查数值等于 / 大于 / 小于 `n`|支持|支持|是|
|`exprs1 -a \| -and exprs2` 逻辑与|支持|支持|是|
|`epxrs1 -o \| -or exprs2` 逻辑或|支持|支持|是|
|`! \| -not exprs` 逻辑非|支持|支持|是|
|`( exprs )` 强制优先级|支持|支持|是|
|`expr1 , expr2` 依次执行表达式，返回最后的表达式值|支持|支持|是|
|`-daystart` 调整计时时间为当日零点|支持*|支持|是|
|`-follow` 默认跟随符号链接|支持|支持|是|
|`-regextype type` 设置正则表达式类型|支持|支持|是|
|`-d \| -depth` 优先处理目录内容而不是目录本身|支持|支持|是|
|`-files0-from name` 从文件读取搜索起点|支持|支持|是|
|`-help \| --help` 输出帮助信息并退出|支持|支持|否*|
|`-maxdepth n` 设置最大搜索层数|支持|支持|是|
|`-mindepth n` 设置最小搜索层数|支持|支持|是|
|`-mount` 不搜索不同设备的路径|支持|支持|是*|
|`-version \| --version` 输出版本信息并退出|支持|支持|否*|
|`-xdev` 同 `mount`|支持|支持|是|
|`-delete` 删除文件|支持|支持|是|
|`-exec` 执行命令|支持|支持|是|
|`-execdir` 切换目录并执行命令|支持|支持|是|
|`-fls file` 使用 `ls -dils` 格式输出元数据到文件|支持|支持|是|
|`-fprint file` 输出文件路径到文件并换行|支持|支持|是|
|`-fprint0 file` 输出文件路径到文件，以 `\0` 分隔|支持|支持|是|
|`-fprintf file format` 输出格式化字符串到文件|支持|支持|是|
|`-ls` 使用 `ls -dils` 格式输出元数据|支持|支持|是|
|`-ok` 在用户确认后执行命令|支持|支持|是|
|`-okdir` 在用户确认后切换目录并执行命令|支持|支持|是|
|`-print` 输出文件路径并换行|支持|支持|是|
|`-print0` 输出文件路径并以 `\0` 分隔|支持|支持|是|
|`-printf format` 输出格式化字符串|支持|支持|是|
|`-prune` 剪去当前路径搜索枝|支持|支持|是|
|`-quit` 立即退出程序|支持|支持|是|
|`-amin n` 检查文件访问时间（以分钟为单位）|支持|支持|是|
|`-anewer reference` 检查文件访问时间是否比 `reference` 新|支持|支持|是|
|`-atime n` 检查文件访问时间（以天为单位）|支持|支持|是|
|`-cmin n` 检查文件状态修改时间（以分钟为单位）|支持|支持|是|
|`-cnewer reference` 检查文件状态修改时间是否比 `reference` 新|支持|支持|是|
|`-ctime n` 检查文件状态修改时间（以天为单位）|支持|支持|是|
|`-empty` 检查文件是否为空|支持|支持|是|
|`-executable` 检查文件是否可执行|支持|支持|是|
|`-false` 直接返回假|支持|支持|是|
|`-fstype fs` 检查文件路径所在的设备 / 文件系统|支持|支持|是|
|`-gid id` 检查文件组 ID|支持|支持|是|
|`-group name\|id` 检查文件是否由`group` 组所有（支持组名与 GID）|支持|支持|是|
|`-ilname pattern` 检查文件是否为符号链接并指向 `name` glob 模式的文件（大小写不敏感）|支持|支持|是|
|`-iname pattern` 检查文件名是否满足 `name` glob 模式（大小写不敏感）|支持|支持|是|
|`-inum inode` 检查文件 inode 号|支持|支持|是|
|`-ipath pattern` 检查文件路径（大小写不敏感）|支持|支持|是|
|`-iregex pattern` 检查文件名是否满足正则表达式（大小写不敏感）|支持|支持|是|
|`-iwholename` 同 `-ipath`|支持|支持|是|
|`-links n` 检查文件硬链接数|支持|支持|是|
|`-lname pattern` 检查文件是否为符号链接并指向 `name` glob 模式的文件|支持|支持|是|
|`-mmin n` 检查文件修改时间（以分钟为单位）|支持|支持|是|
|`-mtime n` 检查文件修改时间（以天为单位）|支持|支持|是|
|`-name pattern` 检查文件名是否满足 `name` glob 模式|支持|支持|是|
|`-newer reference` 检查文件的修改时间是否比 `reference` 的修改时间新|支持|支持|是|
|`-nogroup` 检查文件是否属于某个组|支持|支持|是|
|`-nouser` 检查文件是否属于某个用户|支持|支持|是|
|`-path pattern` 检查文件路径是否满足 `path` glob 模式|支持|支持|是|
|`-perm mode` 检查文件权限位是否完全等于 `mode`|支持|支持|是|
|`-perm /mode` 检查文件权限位是否设置了 `mode` 的某一位|支持|支持|是|
|`-perm -mode` 检查文件权限位是否设置了 `mode` 的所有位|支持|支持|是|
|`-readable` 检查文件是否可读|支持|支持|是|
|`-regex pattern` 检查文件名是否满足正则表达式|支持|支持|是|
|`-samefile name` 检查文件是否与 `name` 指向同一个 inode|支持|支持|是|
|`-size n` 检查文件大小|支持|支持|是|
|`-true` 直接返回真|支持|支持|是|
|`-type [bcdpfls]` 检查文件类型|支持|支持|是|
|`-uid id` 检查文件所有者 ID|支持|支持|是|
|`-used n` 检查文件上次访问时间与上次修改时间|支持|支持|是|
|`-user u` 检查文件所有者（支持用户名与 UID ）|支持|支持|是|
|`-wholename` 同 `-path`|支持|支持|是|
|`-writable` 检查文件是否可写|支持|支持|是|
|`-xtype [bcdpfls]` 检查文件是否为某个类型，默认跟随符号链接|支持|支持|是|
|`newerXY ref` 检查文件的 `X` 时间戳是否比 `ref` 的 `Y` 时间戳新，`X` 支持 `a` 访问时间、 `c` 状态修改时间、 `m` 修改时间，`Y` 额外支持 `t` 支持 `date` 格式时间戳|支持|支持|是*|

注：

1. Gnu Findutils [实现的 `-daystart` 选项存在 Bug](https://savannah.gnu.org/bugs/index.php?printer=1&func=detailitem&item_id=23065) ，对计时起点的设置与文档中描述不同。

2. `-help` 选项输出的帮助信息带有版权信息与本地化输出，无法直接进行对比。

3. `-version` 选项输出的版本信息中带有版权信息与不同的版本号，无法直接进行对比。

4. 需要使用 `sleep` 命令进行休眠，以创建出创建时间、修改时间、元数据修改时间均不同的文件。由于目前 `test_killall` 中存在直接杀死 `sleep` 进程的测试，需要使用其他命令进行绕过。目前采用轮询时间戳等待的方式：

    ```bash
    # Tests in test_killall may kill sleep in this script.
    # So let we use new _snap function to take a break.
    _snap () {
        start=$(date +%s)
        while [ $(($(date +%s) - start)) -lt 2 ]; do
            :
        done
    }
    ```

### 软件自带用例对比验证

将来自 Gnu Findutils 的测试用例脚本移植到了 easybox 框架下，位于 `tests/fixtures/find/gnu` 目录。另重写了用于初始化测试用例脚本环境的脚本 `tests/fixtrues/find/init.sh` 。测试用例清单如下：

|测试名称|测试功能|测试情况|
|---|---|---|
|arg-nan.sh|对需要数的表达式传递非数值参数，程序应错误退出|通过|
|depth_unreadable_dir.sh|启用 `-depth` 后，仍然可以读取不可进入的目录名称|通过|
|exec-plus-last-file.sh|测试 `exec ... +` 族表达式是否能够追加文件名|通过|
|execdir-fd-leak.sh|测试 `execdir` 族表达式是否泄漏执行目录|通过|
|files0-from.sh|测试读取搜索起点|通过|
|inode-zero.sh|测试 `-inum` 表达式正确处理 0|通过|
|name-lbracket-literal.sh|测试能否正确处理 `[` 字面量|通过|
|name-slash.sh|当 `-name` 族表达式中出现路径分割 `/` 时，永远返回假|通过|
|newer.sh|检查 `-newer` 族表达式正常工作|通过*|
|printf_escape_c.sh|检查 `-printf` 表达式可以正确处理 `\\c` 转义，清空缓冲区|通过|
|printf_escape_chars.sh|检查 `-printf` 表达式可以正确处理其他转义|通过|
|printf_inode.sh|测试 `-printf` 表达式可以正确处理输出 inode 号|通过|
|refuse-noop.sh|拒绝非法表达式|通过|
|type_list.sh|测试文件类型过滤|通过|
|used.sh|测试 `-used` 表达式|通过|

注：

1. 在不修改原脚本中的 `sleep` 方式的情况下，该测试的时间延迟会被 `test_killall` 中的各类测试发送 SIGINT 信号导致无法成功 `sleep` ，造成失败的初始化。

如下为使用 easybox 框架进行测试的结果：

![测试结果](https://foruda.gitee.com/images/1728997009571624705/f1dd5168_9377817.png)

### 功能对比验证

1. 功能点: `-H` `-L` `-P` `-follow` 处理符号链接
    - 测试方法: 分别创建文件，目录以及指向它们的符号链接，然后创建一个悬垂符号链接，使用循环分别向 `find` 命令中假如参数，运行文件类型筛选过滤。
        ```bash
        for param in -H -L -P -follow ; do
            echo "# File" > out
            find $param . -type f >> out
            echo "# Dir" >> out
            find $param . -type d >> out
            echo "# Link" >> out
            find $param . -type l >> out
            compare exp out
        done
        ```
    - 测试结果: 使用 `-P` `-H` 选项时，不跟随符号链接，输出对应文件类型的文件；使用 `-L` `-follow` 时，输出符号链接指向的文件类型，且在符号链接损坏时认为是符号链接。

1. 功能点: `n \| +n \| -n` 检查数值等于 / 大于 / 小于 `n`
    - 测试方法: 分别创建大小为 0 ，1k ，2k 的文件，运行 `find -size 1k | +1k | -1k`
    - 测试结果: `1k` 参数输出恰好为 1k 大小的文件；`-1k` 输出大小为 0 的文件；`+1k` 输出大小为 2k 的文件。

1. 功能点: `exprs1 -a \| -and exprs2` 逻辑与
    - 测试方法: 将列表 `[-true, -false]` 与列表 `[-true, -false]` 进行 `flatmap` 后，带入命令 `find $1 -a $2` 运行
    - 测试结果: 当且仅当两个参数均为 `true` 时，find 会输出文件名。

1. 功能点: `exprs1 -o \| -or exprs2` 逻辑或
    - 测试方法: 将列表 `[-true, -false]` 与列表 `[-true, -false]` 进行 `flatmap` 后，带入命令 `find $1 -o $2` 运行
    - 测试结果: 至少一个参数为 `true` 时，find 会输出文件名。

1. 功能点: `! \| -not exprs` 逻辑非
    - 测试方法: 使用循环分别将 `-true` `-false` 带入 `find ! $1` 运行。
    - 测试结果: 只在参数为 `-false` 时输出文件名。

1. 功能点: `( exprs )` 强制优先级
    - 测试方法: `find . -name afile -o -name bfile -print` `find . \( -name afile -o -name bfile \) -print`
    - 测试结果: 第一条指令永远不会输出 `afile`，而第二条指令可以。与 find 执行结果相同

1. 功能点: `expr1 , expr2` 依次执行表达式，返回最后的表达式值
    - 测试方法: `find false , true`
    - 测试结果: 与 find 执行结果相同，总是会输出文件名。

1. 功能点: `-d \| -depth` 优先处理目录内容而不是目录本身
    - 测试方法: 在任一多级目录中运行 `find -d`
    - 测试结果: 与 find 执行结果相同，文件输出顺序相同。

1. 功能点: `-files0-from name` 从文件读取搜索起点
    - 测试方法: `find -files0-from files`。运行 `tests/fixtures/gnu/files0-from.sh`。
    - 测试结果: 分别测试了 `-files0-from` 与 `-ok` 选项结合，输入文件不存在，输入文件为空以及正确输入的情况。与原实现结果相同。

1. 功能点: `-help \| --help` 输出帮助信息并退出
    - 测试方法: `find -help`
    - 测试结果: 正常输出帮助信息。

1. 功能点: `-maxdepth n` 设置最大搜索层数
    - 测试方法: `find -maxdepth 0`
    - 测试结果: 只输出搜索起点目录。

1. 功能点: `-mindepth n` 设置最小搜索层数
    - 测试方法: `find -mindepth 9999`
    - 测试结果: 不输出任何目录。

1. 功能点: `-mount` `-xdev` 不搜索不同设备的路径
    - 测试方法: 首先在搜索目录下创建内存文件系统并挂载 `mkdir tmp && mount -t tmpfs -o size=10M tmpfs tmp` ，然后运行 `find -xdev`
    - 测试结果: 不输出 `tmp` 目录。

1. 功能点: `-version \| --version` 输出版本信息并退出
    - 测试方法: `find -version`
    - 测试结果: 正常输出版本信息。

1. 功能点: `-delete` 删除文件
    - 测试方法: 分别创建两个测试环境并生成各自的文件，然后分别使用原生 `find` 与 Rust 版本 `find` 运行 `find -name file -delete` ，随后检查目录内容。
    - 测试结果: 两测试环境中目录内容相同。

1. 功能点: `-exec` 执行命令
    - 测试方法: `find -exec ls -dils '{}' ';'`
    - 测试结果: 与 find 执行结果相同。

1. 功能点: `-execdir` 切换目录并执行命令
    - 测试方法: `find -execdir ls -dils '{}' ';'`
    - 测试结果: 与 find 执行结果相同。

1. 功能点: `-fls file` 使用 `ls -dils` 格式输出元数据到文件
    - 测试方法: `find -fls file`
    - 测试结果: 与 find 执行结果相同。

1. 功能点: `-fprint file` 输出文件路径到文件并换行
    - 测试方法: `find -fprint filef`
    - 测试结果: 与 find 执行结果相同

1. 功能点: `-fprint0 file` 输出文件路径到文件，以 `\0` 分隔
    - 测试方法: `find -frpint0 file`
    - 测试结果: 与 find 执行结果相同

1. 功能点: `-fprintf file format` 输出格式化字符串到文件
    - 测试方法: 将特殊转义字符与格式控制字符连接成格式字符串，然后运行`find -fprintf file "$1\n"`
    - 测试结果: 与 find 执行结果相同。正确输出转义字符与格式控制字符对应的文件元数据。

1. 功能点: `-ls` 使用 `ls -dils` 格式输出元数据
    - 测试方法: `find -ls`
    - 测试结果: 输出 `ls -dils` 格式的文件元数据。

1. 功能点: `-ok` `-okdir` 在用户确认后执行命令
    - 测试方法: `find -ok ls -dils '{}' ';'`
    - 测试结果: 提示用户输入 `[y/n]` ，输入 `y` 则输出 `.` 否则不输出。

1. 功能点: `-prune` 剪去当前路径搜索枝
    - 测试方法: 创建 `dir` 目录，分别在当前目录与 `dir` 目录下创建 `file` 文件，然后运行 `find \( -name dir -prune \) -name file` 。
    - 测试结果: 只输出 `file` 而不输出 `dir/file` 。

1. 功能点: `-quit` 立即退出程序
    - 测试方法: 创建 `dir` 目录，分别在当前目录与 `dir` 目录下创建 `file` 文件，然后运行 `find \( -name dir -quit \) -name file` 。
    - 测试结果: 只输出 `file` 而不输出 `dir/file` 。

1. 功能点: `-amin n` 检查文件访问时间（以分钟为单位）
    - 测试方法: 使用 `touch` 创建文件后，再使用 `touch -a -d "3 mintues" file`，`find -amin 3`
    - 测试结果: 与 find 执行结果相同

1. 功能点: `-atime n` 检查文件访问时间（以天为单位）
    - 测试方法: 使用 `touch` 创建文件后，再使用 `touch -a -d "3 days" file`，`find -amin 3`
    - 测试结果: 与 find 执行结果相同

1. 功能点: `-cmin n` 检查文件状态修改时间（以分钟为单位）
    - 测试方法: 使用 `touch` 创建文件后，等待一分钟，`find -cmin 1`
    - 测试结果: 与 find 执行结果相同

1. 功能点: `-ctime n` 检查文件状态修改时间（以天为单位）
    - 测试方法: 使用 `touch` 创建文件后，再使用 `touch -c -d "3 days" file`，`find -ctime 3`
    - 测试结果: 与 find 执行结果相同

1. 功能点: `-mmin n` 检查文件修改时间（以分钟为单位）
    - 测试方法: 使用 `touch` 创建文件后，再使用 `touch -m -d "3 mintues" file`，`find -mmin 3`
    - 测试结果: 与 find 执行结果相同

1. 功能点: `-mtime n` 检查文件修改时间（以天为单位）
    - 测试方法: 使用 `touch` 创建文件后，再使用 `touch -m -d "3 days" file`，`find -mtime 3`
    - 测试结果: 与 find 执行结果相同

1. 功能点: `-used n` 检查文件上次访问时间与上次修改时间
    - 测试方法: `find -used 1`
    - 测试结果: 与 find 执行结果相同

1. 功能点: `newerXY ref` 检查文件的 `X` 时间戳是否比 `ref` 的 `Y` 时间戳新，`X` 支持 `a` 访问时间、 `c` 状态修改时间、 `m` 修改时间，`Y` 额外支持 `t` 支持 `date` 格式时间戳
    - 测试方法: 创建 3 个创建时间间隔大于 1 秒的文件 `file1` 、`file2` 、`file3` ，利用循环，将一下参数代入到命令中
        - anewer
        - cnewer
        - newer
        - neweraa, newerac, neweram
        - newerca, newercc, newercm
        - newerma, newermc, newermm
        然后运行 `find -$param file2` ，每一次运行的输出都应该为 file3 。
    - 测试结果: 与预期输出相同。

1. 功能点: `-anewer reference` 检查文件访问时间是否比 `reference` 新
    - 测试方法: 同上。
    - 测试结果: 与 find 执行结果相同

1. 功能点: `-cnewer reference` 检查文件状态修改时间是否比 `reference` 新
    - 测试方法: 同上。
    - 测试结果: 与 find 执行结果相同

1. 功能点: `-newer reference` 检查文件的修改时间是否比 `reference` 的修改时间新
    - 测试方法: `find -newer file`
    - 测试结果: 与 find 执行结果相同

1. 功能点: `-empty` 检查文件是否为空
    - 测试方法: 分别创建一个空文件与一个 512 字节的文件，运行 `find -empty`。
    - 测试结果: 仅输出空文件名。

1. 功能点: `-executable` 检查文件是否可执行
    - 测试方法: 使用 `touch` 创建两个文件，对其中一个文件执行 `chmod u+x file` ，`find -executable`
    - 测试结果: 仅输出有可执行权限的文件。

1. 功能点: `-false` 直接返回假
    - 测试方法: `find -false`
    - 测试结果: 不输出任何文件名。

1. 功能点: `-fstype fs` 检查文件路径所在的设备 / 文件系统
    - 测试方法: `find -fstype ext4`
    - 测试结果: 与 find 执行结果相同

1. 功能点: `-gid id` 检查文件组 ID
    - 测试方法: `find -gid 1`
    - 测试结果: 仅输出当前目录下由 root 组所有的文件。

1. 功能点: `-group name\|id` 检查文件是否由`group` 组所有（支持组名与 GID）
    - 测试方法: `find -group root`
    - 测试结果: 仅输出当前目录下由 root 组所有的文件。

1. 功能点: `-ilname pattern` 检查文件是否为符号链接并指向 `name` glob 模式的文件（大小写不敏感）
    - 测试方法: 创建一个文件 `Pattern` 并创建到它的符号链接 `ln -s pattern link` ，`find -ilname pattern`
    - 测试结果: 输出 `link` 。

1. 功能点: `-iname pattern` 检查文件名是否满足 `name` glob 模式（大小写不敏感）
    - 测试方法: 创建 `Pattern` 文件 ，`find -iname pattern`
    - 测试结果: 输出 `Pattern` 。

1. 功能点: `-inum inode` 检查文件 inode 号
    - 测试方法: 创建一个新文件 `file` 并使用 `stat -c %i file` 获取对应的 inode 号码，然后`find -inum inode`
    - 测试结果: 输出 `file` 。

1. 功能点: `-ipath pattern` 检查文件路径（大小写不敏感）
    - 测试方法: 创建一个新文件 `Pattern` ，`find -ipath ./pattern`
    - 测试结果: 输出 `./Pattern` 。

1. 功能点: `-iregex pattern` 检查文件名是否满足正则表达式（大小写不敏感）
    - 测试方法: 创建一个文件 `Pattern` ，`find -iregex p.*ern`
    - 测试结果: 输出 `Pattern` 。

1. 功能点: `-links n` 检查文件硬链接数
    - 测试方法: 创建一个新文件 `file` ，同时创建它的硬链接 `ln file link` ，`find -links 2`
    - 测试结果: 同时输出 `file` 与 `link` 。

1. 功能点: `-lname pattern` 检查文件是否为符号链接并指向 `name` glob 模式的文件
    - 测试方法: 创建新文件 `pattern` `Pattern` 及其符号链接 `link` `Link` ，`find -lname pattern`
    - 测试结果: 仅输出 `link`

1. 功能点: `-name pattern` 检查文件名是否满足 `name` glob 模式
    - 测试方法: 创建新文件 `pattern` `Pattern` ，`find -lname pattern`
    - 测试结果: 仅输出 `pattern`

1. 功能点: `-nogroup` 检查文件是否属于某个组
    - 测试方法: 创建新文件 `file` 并运行 `sudo chmod :65534 file` ,`find -nogroup` 。
    - 测试结果: 输出 `file` 。

1. 功能点: `-nouser` 检查文件是否属于某个用户
    - 测试方法: 创建新文件 `file` 并运行 `sudo chmod 65534 file` ,`find -nouser` 。
    - 测试结果: 输出 `file` 。

1. 功能点: `-path pattern` 检查文件路径是否满足 `path` glob 模式
    - 测试方法: 创建新文件 `pattern` `Pattern` ，`find -path ./pattern`
    - 测试结果: 仅输出 `pattern`

1. 功能点: `-perm mode` 检查文件权限位是否完全等于 `mode`
    - 测试方法: 分别创建权限为 664 、764 、777 的文件， `find -perm 644` 。
    - 测试结果: 只输出权限为 664 的文件。

1. 功能点: `-perm /mode` 检查文件权限位是否设置了 `mode` 的某一位
    - 测试方法: 分别创建权限为 664 、764 、777 、000 的文件， `find -perm 764` 。
    - 测试结果: 输出权限为 664 、764 、777 的文件。

1. 功能点: `-perm -mode` 检查文件权限位是否设置了 `mode` 的所有位
    - 测试方法: 分别创建权限为 664 、764 、777 的文件， `find -perm -744` 。
    - 测试结果: 与 find 执行结果相同

1. 功能点: `-readable` 检查文件是否可读
    - 测试方法: 创建一个文件 `file` 并 `chmod -r file` ，`find -readable`
    - 测试结果: 不输出 `file` 。

1. 功能点: `-regex pattern` 检查文件名是否满足正则表达式
    - 测试方法: 创建一个文件 `pattern` ，`find -iregex p.*ern`
    - 测试结果: 输出 `pattern` 。

1. 功能点: `-samefile name` 检查文件是否与 `name` 指向同一个 inode
    - 测试方法: 创建一个新文件 `file` ，同时创建它的硬链接 `ln file link` ，`find -name link -samefile file` 。
    - 测试结果: 输出 `link` 。

1. 功能点: `-size n` 检查文件大小
    - 测试方法: 分别创建大小为 0 ，1k ，2k 的文件，运行 `find -size 1k | +1k | -1k`
    - 测试结果: `1k` 参数输出恰好为 1k 大小的文件；`-1k` 输出大小为 0 的文件；`+1k` 输出大小为 2k 的文件。

1. 功能点: `-true` 直接返回真
    - 测试方法: `find -true` 。
    - 测试结果: 输出当前目录所有文件。

1. 功能点: `-type [bcdpfls]` 检查文件类型
    - 测试方法: 分别创建文件，目录以及指向它们的符号链接，然后创建一个悬垂符号链接，使用循环分别向 `find` 命令中假如参数，运行文件类型筛选过滤。
        ```bash
        for param in -H -L -P -follow ; do
            echo "# File" > out
            find $param . -type f >> out
            echo "# Dir" >> out
            find $param . -type d >> out
            echo "# Link" >> out
            find $param . -type l >> out
            compare exp out
        done
        ```
    - 测试结果: 使用 `-P` `-H` 选项时，不跟随符号链接，输出对应文件类型的文件；使用 `-L` `-follow` 时，输出符号链接指向的文件类型，且在符号链接损坏时认为是符号链接。

1. 功能点: `-uid id` 检查文件所有者 ID
    - 测试方法: `find -uid 1`
    - 测试结果: 只输出由 root 用户所有的文件。

1. 功能点: `-user u` 检查文件所有者（支持用户名与 UID ）
    - 测试方法: `find -user root`
    - 测试结果: 只输出由 root 用户所有的文件。

1. 功能点: `-writable` 检查文件是否可写
    - 测试方法: 创建一个文件 `file` 并 `chmod -w file` ，`find -writable`
    - 测试结果: 不输出 `file` 。

1. 功能点： 与原实现进行性能对比
    - 测试方法：在当前用户 home 目录下，搜索 Rust 源文件（以 glob 模式搜索 `*.rs` ）。
    - 测试结果：每个命令运行 1000 次的运行时间统计如下图。

        ![运行时间统计图](https://foruda.gitee.com/images/1728997072817336036/c0f44211_9377817.png)

        在该测试情景下，重写后的实现性能不弱于原实现。

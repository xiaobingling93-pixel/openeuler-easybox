### 软件支持的功能清单

| 软件功能                                 | 原有软件是否支持 | Rust重写后的软件是否支持 | 是否自动化测试 |
| ---------------------------------------- | ---------------- | ------------------------ | -------------- |
| Mount指定设备                            | 是               | 是                       | 是             |
| 根据fstab挂载所有文件系统                | 是               | 是                       | 是             |
| 将子树挂载到其他地方(等同于-o bind)      | 是               | 是                       | 是             |
| 不对路径进行规范化                       | 是               | 是                       | 是             |
| 模拟挂载操作而不实际执行mount(2)系统调用 | 是               | 是                       | 是             |
| 为每个设备创建子进程(与-a选项一起使用)   | 是               | 是                       | 是             |
| 显示帮助信息                             | 是               | 是                       | 不需要         |
| 不调用mount.<type>辅助程序               | 是               | 是                       | 是             |
| 同时显示文件系统标签                     | 是               | 是                       | 是             |
| 根据LABEL=<label>挂载                    | 是               | 是                       | 是             |
| 移动子树到其他地方                       | 是               | 是                       | 是             |
| 将子树标记为private                      | 是               | 是                       | 是             |
| 递归地将整个子树标记为private            | 是               | 是                       | 是             |
| 递归地将整个子树标记为shared             | 是               | 是                       | 是             |
| 递归地将整个子树标记为slave              | 是               | 是                       | 是             |
| 递归地将整个子树标记为unbindable         | 是               | 是                       | 是             |
| 将子树标记为shared                       | 是               | 是                       | 是             |
| 将子树标记为slave                        | 是               | 是                       | 是             |
| 将子树标记为unbindable                   | 是               | 是                       | 是             |
| 不写入/etc/mtab                          | 是               | 是                       | 是             |
| 在另一个命名空间执行挂载                 | 是               | 是                       | 是             |
| 指定挂载选项                             | 是               | 是                       | 是             |
| 限制文件系统类型集合(与-a选项一起使用)   | 是               | 是                       | 是             |
| 指定如何处理从fstab加载的选项            | 是               | 是                       | 是             |
| 指定挂载选项来源                         | 是               | 是                       | 是             |
| 以只读方式挂载文件系统(等同于-o ro)      | 是               | 是                       | 是             |
| 将子树及所有子挂载点挂载到其他地方       | 是               | 是                       | 是             |
| 指定源(路径、标签、uuid)                 | 是               | 是                       | 是             |
| 限制文件系统类型集合                     | 是               | 是                       | 是             |
| 指定/etc/fstab的替代文件                 | 是               | 是                       | 是             |
| 指定挂载点                               | 是               | 是                       | 是             |
| 为所有挂载点指定路径前缀                 | 是               | 是                       | 是             |
| 根据UUID=<uuid>挂载                      | 是               | 是                       | 是             |
| 打印当前操作                             | 是               | 是                       | 是             |
| 显示版本号                               | 是               | 是                       | 不需要         |
| 以读写方式挂载文件系统(默认)             | 是               | 是                       | 是             |

### 软件自带用例对比验证

| 测试用例 | 测试目的 | 已集成至测试代码 | 备注 |
| --- | --- | :---: | --- |
| mount | 无参数输出 | √ | 输出顺序与C实现不同(seclabel选项位置) |
| mount -t ext4 | -t参数输出指定文件系统类型 | √ | 输出顺序与C实现不同(seclabel选项位置) |
| mount -l | -l参数显示文件系统标签 | √ | 输出顺序与C实现不同(seclabel选项位置) |
| mount -v /dev/loop0 mount_point | -v参数显示详细信息 | √ | |
| mount -r /dev/loop0 mount_point | -r参数只读挂载 | √ | |
| mount -w /dev/loop0 mount_point | -w参数读写挂载 | √ | |
| mount -a | -a参数挂载fstab中所有文件系统 | √ | |
| mount -t ext4 /dev/loop0 mount_point | -t参数指定文件系统类型 | √ | |
| mount -o ro,noexec /dev/loop0 mount_point | -o参数设置挂载选项 | √ | |
| mount --bind source mount_point | --bind参数绑定挂载 | √ | |
| mount -R source mount_point | -R参数递归绑定挂载 | √ | |
| mount --move mount_point new_target | --move参数移动挂载点 | √ | |
| mount -L easyblock mount_point | -L参数通过标签挂载 | √ | |
| mount -U uuid mount_point | -U参数通过UUID挂载 | √ | |
| mount --make-private mount_point | --make-private参数设置私有挂载 | √ | |
| mount --make-shared mount_point | --make-shared参数设置共享挂载 | √ | |
| mount --no-mtab /dev/loop0 mount_point | --no-mtab参数不更新mtab | √ | |
| mount --no-canonicalize /dev/loop0 mount_point | --no-canonicalize参数不规范化路径 | √ | |
| mount -f /dev/loop0 mount_point | -f参数模拟挂载 | √ | |
| mount -a -F | -F参数创建新进程进行挂载 | √ | |
| mount --internal-only /dev/loop0 mount_point | --internal-only参数仅使用内部挂载 | √ | |
| mount -N pid /dev/loop0 mount_point | -N参数在指定命名空间中执行挂载 | √ | 需要特殊容器能力 |
| mount -T new_fstab /dev/loop0 | -T参数使用替代的fstab文件 | √ | |

![输入图片说明](https://foruda.gitee.com/images/1730333962921426661/2b88bb15_11580640.png "屏幕截图")

#### 功能对比验证

1. 功能点：mount无参数输出(`test_mount_print_all`)
   - 测试方法：分别调用`easybox mount`命令和系统`mount`命令,对比两者是否出现差异
   - 测试结果：输出无差异，测试通过
2. 功能点：测试-t 参数输出指定文件系统类型的挂载点信息(`test_mount_print_all_only_types`)
   - 测试方法：分别调用`easybox mount -t ext4`命令和系统`mount -t ext4`,对比两者输出结果是否有差异
   - 测试结果：输出无差异，测试通过
3. 功能点：测试-l 参数，显示文件系统标签(`test_mount_show_labels`)
   - 测试方法：分别调用`easybox mount -l`命令和系统`mount -l`,对比两者输出结果是否有差异
   - 测试结果，输出无差异，测试通过
4. 功能点：测试-v 参数，显示详细信息(`test_mount_verbose`)
   - 测试方法：分别调用`easybox mount -v /dev/loop0 mount_point`命令和系统`mount -v /dev/loop0 mount_point`,对比两者输出结果是否有差异
   - 测试结果：输出无差异，测试通过
5. 功能点：测试-r参数，以只读方式挂载(`test_mount_read_only`)
   - 测试方法：分别调用`easybox mount -r /dev/loop0 mount_point` 和`mount-r /dev/loop0 mount_point` 命令，分别在挂载命令之后调用`mount`命令，对比输出内容是否有差异
   - 测试结果：两次调用`mount`命令输出无差异，测试通过
6. 功能点：测试-w 参数，以读写方式挂载(`test_mount_read_write`)
   - 测试方法：分别调用`easybox mount -w /dev/loop0 mount_point` 和`mount -w /dev/loop0 mount_point` 命令，在两次挂载命令之后调用`mount`命令，对比输出内容是否有差异
   - 测试结果：两次调用`mount`命令输出无差异，测试通过
7. 功能点：测试-a 参数，挂载 fstab 中的所有文件系统(`test_mount_all`)
   - 测试方法：分别调用`easybox mount a`命令和系统`mount -a`,分别在挂载命令之后调用`mount`命令，对比输出内容是否有差异
   - 测试结果：两次调用`mount`命令输出无差异，测试通过
8. 功能点：测试-t 参数,指定文件系统类型(test_mount_types)
   - 测试方法：分别调用`easybox mount -t ext4 /dev/loop0 mount_point`命令和系统`mount -t ext4 /dev/loop0 mount_point`，分别在挂载命令之后调用`mount`命令，对比输出内容是否有差异
   - 测试结果：两次调用`mount`命令输出无差异，测试通过
9. 功能点：测试-o 参数,设置挂载选项(`test_mount_options`)
   - 测试方法：分别调用`easybox mount -o ro,noexec /dev/loop0 mount_point `命令和系统`mount -o ro,noexec /dev/loop0 mount_point `,分别在挂载命令之后调用`mount`命令，对比输出内容是否有差异
   - 测试结果：两次调用`mount`命令输出无差异，测试通过
10. 功能点：测试 `--bind` 参数，绑定挂载(`test_mount_bind`)
    - 测试方法：分别调用`easybox mount --bind source mount_point `命令和系统`mount --bind source mount_point `,分别在挂载命令之后调用`mount`命令，对比输出内容是否有差异
    - 测试结果：两次调用`mount`命令输出无差异，测试通过
11. 功能点：测试 `-R` 参数，递归绑定挂载(`test_mount_rbind`)
    - 测试方法：分别调用`easybox mount -R source mount_point `命令和系统`mount -R source mount_point `,分别在挂载命令之后调用`mount`命令，对比输出内容是否有差异
    - 测试结果：两次调用`mount`命令输出无差异，测试通过
12. 功能点：测试 `--move` 参数，移动挂载点(`test_mount_move`)
    - 测试方法：分别调用`easybox mount --move mount_point new_target `命令和系统`mount --move mount_point new_target `,分别在挂载命令之后调用`mount`命令，对比输出内容是否有差异
    - 测试结果：两次调用`mount`命令输出无差异，测试通过
13. 功能点：测试 `-L` 参数，通过标签挂载(`test_mount_label`)
    - 测试方法：分别调用`easybox mount -L easyblock mount_point`命令和系统`mount -L easyblock mount_point `,(通过文件系统标签 "easyblock" 挂载)。分别在挂载命令之后调用`mount`命令，对比输出内容是否有差异
    - 测试结果：两次调用`mount`命令输出无差异，测试通过
14. 功能点：测试 `-U` 参数，通过 UUID 挂载(`test_mount_uuid`)
    - 测试方法：分别调用`easybox mount -U b191c10c-448d-4e80-a8d6-e5303260cf5f mount_point`命令和系统`mount -U b191c10c-448d-4e80-a8d6-e5303260cf5f mount_point `,(通过 UUID 挂载)。分别在挂载命令之后调用`mount`命令，对比输出内容是否有差异
    - 测试结果：两次调用`mount`命令输出无差异，测试通过
15. 功能点：测试 `--make-private` 参数，设置私有挂载(`test_mount_make_private`)
    - 测试方法：分别调用`easybox mount --make-private mount_point `命令和系统`mount --make-private mount_point `，分别在挂载命令之后调用`mount`命令，对比输出内容是否有差异
    - 测试结果：两次调用`mount`命令输出无差异，测试通过

16. 功能点：`--make-shared` 参数，设置共享挂载(`test_mount_make_shared`)
    - 测试方法：分别调用`easybox mount --make-shared mount_point `命令和系统`mount --make-shared mount_point `，分别在挂载命令之后调用`mount`命令，对比输出内容是否有差异
    - 测试结果：两次调用`mount`命令输出无差异，测试通过

17. 功能点：测试 `--no-mtab` 参数，不更新 mtab 文件(`test_mount_no_mtab`)
    - 测试方法：分别调用`easybox mount --no-mtab /dev/loop0 mount_point `命令和系统`mount --no-mtab /dev/loop0 mount_point `,然后对比`/etc/mtab`文件内容一致
    - 测试结果：`/etc/mtab`文件内容一致，皆未更新，测试通过

18. 功能点：测试 `--no-canonicalize` 参数，不规范化路径(`test_mount_no_canonicalize`)
    - 测试方法：分别调用`easybox mount --no-canonicalize /dev/loop0 mount_point` 和`mount --no-canonicalize /dev/loop0 mount_point` 命令，对比输出是否一致
    - 测试结果：输出一致，测试通过

19. 功能点：测试 `-f` 参数，模拟挂载(`test_mount_fake`)
    - 测试方法：分别调用`easybox mount -f /dev/loop0 mount_point` 和`mount -f /dev/loop0 mount_point` 命令，分别在挂载命令之后调用`mount`命令，对比输出内容是否有差异
    - 测试结果：两次调用`mount`命令输出无差异，测试通过

20. 功能点：测试 `-F` 参数，创建新进程进行挂载(`test_mount_fork`)
    - 测试方法：分别调用`easybox mount -a -F` 和`mount -a -F` 命令，分别在挂载命令之后调用`mount`命令，对比输出内容是否有差异
    - 测试结果：两次调用`mount`命令输出无差异，测试通过

21. 功能点：测试 `--internal-only` 参数，仅使用内部挂载(`test_mount_internal_only`)
    - 测试方法：分别调用`easybox mount --internal-only /dev/loop0 mount_point` 和`mount --internal-only /dev/loop0 mount_point` 命令，分别在挂载命令之后调用`mount`命令，对比输出内容是否有差异
    - 测试结果：两次调用`mount`命令输出无差异，测试通过

22. 功能点：测试 `-N` 参数，在指定命名空间中执行挂载(`test_mount_namespace`)
    - 测试方法：分别调用`easybox mount -N <pid> /dev/loop0 mount_point` 和`mount -N <pid> /dev/loop0 mount_point` 命令，分别在挂载命令之后调用`mount`命令，对比输出内容是否有差异
    - 测试结果：两次调用`mount`命令输出无差异，测试通过

23. 功能点：测试 `-T` 参数，使用替代的 fstab 文件(`test_mount_fstab_alternative`)
    - 测试方法：分别调用`easybox mount -T new_fstab /dev/loop0` 和`mount -T new_fstab /dev/loop0` 命令，分别在挂载命令之后调用`mount`命令，对比输出内容是否有差异
    - 测试结果：两次调用`mount`命令输出无差异，测试通过

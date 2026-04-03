### 软件支持的功能清单


| 软件功能                                 | 原有软件是否支持 | Rust重写后的软件是否支持 | 是否自动化测试 |
| ---------------------------------------- | ---------------- | ------------------------ | -------------- |
| 卸载指定目标                             | 是               | 是                       | 是             |
| 卸载所有文件系统                         | 是               | 是                       | 是             |
| 卸载指定设备在当前命名空间中的所有挂载点 | 是               | 是                       | 是             |
| 不对路径进行规范化                       | 是               | 是                       | 是             |
| 如果是挂载的loop设备,同时释放loop设备    | 是               | 是                       | 是             |
| 强制卸载(适用于无法访问的NFS系统)        | 是               | 是                       | 是             |
| 模拟执行但跳过umount(2)系统调用          | 是               | 是                       | 是             |
| 显示帮助信息                             | 是               | 是                       | 不需要         |
| 不调用umount.<type>辅助程序              | 是               | 是                       | 是             |
| 现在卸离文件系统,稍后清理                | 是               | 是                       | 是             |
| 不写入/etc/mtab                          | 是               | 是                       | 是             |
| 在另一个命名空间中执行卸载               | 是               | 是                       | 是             |
| 限制文件系统集合(与-a一起使用)           | 是               | 是                       | 是             |
| 抑制"未挂载"的错误信息                   | 是               | 是                       | 是             |
| 如果卸载失败,尝试重新挂载为只读          | 是               | 是                       | 是             |
| 递归地卸载目标及其所有子项               | 是               | 是                       | 是             |
| 限制文件系统类型集合                     | 是               | 是                       | 是             |
| 打印当前操作                             | 是               | 是                       | 是             |
| 显示版本号                               | 是               | 是                       | 不需要         |

![输入图片说明](https://foruda.gitee.com/images/1730334013003746891/3061864f_11580640.png "屏幕截图")

#### 功能对比验证

1. 功能点: 测试umount -a选项卸载所有文件系统(`test_umount_all`)
   * 测试方法: 分别调用easybox umount -a命令和系统umount -a命令,对比两者输出是否一致
   * 测试结果: 两个命令的标准输出和标准错误输出完全一致,测试通过

2. 功能点: 测试umount -c选项不对路径进行规范化(`test_umount_no_canonicalize`)
   * 测试方法: 分别调用带有-c选项的easybox umount命令和系统umount命令卸载指定文件系统,然后检查文件系统是否被成功卸载
   * 测试结果: Rust程序和C程序执行umount后,再次执行mount命令,输出结果无差异,文件系统均成功卸载,测试通过

3. 功能点: 测试umount -d选项卸载loop设备(`test_umount_detach_loop`)  
   * 测试方法: 先挂载loop设备到指定目录,分别使用带-d选项的easybox umount命令和系统umount命令进行卸载,然后检查文件系统和loop设备是否被成功卸载
   * 测试结果: Rust程序和C程序执行umount后,再次执行mount命令,输出结果无差异,文件系统和loop设备均成功卸载,测试通过

4. 功能点: 测试umount -f选项强制卸载文件系统(`test_umount_force`)
   * 测试方法: 分别调用带有-f选项的easybox umount命令和系统umount命令强制卸载指定文件系统,然后检查文件系统是否被成功卸载 
   * 测试结果: Rust程序和C程序执行umount后,再次执行mount命令,输出结果无差异,文件系统均成功卸载,测试通过

5. 功能点:测试umount --fake选项模拟卸载文件系统(`test_umount_fake`)
   * 测试方法:先挂载文件系统到指定目录,分别使用带--fake选项的easybox umount命令和系统umount命令执行模拟卸载,然后检查文件系统是否真的被卸载
   * 测试结果:Rust程序和C程序执行umount --fake后,再次执行mount命令,输出结果无差异,文件系统均未被真正卸载,符合预期,测试通过

6. 功能点:测试umount -i选项只卸载由mount内部挂载的文件系统(`test_umount_internal_only`)
   * 测试方法:先手动挂载文件系统,分别使用带-i选项的easybox umount命令和系统umount命令执行卸载,检查手动挂载的文件系统是否被排除
   * 测试结果:Rust程序和C程序使用-i选项执行umount后,手动挂载的文件系统没有被卸载,符合预期,测试通过

7. 功能点: 测试umount -l选项lazy卸载文件系统(`test_umount_lazy`)
   * 测试方法: 分别调用带有-l选项的easybox umount命令和系统umount命令lazy卸载指定文件系统,然后检查文件系统是否被成功卸载
   * 测试结果: Rust程序和C程序执行umount后,再次执行mount命令,输出结果无差异,文件系统均成功卸载,测试通过

8. 功能点:测试umount -n选项卸载时不更新/etc/mtab(`test_umount_no_mtab`)
   * 测试方法:分别使用带-n选项的easybox umount命令和系统umount命令卸载指定文件系统,然后检查/etc/mtab文件是否更新
   * 测试结果:Rust程序和C程序使用-n选项执行umount后,/etc/mtab文件均未更新,符合预期,测试通过

9. 功能点: 测试umount -N选项在指定的namespace中执行umount(`test_umount_namespace`)
   * 测试方法: 获取当前进程的pid作为namespace参数,分别调用带有-N选项的easybox umount命令和系统umount命令在指定namespace中卸载指定文件系统,然后检查文件系统是否被成功卸载
   * 测试结果: Rust程序和C程序执行umount后,再次执行mount命令,输出结果无差异,文件系统均成功卸载,测试通过

10. 功能点:测试umount -q选项静默执行umount(`test_umount_quiet`) 
    * 测试方法:分别调用带有-q选项的easybox umount命令和系统umount命令静默卸载指定文件系统,然后检查文件系统是否被成功卸载
    * 测试结果:Rust程序和C程序执行umount后,再次执行mount命令,输出结果无差异,文件系统均成功卸载,测试通过

11. 功能点:测试umount -r选项重新挂载为只读(`test_umount_read_only`)
    * 测试方法:先以读写模式挂载文件系统,分别使用带-r选项的easybox umount命令和系统umount命令重新挂载为只读模式,检查挂载状态
    * 测试结果:Rust程序和C程序使用-r选项执行umount后,文件系统均以只读方式重新挂载,符合预期,测试通过

12. 功能点:测试umount -R选项递归卸载文件系统(`test_umount_recursive`)
    * 测试方法:在指定目录下挂载多层文件系统,分别使用带-R选项的easybox umount命令和系统umount命令递归卸载,检查各级文件系统是否全部被卸载 
    * 测试结果:Rust程序和C程序使用-R选项执行umount后,各级子目录的文件系统全部被卸载,符合预期,测试通过

13. 功能点:测试umount -t选项卸载指定类型文件系统(`test_umount_types`)
    * 测试方法:挂载多个不同类型的文件系统,分别使用带-t选项的easybox umount命令和系统umount命令指定需要卸载的类型,检查对应类型的文件系统是否被卸载
    * 测试结果:Rust程序和C程序使用-t选项执行umount后,指定类型的文件系统均被卸载,其他类型的文件系统不受影响,符合预期,测试通过

14. 功能点:测试umount -v选项显示执行umount时的详细信息(`test_umount_verbose`)
    * 测试方法:分别调用带有-v选项的easybox umount命令和系统umount命令卸载指定文件系统,获取并比较命令的输出内容
    * 测试结果:Rust程序和C程序使用-v选项执行umount后,输出的详细过程信息一致,符合预期,测试通过

15. 功能点:测试umount -A选项卸载指定设备的所有挂载点(`test_umount_all_targets`)
    * 测试方法:将同一个loop设备挂载到两个不同目录,分别使用带-A选项的easybox umount命令和系统umount命令卸载该设备,检查两个挂载点是否都被卸载
    * 测试结果:Rust程序和C程序使用-A选项执行umount后,指定设备的所有挂载点全部被卸载,符合预期,测试通过
16. 功能点:测试umount -O选项测试文件系统选项(`test_umount_test_opts`)
    - 测试方法:先挂载文件系统,分别使用带-O选项的easybox umount命令和系统umount命令测试指定的文件系统选项,检查输出结果是否一致
    - 测试结果:Rust程序和C程序使用-O选项执行umount后,输出结果完全一致,符合预期,测试通过

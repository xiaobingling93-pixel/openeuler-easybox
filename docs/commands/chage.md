## 功能对比验证
### 软件支持的功能清单


| 软件功能                                                     | 原有软件 | Rust重写后的软件 |
| ------------------------------------------------------------ | -------- | ---------------- |
| chage [options] LOGIN                                        | 支持     | 支持             |
| **-d, --lastday LAST_DAY** 设置上次密码更改的日期为 LAST_DAY | 支持     | 支持             |
| **-E, --expiredate EXPIRE_DATE** 设置账户的到期日期为 EXPIRE_DATE | 支持     | 支持             |
| **-h, --help** 显示此帮助信息并退出                          | 支持     | 支持             |
| **-i, --iso8601** 在打印日期时使用 YYYY-MM-DD 格式           | 支持     | 支持             |
| **-I, --inactive INACTIVE** 设置密码到期后多少天后账户会被禁用为 INACTIVE | 支持     | 支持             |
| **-l, --list** 显示账户的期限信息                            | 支持     | 支持             |
| **-m, --mindays MIN_DAYS** 设置密码更改前的最小天数为 MIN_DAYS | 支持     | 支持             |
| **-M, --maxdays MAX_DAYS** 设置密码更改前的最大天数为 MAX_DAYS | 支持     | 支持             |
| **-R, --root CHROOT_DIR** 设置要切换到的 chroot 目录为 CHROOT_DIR | 支持     | 支持             |
| **-W, --warndays WARN_DAYS** 设置密码到期前的警告天数为 WARN_DAYS | 支持     | 支持             |

注意：原C程序的chage可执行文件权限设置了setuid位，且所有者为root，这样非root用户才能访问/etc/shadow等文件。重写的rust程序需要同样的权限方可正常运行。需要执行以下指令修改权限：

```
sudo chown root:root chage
sudo chmod u+s chage
```

### 软件自带用例对比验证

原有C程序对以下功能点进行了测试：

1. 单个选项，(例如：chage -d 0)
2. 混合选项
3. 交互模式获取正确参数
4. 交互模式获取非法参数
5. 输入非法的参数
6. shadow文件中缺失用户条目
7. 非root用户查看账户信息

用rust重写测试用例，测试结果如下：
![输入图片说明](https://foruda.gitee.com/images/1725343976112418875/584e3f10_14024037.png "f9e9e657ca658c67aa04b6afc8e1d41.png")
### 功能对比验证

对比验证的方式为：首先运行rust程序，另存运行后的/etc/shadow文件；再运行原C程序，对比/etc/shadow是否变化。

1. 功能点1：设置上次修改密码日期
   - 测试方式：运行`chage -d 2024-9-10`
   - 测试结果：/etc/shadow内容与原C程序运行结束一致
2. 功能点2：设置账户到期时间
   - 测试方式：运行`chage -E 2024-9-10`
   - 测试结果：/etc/shadow内容与原C程序运行结束一致
3. 功能点3：设置密码到期后多少天后账户会被禁用
   - 测试方式：运行`chage -I 100`
   - 测试结果：/etc/shadow内容与原C程序运行结束一致
4. 功能点4：设置密码更改前的最小天数
   - 测试方式：运行`chage -m 10`
   - 测试结果：/etc/shadow内容与原C程序运行结束一致
5. 功能点5：设置密码更改前的最大天数
   - 测试方式：运行`chage -M 10`
   - 测试结果：/etc/shadow内容与原C程序运行结束一致
6. 功能点6：设置密码到期前的警告天数
   - 测试方式：运行`chage -W 10`
   - 测试结果：/etc/shadow内容与原C程序运行结束一致
7. 功能点7：显示账户的期限信息
   - 测试方式：运行`chage -l username`
   - 测试结果：输出与原C程序一致
8. 功能点8：以YYYY-MM-DD格式显示账户的期限信息
   - 测试方式：运行`chage -l -i username`
   -  测试结果：输出与原C程序一致
9. 功能点9：运行交互模式
   - 测试方式：运行`chage username`，并根据提示信息输入新的期限信息
   - 测试结果：/etc/shadow内容与原C程序运行结束一致
10. 功能点10：普通用户查看账户信息
    - 测试方式：运行`su myuser -c "chage -l myuser`
    - 测试结果：显示`myuser`的账户信息，且与原C程序的输出一致

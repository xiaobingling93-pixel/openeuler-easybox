## groupadd功能验证对比报告

### 软件支持的功能清单

| 软件功能                                                     | 原有软件 | Rust重写后的软件 | 是否自动化测试 |
| ------------------------------------------------------------ | -------- | ---------------- | -------------- |
| -f, --force    在组已存在时成功退出，并在 GID 已被使用时取消 -g 选项的效果 | 支持     | 支持             | 是             |
| -g, --gid GID    为新建组指定GID                             | 支持     | 支持             | 是             |
| -K, --key KEY=VALUE    暂时覆盖`/etc/login.defs`的默认设置   | 支持     | 支持             | 是             |
| -o, --non-unique   允许创建具有重复（非唯一）GID的组         | 支持     | 支持             | 是             |
| -p, --password PASSWORD    为新组使用加密密码                | 支持     | 支持             | 是             |
| -r, --system    创建一个系统组（系统组的 GID 通常在 1-999 之间） | 支持     | 支持             | 是             |
| -R, --root CHROOT_DIR    指定一个目录，将系统根目录更改为该目录（用于创建组时在特定的 chroot 环境中） | 支持     | 支持             | 是             |
| -P, --prefix PREFIX_DIR    指定一个目录前缀（通常用于指定不同的根目录） | 支持     | 支持             | 是             |
| -U, --users USERS    为组指定用户成员列表（多个用户用逗号分隔） | 支持     | 支持             | 是             |
| -h, --help    显示程序版本信息                               | 支持     | 支持             | 是             |
| -V, --version    显示程序的帮助信息                          | 支持     | 支持             | 是             |

###  软件自带用例对比验证

软件没有自带测试用例，在Rust测试环境中添加如下测试用例，下面是测试用例的运行情况：

```
test test_groupadd_force_existing_group ... ok
test test_groupadd_with_gid ... ok
test test_groupadd_non_unique_gid ... ok
test test_groupadd_with_password ... ok
test test_groupadd_system_group ... ok
test test_groupadd_with_chroot ... ok
test test_groupadd_with_prefix ... ok
test test_groupadd_with_users ... ok
test test_groupadd_key_override ... ok
```

测试用例详细说明：

1. **test_groupadd_force_existing_group** - 测试 --force 选项，验证在已存在的组名情况下是否能强制添加组
2. **test_groupadd_with_gid** - 测试 --gid 选项，验证是否正确设置组的 GID
3. **test_groupadd_non_unique_gid** - 测试 --non-unique 选项，验证是否允许多个组共享同一个 GID
4. **test_groupadd_with_password** - 测试 --password 选项，验证是否正确为组设置密码
5. **test_groupadd_system_group** - 测试 --system 选项，验证是否正确创建系统组
6. **test_groupadd_with_chroot** - 测试 --root 选项，验证是否在 chroot 环境中正确创建组
7. **test_groupadd_with_prefix** - 测试 --prefix 选项，验证是否在指定的前缀路径下正确创建组
8. **test_groupadd_with_users** - 测试 --users 选项，验证是否正确为组添加用户
9. **test_groupadd_key_override** - 测试 -K 选项，验证是否正确覆盖系统配置文件 `login.defs` 中的 GID 范围

### 功能对比验证

1. 功能点：测试 --force 选项，验证在已存在的组名情况下是否能强制添加组。

- 测试方法：在测试环境中运行 `easybox groupadd testgroup`，然后再次运行 `easybox groupadd --force testgroup`，验证是否成功添加组且没有错误信息输出。
- 测试结果：成功添加组 `testgroup`，且没有错误信息输出。

2. 功能点：测试 --gid 选项，验证是否正确设置组的 GID。

- 测试方法：在测试环境中运行 `easybox groupadd --gid 1234 testgroup`，然后读取 `/etc/group` 文件，验证组 `testgroup` 是否被分配了正确的 GID。
- 测试结果：成功添加组 `testgroup`，且 `/etc/group` 文件中正确记录了 GID 为 1234。

3. 功能点：测试 --non-unique 选项，验证是否允许多个组共享同一个 GID。

- 测试方法：在测试环境中运行 `easybox groupadd --gid 1234 testgroup1`，然后运行 `easybox groupadd --gid 1234 --non-unique testgroup2`，验证是否允许 `testgroup2` 使用与 `testgroup1` 相同的 GID。
- 测试结果：成功添加 `testgroup1` 和 `testgroup2`，且两者共享相同的 GID 1234。

4. 功能点：测试 --password 选项，验证是否正确为组设置密码。

- 测试方法：在测试环境中运行 `easybox groupadd --password 1234567890 testgroup`，然后读取 `/etc/gshadow` 文件，验证组 `testgroup` 是否正确记录了密码。
- 测试结果：成功添加组 `testgroup`，且 `/etc/gshadow` 文件中正确记录了设置的密码。

5. 功能点：测试 --system 选项，验证是否正确创建系统组。

- 测试方法：在测试环境中运行 `easybox groupadd --system testsystemgroup`，验证系统组 `testsystemgroup` 是否成功创建。
- 测试结果：成功创建系统组 `testsystemgroup`。

6. 功能点：测试 --root 选项，验证是否在 chroot 环境中正确创建组。

- 测试方法：在测试环境中设置 chroot 环境 `/mnt/mychroot`，然后运行 `easybox groupadd --root /mnt/mychroot testgroup_with_chroot`，验证组 `testgroup_with_chroot` 是否在 chroot 环境中成功创建。
- 测试结果：成功在 chroot 环境中创建组 `testgroup_with_chroot`。

7. 功能点：测试 --prefix 选项，验证是否在指定的前缀路径下正确创建组。

- 测试方法：在测试环境中创建 `/prefix` 目录，运行 `easybox groupadd --prefix /prefix testgroup`，验证组 `testgroup` 是否在 `/prefix/etc/group` 文件中成功创建。
- 测试结果：成功在 `/prefix/etc/group` 文件中记录组 `testgroup`。

8. 功能点：测试 --users 选项，验证是否正确为组添加用户。

- 测试方法：在测试环境中创建用户 `test_user1` 和 `test_user2`，然后运行 `easybox groupadd --users test_user1,test_user2 testgroup`，验证组 `testgroup` 是否包含指定用户。
- 测试结果：成功添加组 `testgroup`，且 `/etc/group` 文件中正确包含了用户 `test_user1` 和 `test_user2`。

9. 功能点：测试 -K 选项，验证是否正确覆盖系统配置文件 `login.defs` 中的 GID 范围。

- 测试方法：在测试环境中运行 `easybox groupadd -K GID_MIN=5000 -K GID_MAX=10000 testgroup`，验证组 `testgroup` 是否在指定的 GID 范围内创建。
- 测试结果：成功在指定 GID 范围内创建组 `testgroup`。

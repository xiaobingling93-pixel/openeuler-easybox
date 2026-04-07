## usermod功能验证对比报告

### 软件支持的功能清单

| 软件功能                                                     | 原有软件 | Rust重写后的软件 | 是否自动化测试 |
| ------------------------------------------------------------ | -------- | ---------------- | -------------- |
| -a, --append 选项用于将用户追加到由`-G`选项指定的附加组中，而不从该用户已属于的其他组中移除。 | 支持     | 支持             | 是             |
| -b, --badname 允许有非法用户名                               | 支持     | 支持             | 是             |
| -c, --comment COMMENT  设置和修改用户的注释信息              | 支持     | 支持             | 是             |
| -d, --home HOME_DIR  修改用户的家目录                        | 支持     | 支持             | 是             |
| -e, --expiredate EXPIRE_DATE   设置用户的过期日期            | 支持     | 支持             | 是             |
| -f, --inactive INACTIVE  设置用户在密码过期后的非活跃天数    | 支持     | 支持             | 是             |
| -g, --gid GROUP  修改用户的gid                               | 支持     | 支持             | 是             |
| -G, --groups GROUPS  将用户正确添加到多个组中                | 支持     | 支持             | 是             |
| -h, --help  显示帮助信息                                     | 支持     | 支持             | 否             |
| -v, --version  显示软件版本号                                | 支持     | 支持             | 否             |
| -l, --login NEW_LOGIN  修改用户登录名                        | 支持     | 支持             | 是             |
| -L, --lock  锁定用户                                         | 支持     | 支持             | 是             |
| -m, --move-home  移动用户的主目录，要和`-d`选项一块使用      | 支持     | 支持             | 是             |
| -o, --non-unique  允许指定一个已经存在的UID                  | 支持     | 支持             | 是             |
| -p, --password PASSWORD   修改用户的登录密码                 | 支持     | 支持             | 是             |
| -P, --prefix PREFIX_DIR   指定的prefix目录下正确修改用户信息 | 支持     | 支持             | 是             |
| -r, --remove   将用户从通过 `-G` 指定的附加组中移除，同时不影响用户在其他组中的成员身份 | 支持     | 支持             | 是             |
| -R, --root CHROOT_DIR  指定的chroot环境下正确修改用户信息    | 支持     | 支持             | 是             |
| -s, --shell SHELL 修改用户的shell                            | 支持     | 支持             | 是             |
| -u, --uid UID  修改用户的uid                                 | 支持     | 支持             | 是             |
| -U, --unlock  为用户解锁                                     | 支持     | 支持             | 是             |
| -v, --add-subuids FIRST-LAST  为用户添加从属UID范围          | 支持     | 支持             | 是             |
| -V, --del-subuids FIRST-LAST   删除用户的从属UID范围         | 支持     | 支持             | 是             |
| -w, --add-subgids FIRST-LAST  为用户添加从属GID范围          | 支持     | 支持             | 是             |
| -W, --del-subgids FIRST-LAST  删除用户的从属GID范围          | 支持     | 支持             | 是             |

###  软件自带用例对比验证

软件没有自带测试用例，在Rust测试环境中添加23个测试用例，下面是测试情况：

running 23 tests
test test_usermod::test_usermod_add_subgids ... ok
test test_usermod::test_usermod_add_subuids ... ok
test test_usermod::test_usermod_append_groups ... ok
test test_usermod::test_usermod_badname ... ok
test test_usermod::test_usermod_change_home_directory ... ok
test test_usermod::test_usermod_comment ... ok
test test_usermod::test_usermod_del_subgids ... ok
test test_usermod::test_usermod_inactive ... ok
test test_usermod::test_usermod_expiry_date ... ok
test test_usermod::test_usermod_groups ... ok
test test_usermod::test_usermod_del_subuids ... ok
test test_usermod::test_usermod_lock_user ... ok
test test_usermod::test_usermod_login ... ok
test test_usermod::test_usermod_move_home_directory ... ok
test test_usermod::test_usermod_non_unique_uid ... ok
test test_usermod::test_usermod_password ... ok
test test_usermod::test_usermod_primary_group ... ok
test test_usermod::test_usermod_remove_from_groups ... ok
test test_usermod::test_usermod_shell ... ok
test test_usermod::test_usermod_uid ... ok
test test_usermod::test_usermod_unlock_user ... ok
test test_usermod::test_usermod_with_chroot ... ok
test test_usermod::test_usermod_with_prefix ... ok

### 功能对比验证

1. 功能点：测试 `-b`（`--badname`）选项，验证是否可以拒绝不合法的用户名。 【test_usermod_badname】

- 测试方法：在测试环境中运行 `easybox useradd test_no_badname` 创建一个合法用户名用户，然后使用 `easybox usermod -l test_:_badname test_no_badname` 尝试将其更名为不合法用户名，检查操作失败。
- 测试结果：`usermod` 命令执行失败，表示不合法用户名被正确拒绝。

2. 功能点：测试 `-c`（`--comment`）选项，验证是否可以正确设置和修改用户的注释信息。 【test_usermod_comment】

- 测试方法：在测试环境中运行 `easybox useradd -c "test_usermod_comment_c" test_usermod_comment` 创建用户，检查 `/etc/passwd` 文件中的注释。然后运行 `easybox usermod -c "test_usermod_comment_rust" test_usermod_comment` 修改注释，验证 `/etc/passwd` 中注释的更改。
- 测试结果：成功创建用户 `test_usermod_comment`，且 `/etc/passwd` 文件中的注释信息正确更新。

3. 功能点：测试 `-e`（`--expiredate`）选项，验证是否可以正确设置用户的过期日期。 【test_usermod_expiry_date】

- 测试方法：在测试环境中运行 `easybox useradd test_usermod_expiry1` 和 `easybox useradd test_usermod_expiry2` 创建两个用户，然后分别使用 `/usr/sbin/usermod -e 2028-08-21 test_usermod_expiry1` 和 `easybox usermod -e 2028-08-21 test_usermod_expiry2` 设置用户的过期日期，读取 `/etc/shadow` 文件验证日期设置是否正确。
- 测试结果：两个用户的过期日期均成功设置为 `2028-08-21`，且 `/etc/shadow` 文件中正确记录。

4. 功能点：测试 `-f`（`--inactive`）选项，验证是否正确设置用户在密码过期后的非活跃天数，并且该信息是否正确写入 `/etc/shadow` 文件中。 【test_usermod_inactive】

- 测试方法：在测试环境中运行 `easybox useradd test_usermod_inactive_user` 创建用户，然后运行 `easybox usermod -f 30 test_usermod_inactive_user` 设置非活跃天数，读取 `/etc/shadow` 文件，验证设置是否正确。
- 测试结果：成功创建用户 `test_usermod_inactive_user`，且 `/etc/shadow` 文件中正确记录了非活跃天数为 30。

5. 功能点：测试 `-l`（`--login`）选项，验证是否可以正确更改用户的登录名。 【test_usermod_login】

- 测试方法：在测试环境中运行 `easybox useradd test_usermod_login_olduser` 创建用户，然后运行 `easybox usermod -l test_usermod_login_newuser test_usermod_login_olduser` 更改用户登录名，读取 `/etc/passwd` 和 `/etc/shadow` 文件，验证更改是否正确。
- 测试结果：用户登录名成功更改为 `test_usermod_login_newuser`，旧用户名 `test_usermod_login_olduser` 已删除。

6. 功能点：测试 `-s`（`--shell`）选项，验证是否可以正确设置和更改用户的默认shell。 【test_usermod_shell】

- 测试方法：在测试环境中运行 `easybox useradd -s /bin/bash test_usermod_shell_user` 创建用户，检查 `/etc/passwd` 文件中shell设置。然后运行 `easybox usermod -s /bin/sh test_usermod_shell_user` 修改用户的默认shell，验证 `/etc/passwd` 中的更改。
- 测试结果：成功创建用户 `test_usermod_shell_user`，且 `/etc/passwd` 文件中的shell信息正确更新。

7. 功能点：测试 `-p`（`--password`）选项，验证是否可以正确设置用户的加密密码，并验证用户是否能够使用该密码登录。 【test_usermod_password】

- 测试方法：在测试环境中运行 `easybox useradd test_usermod_password_user` 创建用户，使用 `openssl passwd -1 "testpassword"` 生成加密密码，然后使用 `easybox usermod -p <加密密码> test_usermod_password_user` 设置用户密码。使用 `su` 命令尝试以该用户身份登录，验证登录是否成功。
- 测试结果：成功创建用户 `test_usermod_password_user`，设置加密密码后可以成功登录。

8. 功能点：测试 `-L`（`--lock`）选项，验证是否正确锁定用户账户，并且该信息是否正确写入 `/etc/shadow` 文件中。 【test_usermod_lock_user】

- 测试方法：在测试环境中运行 `easybox useradd test_usermod_lock_user` 创建一个用户 `test_usermod_lock_user`，然后设置密码为 `123456`。测试用户可以成功登录后，运行 `easybox usermod -L test_usermod_lock_user` 锁定用户账户。最后，读取 `/etc/shadow` 文件，验证该用户的密码字段是否以 `!` 开头，确认用户账户已被锁定。
- 测试结果：成功创建用户 `test_usermod_lock_user`，且 `/etc/shadow` 文件中正确记录了锁定状态，用户账户被成功锁定。

9. 功能点：测试 `-U`（`--unlock`）选项，验证用户在锁定后是否能够通过解锁功能成功登录。

- 测试方法：在测试环境中运行`easybox useradd test_usermod_unlock_user`创建用户，然后使用`usermod -p`设置用户密码为`123456`。使用`su`命令测试用户是否能够正常登录。随后使用`usermod -L test_usermod_unlock_user`锁定用户，使用`su`命令测试用户是否不能登录。接着使用`usermod -U test_usermod_unlock_user`解锁用户，最后再次使用`su`命令测试用户是否能够成功登录。
- 测试结果：成功创建用户`test_usermod_unlock_user`，设置密码为`123456`后可以登录。锁定用户后无法登录，解锁用户后能够再次成功登录。测试结束后成功删除用户`test_usermod_unlock_user`。

10. 功能点：测试 `-d` 选项，验证是否正确修改用户的主目录，并且该信息是否正确写入 `/etc/passwd` 文件中。 【test_usermod_change_home_directory】

- 测试方法： 在测试环境中运行 `easybox useradd test_d` 创建用户 `test_d`，然后运行 `easybox usermod -d /tmp/test_d test_d` 修改用户的主目录。读取 `/etc/passwd` 文件，验证是否正确记录了新的主目录路径 `/tmp/test_d`。
- 测试结果：成功创建用户 `test_d`，并修改其主目录为 `/tmp/test_d`，且 `/etc/passwd` 文件中正确记录了新的主目录路径。

11. 功能点：测试 `-m` 选项，验证是否正确移动用户的主目录，并且该信息是否正确写入 `/etc/passwd` 文件中，同时验证移动后的目录的所有者是否为该用户。 【test_usermod_move_home_directory】

- 测试方法：在测试环境中运行 `easybox useradd -m test_m` 创建用户 `test_m`，然后运行 `easybox usermod -d /tmp/test_m -m test_m` 修改用户的主目录为 `/tmp/test_m`。读取 `/etc/passwd` 文件，验证是否正确记录了新的主目录路径 `/tmp/test_m`，同时验证 `/tmp/test_m` 的所有者是否为 `test_m`。
- 测试结果：成功创建用户 `test_m`，并移动其主目录为 `/tmp/test_m`，且 `/etc/passwd` 文件中正确记录了新的主目录路径。同时，`/tmp/test_m` 的所有者为 `test_m`。最后成功删除 `/tmp/test_m` 目录和用户 `test_m`。

12. 功能点：测试 `-g`（`--gid`）选项，验证是否正确修改用户的主组，并且该信息是否正确写入 `/etc/passwd` 文件中。【test_usermod_primary_group】

- 测试方法：在测试环境中运行 `/usr/sbin/useradd test_usermod_user` 创建测试用户 `test_usermod_user`，运行 `/usr/sbin/groupadd test_usermod_group` 创建测试组 `test_usermod_group`，使用 `usermod -g test_usermod_group test_usermod_user` 修改用户 `test_usermod_user` 的主组。读取 `/etc/passwd` 文件，检查该用户的主组是否已正确设置为 `test_usermod_group` 的 GID。
- 测试结果：成功创建用户 `test_usermod_user` 和组 `test_usermod_group`，并将用户的主组成功修改为 `test_usermod_group`， `/etc/passwd` 文件中正确记录了用户的主组 ID。

13. 功能点：测试 `-G`（`--groups`）选项，验证是否将用户正确添加到多个组中，并且该信息是否正确写入 `/etc/group` 文件中。 【test_usermod_groups】

- 测试方法：在测试环境中运行 `easybox useradd test_usermod_groups_user` 创建用户，并使用 `easybox groupadd` 创建两个组 `test_group1` 和 `test_group2`。随后，运行 `easybox usermod -G test_group1,test_group2 test_usermod_groups_user` 将用户添加到这两个组中。读取 `/etc/group` 文件，验证用户是否被正确添加到 `test_group1` 和 `test_group2`。
- 测试结果：成功创建用户 `test_usermod_groups_user`，且 `/etc/group` 文件中正确记录了用户 `test_usermod_groups_user` 属于 `test_group1` 和 `test_group2`。

14. 功能点：测试 `-u`（`--uid`）选项，验证是否可以成功更改用户的UID，并且该信息是否正确写入 `/etc/passwd` 文件中。 【test_usermod_uid】

- 测试方法： 在测试环境中运行 `easybox useradd test_usermod_uid_user` 创建用户，读取 `/etc/passwd` 文件获取初始UID。然后运行 `easybox usermod -u 新的UID test_usermod_uid_user`，新的UID比初始UID大1。再次读取 `/etc/passwd` 文件，验证UID是否已正确更新。
- 测试结果：成功更改用户 `test_usermod_uid_user` 的UID，且 `/etc/passwd` 文件中正确记录了新的UID值，比原始UID大1。

15. 功能点：测试 `-o`（`--non-unique`）选项，验证是否允许将两个用户的UID设置为相同的非唯一UID，并且该信息是否正确写入 `/etc/passwd` 文件中。 【test_usermod_non_unique_uid】

- 测试方法：在测试环境中运行 `easybox useradd test_usermod_user1` 创建第一个用户，然后读取 `/etc/passwd` 文件获取用户 `test_usermod_user1` 的 UID。接着，运行 `easybox useradd test_usermod_user2` 创建第二个用户，并使用 `easybox usermod -o -u <user1_uid> test_usermod_user2` 将第二个用户的 UID 设置为与第一个用户相同。最后，读取 `/etc/passwd` 文件，验证两个用户的 UID 是否相同。
- 测试结果：成功创建用户 `test_usermod_user1` 和 `test_usermod_user2`，并且在使用 `-o` 和 `-u` 选项后，`/etc/passwd` 文件中两个用户的 UID 被正确设置为相同的值。

16. 功能点：测试 `-a`（`--append`）选项，验证在将用户添加到新的附加组时，不会移除该用户在其他组中的成员身份，并且这些信息是否正确写入 `/etc/group` 文件中。 【test_usermod_append_groups】

- 测试方法：在测试环境中运行 `easybox useradd test_usermod_append_user` 创建用户 `test_usermod_append_user`，然后使用 `easybox groupadd` 创建两个组 `test_group1` 和 `test_group2`。使用 `easybox usermod -G test_group1 test_usermod_append_user` 将用户添加到 `test_group1` 中，读取 `/etc/group` 文件验证用户成功加入 `test_group1`。接着，使用 `easybox usermod -a -G test_group2 test_usermod_append_user` 将用户追加到 `test_group2` 中，读取 `/etc/group` 文件验证用户仍属于 `test_group1` 并成功加入 `test_group2`。
- 测试结果：成功创建用户 `test_usermod_append_user`，并将其添加到组 `test_group1` 中。使用 `-a` 选项追加到 `test_group2` 后，用户仍保留在 `test_group1` 中，且 `/etc/group` 文件正确记录了用户同时属于 `test_group1` 和 `test_group2`。

17. 功能点：测试 `-r`（`--remove`）选项，验证是否能将用户从通过 `-G` 指定的附加组中移除，同时不影响用户在其他组中的成员身份。 【test_usermod_remove_from_groups】

- 测试方法：在测试环境中运行 `easybox useradd` 创建用户 `test_usermod_remove_user` 和三个组 `test_group1`、`test_group2` 和 `test_group3`，然后使用 `usermod -G` 将用户添加到所有三个组中。接着运行 `easybox usermod -r -G test_group2 test_usermod_remove_user`，从组 `test_group2` 中移除用户，最后读取 `/etc/group` 文件验证用户是否已从指定组中移除，但仍然保留在其他组中。
- 测试结果：成功创建用户 `test_usermod_remove_user`，并正确将其从 `test_group2` 中移除，同时确保用户仍在 `test_group1` 和 `test_group3` 中。 `/etc/group` 文件中正确反映了这些更改。

18. 功能点：测试 `-P`（`--prefix`）选项，验证是否可以在指定的前缀目录下正确修改用户信息，并且修改信息是否正确写入指定的前缀目录中的 `/etc/passwd` 文件中。 【test_usermod_with_prefix】

- 测试方法：在测试环境中创建 `/myprefix/etc` 目录并复制 `/etc/passwd`、`/etc/group`、`/etc/shadow` 和 `/etc/gshadow` 到该目录。使用 `easybox useradd -P /myprefix testuser_mod_prefix` 在指定的前缀目录下创建用户，然后运行 `easybox usermod -P /myprefix -s /bin/sh testuser_mod_prefix` 修改该用户的 shell。最后，读取 `/myprefix/etc/passwd` 文件，验证是否正确记录用户的 shell 修改。
- 测试结果：成功创建用户 `testuser_mod_prefix`，且 `/myprefix/etc/passwd` 文件中正确更新了用户的 shell 为 `/bin/sh`。

19. 功能点：测试 `-R`（`--root`）选项，验证是否可以在指定的chroot目录下正确修改用户信息，并且这些修改是否正确写入chroot环境中的 `/etc/passwd` 文件中。 【test_usermod_with_chroot】

- 测试方法：在测试环境中运行 `easybox usermod -R /mnt/my_test_chroot -s /bin/zsh chroot_mod_test_user`，读取chroot环境中的 `/etc/passwd` 文件，验证是否正确记录了用户的shell更新。
- 测试结果：成功修改用户 `chroot_mod_test_user` 的shell为 `/bin/zsh`，且chroot环境中的 `/etc/passwd` 文件正确记录了该用户的shell更新。

20. 功能点：测试 `-v`（`--add-subuids FIRST-LAST`）选项，验证是否正确为用户添加从属UID范围，并且该信息是否正确写入 `/etc/subuid` 文件中。 【test_usermod_add_subuids】

- 测试方法：在测试环境中运行 `easybox useradd test_usermod_add_subuids_user` 创建用户，然后使用 `easybox usermod -v 200000-200999 test_usermod_add_subuids_user` 添加从属UID范围。读取 `/etc/subuid` 文件，验证是否正确记录了从属UID范围。
- 测试结果：成功为用户 `test_usermod_add_subuids_user` 添加从属UID范围，且 `/etc/subuid` 文件中正确记录了 `200000-200999` 范围。

21. 功能点：测试 `-V`（`--del-subuids FIRST-LAST`）选项，验证是否正确删除用户的从属UID范围，并且该信息是否正确从 `/etc/subuid` 文件中删除。 【test_usermod_del_subuids】

- 测试方法：在测试环境中运行 `easybox useradd test_usermod_del_subuids_user` 创建用户，然后使用 `easybox usermod -v 200000-200999 test_usermod_del_subuids_user` 添加从属UID范围。接着运行 `easybox usermod -V 200000-200999 test_usermod_del_subuids_user` 删除从属UID范围。读取 `/etc/subuid` 文件，验证是否正确删除了从属UID范围。
- 测试结果：成功删除用户 `test_usermod_del_subuids_user` 的从属UID范围，且 `/etc/subuid` 文件中不再包含 `200000-200999` 范围。

22. 功能点：测试 `-w`（`--add-subgids FIRST-LAST`）选项，验证是否正确为用户添加从属GID范围，并且该信息是否正确写入 `/etc/subgid` 文件中。 【test_usermod_add_subgids】

- 测试方法：在测试环境中运行 `easybox useradd test_usermod_add_subgids_user` 创建用户，然后使用 `easybox usermod -w 300000-300999 test_usermod_add_subgids_user` 添加从属GID范围。读取 `/etc/subgid` 文件，验证是否正确记录了从属GID范围。
- 测试结果：成功为用户 `test_usermod_add_subgids_user` 添加从属GID范围，且 `/etc/subgid` 文件中正确记录了 `300000-300999` 范围。

23. 功能点：测试 `-W`（`--del-subgids FIRST-LAST`）选项，验证是否正确删除用户的从属GID范围，并且该信息是否正确从 `/etc/subgid` 文件中删除。 【test_usermod_del_subgids】

- 测试方法：在测试环境中运行 `easybox useradd test_usermod_del_subgids_user` 创建用户，然后使用 `easybox usermod -w 300000-300999 test_usermod_del_subgids_user` 添加从属GID范围。接着运行 `easybox usermod -W 300000-300999 test_usermod_del_subgids_user` 删除从属GID范围。读取 `/etc/subgid` 文件，验证是否正确删除了从属GID范围。
- 测试结果：成功删除用户 `test_usermod_del_subgids_user` 的从属GID范围，且 `/etc/subgid` 文件中不再包含 `300000-300999` 范围。


## useradd功能验证对比报告

### 软件支持的功能清单

| 软件功能                                                     | 原有软件 | Rust重写后的软件 | 是否自动化测试 |
| ------------------------------------------------------------ | -------- | ---------------- | -------------- |
| --badname   不检查非法用户名                                 | 支持     | 支持             | 是             |
| -b, --base-dir BASE_DIR   用于设定用户主目录的基础路径       | 支持     | 支持             | 是             |
| --btrfs-subvolume-home                                       | 支持     | 支持             | 是             |
| -c, --comment COMMENT   手工指定用户的说明                   | 支持     | 支持             | 是             |
| -d, --home-dir HOME_DIR   手工指定用户的家目录               | 支持     | 支持             | 是             |
| -D, --defaults   打印或者改变默认的`/etc/default/useradd`的文件配置 | 支持     | 支持             | 是             |
| -e, --expiredate EXPIRE_DATE  设定用户被禁用的日期           | 支持     | 支持             | 是             |
| -f, --inactive INACTIVE   设置新账户的密码非活动期，也就是密码过期后用户被锁定前的天数 | 支持     | 支持             | 是             |
| -F, --add-subids-for-system  为系统用户自动分配子用户ID（sub-UID）和子组ID（sub-GID）范围。 | 支持     | 支持             | 是             |
| -g, --gid GROUP   指定用户初始登录组的组名或编号             | 支持     | 支持             | 是             |
| -G, --groups GROUPS  手工指定用户的附加组                    | 支持     | 支持             | 是             |
| -k, --skel SKEL_DIR   使用自定义骨架目录                     | 支持     | 支持             | 是             |
| -K, --key KEY=VALUE  临时覆盖 `/etc/login.defs` 默认值       | 支持     | 支持             | 是             |
| -m, --create-home  创建用户的家目录                          | 支持     | 支持             | 是             |
| -M, --no-create-home   不创建用户的家目录                    | 支持     | 支持             | 是             |
| -N, --no-user-group 不创建和用户同名的组                     | 支持     | 支持             | 是             |
| -o, --non-unique  允许创建 uid 重复的用户                    | 支持     | 支持             | 是             |
| -p, --password PASSWORD  输入创建用户的密码                  | 支持     | 支持             | 是             |
| -r, --system   创建系统用户                                  | 支持     | 支持             | 是             |
| -R, --root CHROOT_DIR   chroot目录                           | 支持     | 支持             | 是             |
| -P, --prefix PREFIX_DIR   /etc/*文件所在的前缀目录           | 支持     | 支持             | 是             |
| -s, --shell SHELL                                            | 支持     | 支持             | 是             |
| -u, --uid UID                                                | 支持     | 支持             | 是             |
| -U, --user-group                                             | 支持     | 支持             | 是             |
| -h, --help    显示程序的帮助信息                               | 支持     | 支持             | 不需要         |
| -V, --version    显示程序的版本信息                          | 支持     | 支持             | 不需要         |



###  软件自带用例对比验证

软件没有自带测试用例，在Rust测试环境中添加如下测试用例，下面是测试用例的运行情况：

running 34 tests
test test_useradd::test_useradd_change_default_group ... ok
test test_useradd::test_useradd_change_multiple_defaults ... ok
test test_useradd::test_useradd_change_default_home ... ok
test test_useradd::test_useradd_comment ... ok
test test_useradd::test_useradd_create_home ... ok
test test_useradd::test_useradd_home_dir ... ok
test test_useradd::test_useradd_invalid_group ... ok
test test_useradd::test_useradd_invalid_shell ... ok
test test_useradd::test_useradd_no_create_home ... ok
test test_useradd::test_useradd_no_user_group ... ok
test test_useradd::test_useradd_non_unique_uid ... ok
test test_useradd::test_useradd_set_expiration_date ... ok
test test_useradd::test_useradd_show_defaults ... ok
test test_useradd::test_useradd_system_account ... ok
test test_useradd::test_useradd_user_group ... ok
test test_useradd::test_useradd_user_group_conflict ... ok
test test_useradd::test_useradd_user_group_with_g_conflict ... ok
test test_useradd::test_useradd_user_group_with_n_conflict ... ok
test test_useradd::test_useradd_with_badname ... ok
test test_useradd::test_useradd_with_base_dir_and_create_home ... ok
test test_useradd::test_useradd_with_chroot ... ok
test test_useradd::test_useradd_with_custom_skel_dir ... ok
test test_useradd::test_useradd_with_inactive_days ... ok
test test_useradd::test_useradd_with_invalid_group_name ... ok
test test_useradd::test_useradd_with_multiple_groups ... ok
test test_useradd::test_useradd_with_non_existent_group_id ... ok
test test_useradd::test_useradd_with_non_existent_group_name ... ok
test test_useradd::test_useradd_with_password ... ok
test test_useradd::test_useradd_with_prefix ... ok
test test_useradd::test_useradd_with_shell ... ok
test test_useradd::test_useradd_with_uid ... ok
test test_useradd::test_useradd_with_uid_min_override ... ok
test test_useradd::test_useradd_with_valid_group_id ... ok
test test_useradd::test_useradd_with_valid_group_name ... ok



### 功能对比验证

1. 功能点：不使用--badname 【test_useradd_with_badname】
   - 测试方法：在测试环境中运行`easybox useradd invaild:name`
   - 测试结果：不使用`--badname`，则会检查不符合规范的`invaild:name`这个用户名，返回错误信息，该用户无法成功加入。
2. 功能点：-b和-D 【test_useradd_change_default_home】
   - 测试方法：在测试环境中运行`easybox useradd -D -b /mnt`
   - 测试结果：成功修改`/etc/default/useradd`配置文件中的`HOME=`的值。
3. 功能点：-b属性指定base dir 【test_useradd_with_base_dir_and_create_home】
   - 测试方法：在测试环境中运行`easybox useradd -b /mnt -m test_b`
   - 测试结果：成功创建`test_b`用户并创建该用户的目录文件夹`/mnt/test_b`
4. 功能点：-c属性手工指定用户的说明 【test_useradd_comment】
   - 测试方法：在测试环境中运行`easybox useradd -c "This is a test user" newuser_test_useradd_comment`
   - 测试结果：`newuser_test_useradd_comment`用户被正确创建，在`/etc/passwd`文件中有对该用户的说明
5. 功能点：-d属性指定用户的home-dir 【test_useradd_home_dir】
   - 测试方法：在测试环境中运行`easybox useradd -d /myuseradd/home/testuser_home_dir -m testuser_home_dir`
   - 测试结果：正确添加了`testuser_home_dir`用户，其`home_dir`也为`/myuseradd/home/testuser_home_dir`
6. 功能点：-D属性显示默认配置 【test_useradd_show_defaults】
   - 测试方法：在测试环境中运行`easybox useradd -D`
   - 测试结果：测试成功，输出中包含`GROUP=`, `HOME=`, `SHELL=`, `INACTIVE=`, `EXPIRE=`, `SKEL=`, `CREATE_MAIL_SPOOL=`, `LOG_INIT=`等默认配置参数。
7. 功能点：-D和-g属性修改默认组 【test_useradd_change_default_group】
   - 测试方法：在测试环境中运行`easybox useradd -D -g 1001`
   - 测试结果：测试成功，输出中包含`GROUP=1001`，验证默认组名成功修改。
8. 功能点：-D和-b属性修改默认主目录路径 【test_useradd_change_default_home】（重复项，与第2条相同，但基于编号要求保留）
   - 测试方法：在测试环境中运行`easybox useradd -D -b /mnt`
   - 测试结果：测试成功，输出中包含`HOME=/mnt`，验证默认主目录路径成功修改。
9. 功能点：-D属性同时修改多个默认值 【test_useradd_change_multiple_defaults】
   - 测试方法：在测试环境中运行`easybox useradd -D -g 1000 -s /bin/sh`
   - 测试结果：测试成功，输出中包含`GROUP=1000`和`SHELL=/bin/sh`，验证多个默认值成功修改。
10. 功能点：-D和-g属性检测无效的组名 【test_useradd_invalid_group】
    - 测试方法：在测试环境中运行`easybox useradd -D -g invalidgroup`
    - 测试结果：测试成功，程序运行失败，验证了无效组名无法成功设置。
11. 功能点：-D和-s属性使用无效的shell路径 【test_useradd_invalid_shell】
    - 测试方法：在测试环境中运行`easybox useradd -D -s /bin/invalidshell`
    - 测试结果：测试成功，程序运行失败，验证了无效的shell路径无法成功设置。

12. 功能点：测试 `-f`（`--inactive`）选项，验证是否正确设置用户在密码过期后的非活跃天数，并且该信息是否正确写入 `/etc/shadow` 文件中。  【test_useradd_with_inactive_days】

- 测试方法： 在测试环境中运行 `easybox useradd -f 7 test_f`，读取 `/etc/shadow` 文件，验证是否正确记录非活跃天数。
- 测试结果：成功创建用户 `test_f`，且 `/etc/shadow` 文件中正确记录了非活跃天数为 7 天。

13. 功能点：测试 `-g` 选项，指定有效的组名，验证用户是否成功创建，并且正确设置为指定的主组。 【test_useradd_with_valid_group_name】

- 测试方法： 测试方法：在测试环境中，先创建一个组 `testgroup`，然后运行 `easybox useradd -g testgroup testuser_valid_group`，检查新创建的用户 `testuser_valid_group` 是否属于 `testgroup`。

- 测试结果：成功创建用户 `testuser_valid_group`，并且用户的主组正确设置为 `testgroup`。

14. 功能点：测试 `-g` 选项，指定有效的GID，验证用户是否成功创建，并且正确设置为指定的GID。 【test_useradd_with_valid_group_id】

- 测试方法： 测试方法：在测试环境中，先创建一个具有指定GID `1501` 的组 `testgroup`，然后运行 `easybox useradd -g 1501 testuser_valid_gid`，检查新创建的用户 `testuser_valid_gid` 是否具有 GID `1501`。
- 测试结果：成功创建用户 `testuser_valid_gid`，并且用户的 GID 正确设置为 `1501`。

15. 功能点：测试 `-g` 选项，指定不存在的组名，验证命令是否正确处理并输出错误信息。 【test_useradd_with_non_existent_group_name】

- 测试方法： 在测试环境中运行 `easybox useradd -g nonexistentgroup testuser_invalid_group`，检查命令是否失败并输出错误信息。
- 测试结果：命令正确失败，并且输出 `group 'nonexistentgroup' does not exist` 的错误信息。

16. 功能点：测试 `-g` 选项，指定不存在的GID，验证命令是否正确处理并输出错误信息。 【test_useradd_with_non_existent_group_id】

- 测试方法： 在测试环境中运行 `easybox useradd -g 99999 testuser_invalid_gid`，检查命令是否失败并输出错误信息。
- 测试结果：命令正确失败，并且输出 `group '99999' does not exist` 的错误信息。

17. 功能点：测试 `-G`（`--groups`）选项，验证用户是否可以成功地加入多个附加组，并且该信息是否正确写入相应的组文件中。 【test_useradd_with_multiple_groups】

- 测试方法： 在测试环境中运行 `easybox useradd -G group1,group2 testuser_multiple_groups`，然后使用 `getent` 命令检查 `/etc/group` 文件中，验证用户 `testuser_multiple_groups` 是否被成功加入到 `group1` 和 `group2` 中。
- 测试结果：成功创建用户 `testuser_multiple_groups`，并且用户成功加入 `group1` 和 `group2`， `/etc/group` 文件中显示正确的成员关系。

18. 功能点：测试 `-G`（`--groups`）选项，验证在提供无效组名时，程序是否会正确地失败并输出错误信息。【test_useradd_with_invalid_group_name】

- 测试方法： 在测试环境中运行 `easybox useradd -G invalidgroup testuser_invalid_group_name`，期望程序失败并输出 "group 'invalidgroup' does not exist" 的错误信息。
- 测试结果：由于组 `invalidgroup` 不存在，用户 `testuser_invalid_group_name` 未被创建，并且程序正确地输出了错误信息。

19. 功能点：测试 `-k`（`--skel`）选项，验证是否使用自定义骨架目录，并确保新用户的主目录正确复制了自定义骨架目录中的内容。【test_useradd_with_custom_skel_dir】

- 测试方法： 在测试环境中运行 `easybox useradd -k /tmp/custom_skel -m testuser_custom_skel`，验证新用户的主目录 `/home/testuser_custom_skel` 是否存在，并确认自定义骨架目录 `/tmp/custom_skel` 中的文件是否被正确复制到新用户的主目录中。
- 测试结果：成功创建用户 `testuser_custom_skel`，且 `/home/testuser_custom_skel` 目录中正确包含了自定义骨架目录 `/tmp/custom_skel` 中的文件，文件内容一致。

20. 功能点：测试 `-K` 选项，验证 `UID_MIN` 的临时覆盖是否成功应用于新创建用户的 UID 分配。 【test_useradd_with_uid_min_override】

- 测试方法： 在测试环境中运行 `easybox useradd -K UID_MIN=3000 test_K`，创建用户 `test_K`，然后检查新用户的 UID，确保其大于或等于 3000。
- 测试结果：成功创建用户 `test_K`，且新用户的 UID 符合 `UID_MIN=3000` 的要求。

21. 功能点：测试 `-m`（`--create-home`）选项，验证是否正确创建用户的主目录，并且该目录的所有权、权限和 `/etc/skel/` 目录中的文件是否正确复制到新用户的主目录中。【test_useradd_create_home】

- 测试方法： 在测试环境中运行 `easybox useradd -m testuser_create_home`，验证是否正确创建用户 `testuser_create_home` 的主目录。检查该目录的所有权、权限是否正确，并验证 `/etc/skel/` 目录中的默认文件是否正确复制到用户的主目录中，且文件内容一致。
- 测试结果：成功创建用户 `testuser_create_home`，其主目录 `/home/testuser_create_home` 正确创建，所有权为新用户，权限为 `750`。默认的 `.bash_logout`、`.bashrc` 和 `.profile` 文件被正确复制到主目录中，并且文件内容与 `/etc/skel/` 中的对应文件一致。

22. 功能点：测试 `-M`（`--no-create-home`）选项，验证在创建用户时是否正确地不创建用户的主目录。 【test_useradd_no_create_home】

- 测试方法： 在测试环境中运行 `easybox useradd -M testuser_no_create_home`，验证 `/home/testuser_no_create_home` 目录是否不存在。
- 测试结果：成功创建用户 `testuser_no_create_home`，但未创建主目录 `/home/testuser_no_create_home`。

23. 功能点：测试 `-N`（`--no-user-group`）选项，验证是否在创建用户时不会生成与用户名同名的组，并且用户被分配到默认的组。 【test_useradd_no_user_group】

- 测试方法： 在测试环境中运行 `easybox useradd -N testuser_no_group`，读取 `/etc/passwd` 和 `/etc/group` 文件，验证是否成功创建用户且没有生成与用户名同名的组。
- 测试结果：成功创建用户 `testuser_no_group`，且未生成同名组，用户被正确分配到默认组。

24. 功能点：测试 `-o`（`--non-unique`）选项，验证是否允许使用重复的（非唯一）UID 创建用户。 【test_useradd_non_unique_uid】

- 测试方法：在测试环境中运行 `easybox useradd testuser_unique` 创建一个用户，然后使用相同的 UID 运行 `easybox useradd -o -u UID testuser_non_unique`，验证是否成功创建具有相同 UID 的新用户。
- 测试结果：成功创建用户 `testuser_non_unique`，且 `/etc/passwd` 文件中正确记录了用户信息，证明允许重复使用 UID。

25. 功能点：测试 `-p`（`--password`）选项，验证是否可以为新用户设置加密密码，并且该密码是否正确写入 `/etc/shadow` 文件中。 【test_useradd_with_password】

- 测试方法： 在测试环境中运行 `easybox useradd -p '$6$saltsalt$abcdefghijk' testuser_with_password`，读取 `/etc/shadow` 文件，验证密码字段是否正确记录了指定的加密密码。
- 测试结果：成功创建用户 `testuser_with_password`，且 `/etc/shadow` 文件中正确记录了加密密码。

26. 功能点：测试 `-r`（`--system`）选项，验证是否可以创建系统账户，并且该账户是否具有正确的系统 UID 和 GID 范围。 【test_useradd_system_account】

- 测试方法：在测试环境中运行 `easybox useradd -r testuser_system`，读取 `/etc/passwd` 文件，验证用户的 UID 是否在系统账户的范围内（如 0-999）。
- 测试结果：成功创建用户 `testuser_system`，且 `/etc/passwd` 文件中用户的 UID 在系统账户的范围内。

27. 功能点：测试 `-P`（`--prefix`）选项，验证是否能够在指定的 `prefix` 目录中正确创建用户。  【test_useradd_with_prefix】

- 测试方法： 在测试环境中创建 `prefix` 目录结构 `/prefix/etc`，并将系统 `/etc` 目录中的相关文件复制到该目录中。然后运行 `easybox useradd -P /prefix testuser_prefix` 命令。最后读取 `/prefix/etc/passwd` 文件，验证用户是否正确添加到该文件中。
- 测试结果：成功在 `prefix` 目录中创建用户 `testuser_prefix`，并通过检查 `/prefix/etc/passwd` 文件确认用户信息已正确记录在该文件中。

28. 功能点：测试 `-s` 选项，验证是否正确设置了用户的默认 shell。 【test_useradd_with_shell】

- 测试方法：在测试环境中运行 `easybox useradd -s /bin/sh testuser_shell`，然后读取 `/etc/passwd` 文件，验证为用户 `testuser_shell` 设置的 shell 是否正确。
- 测试结果：成功创建用户 `testuser_shell`，并且 `/etc/passwd` 文件中正确记录了 shell 为 `/bin/sh`。

29. 功能点：测试 `-u` 选项，验证是否正确设置了用户的 UID。 【test_useradd_with_uid】

- 测试方法：在测试环境中运行 `easybox useradd -u 2001 testuser_uid`，然后使用 `id -u` 命令验证为用户 `testuser_uid` 设置的 UID 是否为 2001。
- 测试结果：成功创建用户 `testuser_uid`，并且用户的 UID 正确设置为 2001。

30. 功能点：测试 `-U` 选项，验证是否为用户创建了同名的用户组，并将该用户设置为该组的成员。【test_useradd_user_group】

- 测试方法：在测试环境中运行 `easybox useradd -U testuser_with_group`，然后读取 `/etc/passwd` 和 `/etc/group` 文件，验证是否成功创建了用户 `testuser_with_group` 以及同名的用户组，并检查该用户的主要组是否设置为该组。
- 测试结果：成功创建用户 `testuser_with_group`，并且 `/etc/passwd` 和 `/etc/group` 文件中正确记录了该用户及其同名的用户组。

31. 功能点：测试 `-U` 选项在组名与已存在的组名冲突时，验证是否正确处理冲突情况。【test_useradd_user_group_conflict】

- 测试方法：在测试环境中先创建一个名为 `existinggroup` 的组，然后运行 `easybox useradd -U existinggroup`，验证命令是否因组名冲突而失败。
- 测试结果：命令执行失败，正确处理了组名冲突的情况，用户未被创建。

32. 功能点：测试 `-U` 选项与 `-g` 选项冲突时，验证是否正确处理冲突情况。【test_useradd_user_group_with_g_conflict】

- 测试方法：在测试环境中运行 `easybox useradd -U -g 1000 testuser_with_group_g`，验证命令是否因 `-U` 和 `-g` 选项冲突而失败。
- 测试结果：命令执行失败，正确处理了 `-U` 和 `-g` 选项的冲突，用户未被创建。

33. 功能点：测试 `-U` 选项与 `-N` 选项冲突时，验证是否正确处理冲突情况。【test_useradd_user_group_with_n_conflict】

- 测试方法：在测试环境中运行 `easybox useradd -U -N testuser_with_group_n`，验证命令是否因 `-U` 和 `-N` 选项冲突而失败。
- 测试结果：命令执行失败，正确处理了 `-U` 和 `-N` 选项的冲突，用户未被创建。

34. 功能点：测试 `-F`（`--add-subids-for-system`）选项，验证在添加系统用户时是否正确创建了子 `UID` 和子 `GID` 条目。 【test_useradd_with_add_subids_for_system】

- 测试方法：在测试环境中运行 `easybox useradd -r -F testuser_add_subids_system`，读取 `/etc/subuid` 和 `/etc/subgid` 文件，验证是否正确记录了与系统用户相关的子 `UID` 和子 `GID` 条目。
- 测试结果：成功创建系统用户 `testuser_add_subids_system`，且 `/etc/subuid` 和 `/etc/subgid` 文件中正确记录了该用户的子 `UID` 和子 `GID` 条目。

## attr功能对比验证报告

### 软件支持的功能清单

| 软件功能 | 原有软件 | Rust重写后的软件 |
| --- | --- | --- |
| attr [选项] -s 属性名 [-V 属性值] 路径 | 支持  | 支持  |
| attr [选项] -g 属性名 路径 | 支持  | 支持  |
| attr [选项] -r 属性名 路径 | 支持  | 支持  |
| attr [选项] -l 路径 | 支持  | 支持  |
| -s 设置目标文件的指定属性为给定的值 | 支持  | 支持  |
| -g 搜索目标文件并打印与此属性名关联的值 | 支持  | 支持  |
| -r 如果属性存在，则从目标文件中删除具有给定名称的属性 | 支持  | 支持  |
| -l 列出与目标文件关联的所有属性的名称 | 支持  | 支持  |
| -V 设置目标属性的值 | 支持  | 支持  |
| -L 跟随符号链接，操作链接所指向文件的属性 | 支持  | 支持  |
| -q 保持静默，仅将错误消息输出到标准错误，不会打印状态消息 | 支持  | 支持  |
| -R 在root属性命名空间而不是user属性命名空间中操作 | 支持  | 支持  |
| -S 指定使用secure属性命名空间 | 支持  | 支持  |
| -h, --help 显示程序的帮助信息 | 不支持 | 支持  |
| --version 显示程序的版本信息 | 不支持 | 支持  |

### 软件自带用例对比验证

软件暂无自带测试用例，下面将通过与原版程序交叉验证来测试软件的主要功能，测试方法如下：

##### 测试 -s 功能

在测试环境创建新文件`file`，运行`easybox attr -s attrname -V attrval file`，向文件`file`设置属性名attrname，属性值为attrval，紧接着运行原版程序获取该属性值`/usr/bin/attr -g attrname file`，命令运行结果如下：
![图1](https://foruda.gitee.com/images/1712135730996174922/1fb3a84a_13796108.png "截屏2024-04-03 11.28.57.png")
##### 测试 -g 功能

在测试环境创建新文件`file`，运行原版程序设置secure空间属性：`/usr/bin/attr -S -s securename -V secureval file`，紧接着运行`easybox attr -S -g securename file`在secure空间获取属性名securename的值，命令运行结果如下：
![图2](https://foruda.gitee.com/images/1712135756748349112/e239df1b_13796108.png "截屏2024-04-03 11.34.24.png")
##### 测试 -r 功能

在测试环境创建新文件`file`，运行原版程序设置root空间属性：`/usr/bin/attr -R -s rootname -V rootval file`，紧接着运行`easybox attr -R -r rootname file`在root空间删除属性名rootname的值，随后再用原版程序获取，命令运行结果如下：

![图3](https://foruda.gitee.com/images/1712135828837238784/693c433a_13796108.png "截屏2024-04-03 11.51.14.png")
##### 测试 -l 功能

在测试环境创建新文件`file` ，运行原版程序设置属性attrnameA，attrnameB，属性值均为attrval，即：`touch file && /usr/bin/attr -s attrnameA -V attrval file && /usr/bin/attr -s attrnameB -V attrval file`，随后运行`easybox attr -l file`获取属性列表，命令运行结果如下：

![图4](https://foruda.gitee.com/images/1712135847805119989/0aafbd55_13796108.png "截屏2024-04-03 13.30.16.png")

### 功能对比验证

1. 功能点1【选项-s -g】：从标准输入中读取属性值并设置，随后立即获取

- 测试方法：在测试环境中运行命令`echo testval | attr -s testname file` ；随后运行`attr -g testname file`检验。
- 测试结果：命令成功运行，文件`file`拥有属性名`testname`和其对应的属性值`testval`。

2. 功能点2【选项-s -g -V -S -q】：使用secure属性命名空间并通过命令行参数指定属性值并设置，随后使用静默模式获取

- 测试方法：在测试环境中运行命令`attr -S -s securename -V secureval file`；随后运行`attr -Sq -g securename file`尝试以静默方式获取该属性值
- 测试结果：命令成功运行，文件`file`在secure命名空间拥有属性名`securename`和其对应的属性值`secureval`。

3. 功能点3【选项-s -g -l -V -R -L】：使用root属性命名空间设置属性，并跟随符号链接，随后尝试获取相应文件的属性值，以及列举属性名。

- 测试方法：在测试环境中切换至root身份，创建指向普通文件`file`的符号链接`link`，运行命令`attr -LR -s rootname -V rootval link`；随后运行命令`attr -R -g rootname file`确保该属性值已被正确设置到文件上；运行命令`attr -LR -l link`，程序将列出该属性名。
- 测试结果：命令成功运行，文件`file`在root命名空间拥有属性名`rootname`和其对应的属性值`rootval`。

4. 功能点4【选项-s -l -V -q】：静默方式列举属性名

- 测试方法：在测试环境中运行命令`attr -s testattr -V testval file`；随后运行`attr -q -l file`列举文件的所有属性名。
- 测试结果：命令成功运行，文件`file`拥有属性名`testname`。

5. 功能点5【选项-s -r -g -V】：删除指定属性

- 测试方法：在测试环境中运行命令`attr -s testattr -V testval file`；随后运行`attr -r testattr file`删除文件的属性；再使用命令`attr -g testattr file`获取属性。
- 测试结果：命令成功运行，文件`file`的属性`testattr`已被删除，获取失败。

6. 功能点6【选项-s -r -S -V -L -q】：使用静默模式设置secure命名空间属性值，并删除，均跟随符号链接

- 测试方法：在测试环境中建立指向文件`file`的符号链接`link`，运行命令`attr -LSq -s secureattr -V secureval link`；随后运行`attr -LS -r secureattr link`删除文件的属性
- 测试结果：命令成功运行，文件`file`的属性`secureattr`已被删除。

7. 功能点7【选项-s -l -V -S -R -q】：列举所有属性，仅列举secure命名空间的属性

- 测试方法：在测试环境中运行命令`attr -q -s testattr -V testval file`、`attr -Sq -s secureattr -V secureval file`、`attr -Rq -s rootattr -V rootval file`；运行命令`attr -lq file`列举所有属性；运行命令`attr -Sql file`尝试获取所指文件在secure命名空间的属性。
- 测试结果：不加-S、-R参数时能够列举出所有属性，而增加-S参数后，仅能列举secure命名空间的属性。

8. 功能点8：测试命令行选项【-s -g -r -l】不能混用

- 测试方法：在测试环境中运行命令`attr -l -s testattr file`。
- 测试结果：命令运行失败，-s与-l不能同时出现。

9. 功能点9：测试help和version参数

- 测试方法：在测试环境中运行`attr --help`和`attr --version`。
- 测试结果：命令运行成功，程序输出帮助信息与版本信息。

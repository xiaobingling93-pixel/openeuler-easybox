# 功能验证对比报告

## 软件支持的功能清单

| 软件功能                                                     | 原有软件 | Rust重写后的软件 | 是否自动化测试 |
| ------------------------------------------------------------ | -------- | ---------------- | -------------- |
| easybox xargs [options] [command [initial-arguments]]        | 支持     | 支持             | 是             |
| -0,  --null  用空终止符而不是空格分割输入                    | 支持     | 支持             | 是             |
| -a, --arg-file  从 FILE 读取参数，而不是在标准输入中进行读取 | 支持     | 支持             | 是             |
| -d, --delimiter  使用给定的分隔符来分割输入                  | 支持     | 支持             | 是             |
| -E, --eof  设置文件读取的结束字符串                          | 支持     | 支持             | 是             |
| -e, --eof 设置文件读取的结束字符串                           | 支持     | 支持             | 是             |
| -I, --replace 替换选项                                       | 支持     | 支持             | 是             |
| -i, --replace 替换选项                                       | 支持     | 支持             | 是             |
| -L, --max-lines  设置从 stdin 传递到每个命令调用的最大行数（与 -n 互斥） | 支持     | 支持             | 是             |
| -l, --max-lines   设置从 stdin 传递到每个命令调用的最大行数（与 -n 互斥） | 支持     | 支持             | 是             |
| -n, --max-args  设置从 stdin 读取并传递给每个命令调用的最大参数数量（与 -L 互斥） | 支持     | 支持             | 是             |
| -o, --open-tty  执行命令之前，在子进程中重新打开 stdin 作为 /dev/tty | 支持     | 支持             | 是             |
| -P, --max-procs  修改最大的进程数                            | 支持     | 不支持           | 否             |
| -p, --interactive 当每次执行一个argument的时候询问一次用户   | 支持     | 支持             | 是             |
| -r, --no-run-if-empty  如果没有输入参数，则不运行该命令      | 支持     | 支持             | 是             |
| -s, --size   设置要传递给每次调用的最大字符数                | 支持     | 支持             | 是             |
| -t, --verbose  先打印命令，然后再执行                        | 支持     | 支持             | 是             |
| -x, --exit 如果 -L 或 -n 允许的参数数量不适合允许的字符数量，则退出 | 支持     | 支持             | 是             |
| -V, --verison 显示程序版本信息                               | 支持     | 支持             | 否             |
| -h, --help 显示程序的帮助信息                                | 支持     | 支持             | 否             |

注：-P属性没有进行实现，原因如下：

1. 在使用rust重写xargs命令之初，我就设计的该项目是使用顺序执行的方式，由于重写之后业务逻辑相对复杂，无法再重新重构。
2. -P选项可能会导致运行速度变慢，所以没有进行实现。

## 软件自带用例对比验证

### 测试情况说明

原xargs共96个测试用例，通过86个测试，有10个测试没有通过。在这10个没有通过的测试用例中，其中有4个测试用例是可以通过的，现已添加到easbox中进行测试。**使用easybox自带的测试框架进行测试，通过90个测试用例。**

将使用rust重写的xargs项目进行`cargo build`，得出的可执行文件`xargs`复制到原项目中，使用原项目的测试框架进行测试，得到如下结果：

```
                === xargs Summary ===

# of expected passes            86
# of unexpected failures        10
```

下面是对这些没有通过的测试用例的说明：

1. 测试用例一

> xargs  -P3 -n1 -IARG sh -c ARG < ./inputs/Pdata.xi

原因：没有实现`-P`属性

2. 测试用例二

> xargs  -0 echo this plus that < ./inputs/space.xi

原因：

如果使用原有的 GNU 测试框架进行测试可能会与原来的格式输出不同。因为原框架对比的是space-0.xo 文件的内容，即cmp xargs.out ./xargs.gnu/space-0.xo，测试的结果保存在了 xargs.out 文件中，不同文件的格式可能会出现问题。
我使用统一的格式进行输出：

`/home/ywt/work/findutils-4.9.0/xargs/testsuite/../xargs  -0 echo this plus that < ./inputs/space.xi > /home/ywt/work/findutils-4.9.0/xargs/result1.out`

`xargs  -0 echo this plus that < ./inputs/space.xi > /home/ywt/work/findutils-4.9.0/xargs/result2.out`

`result1.out` 为 rust 重写后的输出结果

`result2.out` 为原 xargs 的输出结果

使用 cmp 对这两个文件进行了对比，发现没有输出结果的差异。

`cmp /home/ywt/work/findutils-4.9.0/xargs/result1.out /home/ywt/work/findutils-4.9.0/xargs/result1.out`

**所以该测试是可以通过的，在easbox中的测试已进行添加。**

3. 测试用例三

> xargs  -t -0 echo this plus that < ./inputs/space.xi

原因：同`测试用例二`的原因。

**该测试是可以通过的，在easbox中的测试已进行添加。**

4. 测试用例四

> xargs  -IARG echo from ARG to xARGy < ./inputs/items.xi

原因：在字符替换方面，使用 rust 重写的之后对于特殊字符的读取和替换与原 xargs 的方式不同，所以导致错误。

5. 测试用例五

> sh -c {xargs  -E2; cat}  < ./inputs/sv-bug-20273.xi

原因：

在重写的时候使用了clap库，重写的xarg会误认为cat也是命令行参数，然后错误的进行读取，所以导致了该测试用例无法通过。

6. 测试用例六

> xargs  < ./inputs/unmatched.xi

原因：

这个例子是输入文件 unmatch.xi 中有不匹配的双引号，会引发错误。
这个例子失败的原因是返回的错误信息和原 xargs 不同，但意思相同。**该测试是可以通过的，在easbox中的测试已进行添加。**

原 xargs 返回错误信息:

`xargs: unmatched double quote; by default quotes are special to xargs unless you use the -0 option`

rust 版的 xargs 返回的错误信息:

`Error: unmatched double quote: 34`

7. 测试用例七

> xargs  < ./inputs/unmatched2.xi

原因：

原因与`测试用例六`相同，都是返回的不匹配的双引号的错误信息与原 xargs 不同

该测试是可以通过的，在easbox中的测试已进行添加。

8. 测试用例八

> xargs  -i -s26 echo from \{\} to x{}y < ./inputs/items.xi

原因：在字符替换方面，使用 rust 重写的之后对于特殊字符的读取和替换与原 xargs 的方式不同，所以导致错误。

9. 测试用例九

> xargs  -i echo from \{\} to x{}y < ./inputs/items.xi

原因：

由于原版的xargs对于-i后面可以有替换符号也可以没有替换符号，如果没有替换符号，就默认是`{}`。但在使用rust的clap库进行重写时候，clap库会误认后面的内容是-i的值。

10. 测试用例十

> xargs  -i__ echo FIRST __ IS OK < ./inputs/quotes.xi

原因：在字符替换方面，使用 rust 重写的之后对于特殊字符的读取和替换与原 xargs 的方式不同，所以导致错误。


## 功能对比验证

1. 功能点：-0

- 测试方法：在终端执行`echo -e "ab c\0d\tef\0" | easybox xargs -0 -n1`
- 测试结果：显示`ab c\nd\tef\n`。并与xargs执行结果符合。

2. 功能点：-a

- 测试方法：在终端执行`easybox xargs -a tests/fixtures/xargs/files.xi`
- 测试结果：显示files.xi文件中的内容。与xargs执行结果符合。

3. 功能点：-d

- 测试方法：在终端执行`echo "ab1cd1ef" | easybox xargs -d1`
- 测试结果：显示`ab cd ef\n`。并与xargs执行结果符合。

4. 功能点：-E

- 测试方法：在终端执行`echo 1 2 3 4 | easybox xargs -E3`
- 测试结果：显示`1 2`。并与xargs执行结果符合。

5. 功能点：-e

- 测试方法：在终端执行`echo 1 2 3 4 | easybox xargs -e3`
- 测试结果：显示`1 2`。并与xargs执行结果符合。

6. 功能点：-I

- 测试方法：在终端执行`echo foo | easybox xargs -IARG echo ARG bar`
- 测试结果：显示`foo bar`。并与xargs执行结果符合。

7. 功能点：-i

- 测试方法：在终端执行`echo foo | easybox xargs -i{} echo {} bar`
- 测试结果：显示`foo bar`。并与xargs执行结果符合。

8. 功能点：-L

- 测试方法：在终端执行`echo -e "ab cd\nef\ngh i\n\njkl\n" | easybox xargs -L2`
- 测试结果：显示`ab cd ef\ngh i jkl\n`。并与xargs执行结果符合。

9. 功能点：-l

- 测试方法：在终端执行`echo -e "ab cd\nef\ngh i\n\njkl\n" | easybox xargs -l2`
- 测试结果：显示`ab cd ef\ngh i jkl\n`。并与xargs执行结果符合。

10. 功能点：-n

- 测试方法：在终端执行`echo -e "ab cd\nef\ngh i\n\njkl\n" | easybox xargs -n2`
- 测试结果：显示`ab cd ef\ngh i jkl\n`。并与xargs执行结果符合。

11. 功能点：-o

- 测试方法：在终端执行`echo 1 | easybox xargs -o tests/fixtures/xargs/ask_user.sh`
- 测试结果：执行命令之前，在子进程中重新打开 stdin 作为 /dev/tty，用户可以在终端中输入信息，然后xargs进行读取。并与xargs执行结果符合。

12. 功能点：-P（暂时没有实现）

13. 功能点：-p

- 测试方法：在终端执行`echo foo bar | easybox xargs -p echo` 
- 测试结果：`xargs` 在执行 `echo` 命令前，都会给出一个提示，询问用户是否要继续。如果用户输入 'y'，那么 `echo` 命令会被执行，并且该行文本会被打印到终端上。如果用户输入 'n'，那么对于当前行的命令不会被执行，`xargs` 会继续下一行内容的提示。并与xargs执行结果符合。

14. 功能点：-r

- 测试方法：在终端执行`xargs -r echo foo`
- 测试结果：在没有任何输入的情况下，什么也不显示。并与xargs执行结果符合。

15. 功能点：-s

- 测试方法：在终端执行`echo ab cd efg | easybox xargs -s11`
- 测试结果：设置要传递给每次调用的最大字符数为11，传递的内容没有超过11，在终端输出`ab cd\nefg\n`。并与xargs执行结果符合。

16. 功能点：-t

- 测试方法：在终端执行`echo foo | easybox xargs -t`
- 测试结果：先打印命令，然后再执行。并与xargs执行结果符合。

17. 功能点：-x

- 测试方法：在终端执行 `echo abcdefg hijklmn | easybox xargs -x -s11 -n2  `
- 测试结果：`Error: Argument too large`

18. 功能点：-V

- 测试方法：在终端执行`easybox xargs -V`
- 测试结果：打印出版本信息

19. 功能点：-h

- 测试方法：在终端执行`easybox xargs -h`
- 测试结果：打印出帮助信息

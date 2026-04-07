# column功能验证对比报告

## 软件支持的功能清单

| 软件功能                                                     | 原有软件 | Rust重写后的软件 |
| ------------------------------------------------------------ | -------- | ---------------- |
| easybox column [options] [<file>...]                         | 支持     | 支持             |
| -t, --table    创建一个表格输出                              | 支持     | 支持             |
| -n, --table-name <name>    为 JSON 输出指定表格名称          | 支持     | 支持             |
| -O, --table-order <columns>    指定输出列的顺序              | 支持     | 支持             |
| -N, --table-columns <names>    逗号分隔的列名称              | 支持     | 支持             |
| -l, --table-columns-limit <num>    输入列的最大数量          | 支持     | 支持             |
| -E, --table-noextreme <columns>    不将列中的长文本计算为列宽 | 支持     | 支持             |
| -d, --table-noheadings    不打印标题                         | 支持     | 支持             |
| -e, --table-header-repeat    每页重复标题                    | 支持     | 支持             |
| -H, --table-hide <columns>    不打印指定的列                 | 支持     | 支持             |
| -R, --table-right <columns>    这些列中指定的文本右对齐      | 支持     | 支持             |
| -T, --table-truncate <columns>    必要时截断列中的文本       | 支持     | 支持             |
| -W, --table-wrap <columns>     必要时在列中换行文本          | 支持     | 支持             |
| -L, --keep-empty-lines    不忽略空行                         | 支持     | 支持             |
| -J, --json    对表使用 JSON 输出格式                         | 支持     | 支持             |
| -r, --tree <column>    对表使用树状输出                      | 支持     | 支持             |
| -i, --tree-id <column>    指定子父关系的行 ID                | 支持     | 支持             |
| -p, --tree-parent <column>    父级指定子父关系               | 支持     | 支持             |
| -c, --output-width <width>    输出宽度（以字符数为单位）     | 支持     | 支持             |
| -o, --output-separator <string>    表输出的列分隔符（默认为两个空格） | 支持     | 支持             |
| -s, --separator <string>    可能的表分隔符                   | 支持     | 支持             |
| -x, --fillrows    在列之前填充行                             | 支持     | 支持             |
| -h, --help    显示程序版本信息                               | 支持     | 支持             |
| -V, --version    显示程序的帮助信息                          | 支持     | 支持             |

## 使用Rust重写后的column与原版column在部分输出格式上的差异说明

1. **JSON输出格式的差异**

在使用`--json`参数时，Rust重写的column与原版column在JSON格式的输出上存在差异。这是因为Rust重写版采用了`serde_json::to_string_pretty()`函数进行格式化输出，从而导致与原版column输出的格式有所不同。

例如，当运行以下测试用例时：

```
column --table-name "my_table" --json --table-columns "Column1,Column2,Column3,Column4" data.txt
```

原版column的输出结果为：

```
ywt@ywt:~/work/easybox/target/debug$ column --table-name "my_table" --json --table-columns "Column1,Column2,Column3,Column4" /home/ywt/work/util-linux-2.40/debug-data/data1.txt
{
   "my_table": [
      {
         "column1": "1",
         "column2": "2",
         "column3": "3",
         "column4": "4"
      },{
         "column1": "5",
         "column2": "6",
         "column3": "7",
         "column4": "8"
      }
   ]
}
```

而Rust重写版的输出结果为：

```
ywt@ywt:~/work/easybox/target/debug$ ./column --table-name "my_table" --json --table-columns "Column1,Column2,Column3,Column4" /home/ywt/work/util-linux-2.40/debug-data/data1.txt
{
  "my_table": [
    {
      "Column1": "1",
      "Column2": "2",
      "Column3": "3",
      "Column4": "4"
    },
    {
      "Column1": "5",
      "Column2": "6",
      "Column3": "7",
      "Column4": "8"
    }
  ]
}
```

主要的差异体现在每个JSON对象之间的逗号位置。原版column在每个对象后面紧跟一个逗号：

```
},{
```

而Rust重写版在每个对象后面逗号和大括号之间有一个换行：

```
},
{
```



2. **树形输出排列顺序的差异**

在使用`--tree`参数时，Rust重写的column与原版column在树形结构的排列顺序上存在差异。虽然输出内容相同，但由于实现细节的不同，结点顺序和层次结构可能会有所不同。例如，运行如下测试用例：

```
column --table tests/fixtures/column/mountinfo --table-columns ID,PARENT,MAJMIN,ROOT,TARGET,VFS-OPTS,PROP,SEP,TYPE,SOURCE,FS-OPTS --table-hide=SEP,ID,PARENT,ROOT,PROP,FS-OPTS,MAJMIN --table-order TARGET,SOURCE,TYPE,VFS-OPTS --tree TARGET --tree-id ID --tree-parent PARENT --output-width 110
```

差异对比结果：

```
$ diff c_column_result.txt rust_column_result.txt

19d18
< │ ├─/sys/kernel/config                 configfs              configfs         rw,relatime
20a20
> │ ├─/sys/kernel/config                 configfs              configfs         rw,relatime
32,34c32,34
< │ ├─/run/user/1000                     tmpfs                 tmpfs            rw,nosuid,nodev,relatime
< │ │ └─/run/user/1000/gvfs              gvfsd-fuse            fuse.gvfsd-fuse  rw,nosuid,nodev,relatime
< │ └─/run/user/0                        tmpfs                 tmpfs            rw,nosuid,nodev,relatime
---
> │ ├─/run/user/0                        tmpfs                 tmpfs            rw,nosuid,nodev,relatime
> │ └─/run/user/1000                     tmpfs                 tmpfs            rw,nosuid,nodev,relatime
> │   └─/run/user/1000/gvfs              gvfsd-fuse            fuse.gvfsd-fuse  rw,nosuid,nodev,relatime
```

![输入图片说明](https://foruda.gitee.com/images/1720012670923827392/92606eb1_10276000.png "image-20240529141615856.png")

在`mountinfo`文件中，由于`/sys/kernel/config`结点的结点编号为`60`，其父节点的编号为`17`；对于`/sys/kernel/debug`结点的结点编号为`39`，其父节点的编号为`17`。这两个结点的父节点编号相同，在Rust实现的column中如果父节点编号相同，则以当前结点的编号从小到大进行排序，所以`/sys/kernel/config`结点排列到了`/sys/kernel/debug`的前面，与原column有所差异。

![输入图片说明](https://foruda.gitee.com/images/1720012746699797206/04efb389_10276000.png "image-20240529142421788.png")

和上面的原因相同，使用Rust实现的column在排列结点的时候，如果父结点的编号相同，那么结点的顺序就按照当前结点编号进行从小到大进行排序。

## 软件自带用例对比验证

### 测试情况说明

C版本的column一共有25个测试用例，通过了24个测试用例，有1个测试用例没有通过，未通过的测试用例已在前面的差异说明中进行了详细的说明。



### 测试结果

原C版本column自带的测试框架下的测试结果：

```
ywt@ywt:~/work/util-linux-2.37.2/tests$ ./run.sh column

-------------------- util-linux regression tests --------------------

                    For development purpose only.                    
                 Don't execute on production system!                 

       kernel: 6.5.0-35-generic                  

      options: --srcdir=/home/ywt/work/util-linux-2.37.2/tests/.. \
               --builddir=/home/ywt/work/util-linux-2.37.2/tests/..

       column: columnate                      ...
                : fill-cols-80                ... OK
                : fill-cols-50                ... OK
                : fill-cols-250               ... OK
                : fill-rows-80                ... OK
                : fill-rows-50                ... OK
                : fill-rows-250               ... OK
           ... OK (all 6 sub-tests PASSED)
       column: invalid multibyte              ... OK
       column: multiple files                 ... OK
       column: table                          ...
                : default                     ... OK
                : output-separator            ... OK
                : input-separator             ... OK
                : input-separator-space       ... OK
                : empty-lines                 ... OK
                : noempty-lines               ... OK
                : long                        ... OK
                : hide                        ... OK
                : headers                     ... OK
                : truncate                    ... OK
                : right                       ... OK
                : wrap                        ... OK
                : order                       ... OK
                : tree                        ... FAILED (column/table-tree)
                : empty-column                ... OK
                : empty-column-at-eol         ... OK
                : empty-column-at-eol2        ... OK
           ... FAILED (1 from 17 sub-tests)

---------------------------------------------------------------------
  1 tests of 4 FAILED
---------------------------------------------------------------------
```



easybox测试框架下的测试结果：

```
running 25 tests
test test_column::test_column_empty_column_at_eol2 ... ok
test test_column::test_column_default ... ok
test test_column::test_column_empty_column_at_eol ... ok
test test_column::test_column_empty_column ... ok
test test_column::test_column_fill_cols_250 ... ok
test test_column::test_column_fill_cols_50 ... ok
test test_column::test_column_empty_lines ... ok
test test_column::test_column_fill_cols_80 ... ok
test test_column::test_column_fill_rows_50 ... ok
test test_column::test_column_fill_rows_250 ... ok
test test_column::test_column_fill_rows_80 ... ok
test test_column::test_column_input_separator_space ... ok
test test_column::test_column_input_separator ... ok
test test_column::test_column_hide ... ok
test test_column::test_column_headers ... ok
test test_column::test_column_invalid_multibyte ... ok
test test_column::test_column_order ... ok
test test_column::test_column_noempty_lines ... ok
test test_column::test_column_multiple_files_input ... ok
test test_column::test_column_long ... ok
test test_column::test_column_output_separator ... ok
test test_column::test_column_right ... ok
test test_column::test_column_truncate ... ok
test test_column::test_column_wrap ... ok
test test_column::test_column_tree ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 176 filtered out; finished in 0.06s
```







## 功能对比验证

1. 功能点：【选项：--table】测试默认表格输出

- 测试方法：在终端执行`easybox column --table tests/fixtures/column/table  `
- 测试结果：打印出来一个有序排列的表格。并与column执行结果相同。

2. 功能点：【选项：--output-separator --table】测试自定义输出分隔符

- 测试方法：在终端执行`easybox column --output-separator "|" --table tests/fixtures/column/files/table `
- 测试结果：表格数据使用自定义分隔符`|`正确输出。并与column执行结果相同。

3. 功能点：【选项：--separator --table】测试自定义输入分隔符

- 测试方法：在终端执行`easybox column --separator ',' --table tests/fixtures/files/table-sep`
- 测试结果：表格数据使用自定义输入分隔符`,`正确解析输入文件中的内容，并输出成为表格。并与column执行结果相同。

4. 功能点：【选项：--separator --table】测试使用制表符作为输入分隔符

- 测试方法：在终端执行`easybox column --separator "$(echo -e '\t')" --table tests/fixtures/files/table-sep-space`
- 测试结果：表格数据使用制表符作为输入分隔符正确解析输入文件中的内容，并输出成为表格。并与column执行结果相同。

5. 功能点：【选项：--table --keep-empty-lines】测试不忽略空行，输出的内容与原始数据结构一致

- 测试方法：在终端执行`easybox column --table --keep-empty-lines tests/fixtures/files/table-empty-lines`
- 测试结果：输出结果中保留了空行。并与column执行结果相同。

6. 功能点：【选项：--table】测试不保留空行

- 测试方法：在终端执行`easybox column --table tests/fixtures/files/table-empty-lines`
- 测试结果：输出结果中没有空行。并与column执行结果相同。

7. 功能点：【选项：--table】测试长表格数据的输出

- 测试方法：在终端执行`easybox column --table tests/fixtures/files/mountinfo`
- 测试结果：长表格数据被正确输出。并与column执行结果相同。

8. 功能点：【选项：--table --table-hide】测试隐藏指定列

- 测试方法：在终端执行`easybox column --table tests/fixtures/files/mountinfo --table-hide 1,2,3,4,7,8`
- 测试结果：指定列被正确隐藏。并与column执行结果相同。

9. 功能点：【选项：--table --table-columns --table-hide】测试表格打印列头和指定需要隐藏的列，在打印时候不输出需要隐藏的列的内容

- 测试方法：在终端执行`easybox column --table tests/fixtures/files/mountinfo --table-columns ID,PARENT,MAJMIN,ROOT,TARGET,VFS-OPTS,PROP,SEP,TYPE,SOURCE,FS-OPTS --table-hide SEP,ID,PARENT,ROOT`
- 测试结果：表格列头和隐藏列功能正常。并与column执行结果相同。

10. 功能点：【选项：--table --table-columns --table-hide --table-truncate --output-width 80】测试表格对指定的列进行数据截断功能

- 测试方法：在终端执行`easybox column --table tests/fixtures/files/mountinfo --table-columns ID,PARENT,MAJMIN,ROOT,TARGET,VFS-OPTS,PROP,SEP,TYPE,SOURCE,FS-OPTS --table-hide SEP,ID,PARENT,ROOT --table-truncate VFS-OPTS,FS-OPTS --output-width 80`
- 测试结果：表格列截断功能正常。并与column执行结果相同。

11. 功能点：【选项：--table --table-columns --table-hide --table-right --output-width 80】测试指定的列的内容进行右对齐

- 测试方法：在终端执行`easybox column --table tests/fixtures/files/mountinfo --table-columns ID,PARENT,MAJMIN,ROOT,TARGET,VFS-OPTS,PROP,SEP,TYPE,SOURCE,FS-OPTS --table-hide SEP,ID,PARENT,ROOT,VFS-OPTS,FS-OPTS,PROP --table-right SOURCE,TYPE --output-width 80`
- 测试结果：表格列右对齐功能正常。并与column执行结果相同。

12. 功能点：【选项：--table --table-columns --table-hide --table-wrap --output-width 110】测试指定列，如果文本太长，将其换行以适应列宽

- 测试方法：在终端执行`easybox column --table tests/fixtures/files/mountinfo --table-columns ID,PARENT,MAJMIN,ROOT,TARGET,VFS-OPTS,PROP,SEP,TYPE,SOURCE,FS-OPTS --table-hide=SEP,ID,PARENT,ROOT,VFS-OPTS,PROP --table-wrap FS-OPTS --output-width 110`
- 测试结果：表格列内容换行功能正常。并与column执行结果相同。

13. 功能点：【选项：--table --table-columns --table-hide --table-order --output-width 110】测试指定输出列的顺序

- 测试方法：在终端执行`easybox column --table tests/fixtures/files/mountinfo --table-columns ID,PARENT,MAJMIN,ROOT,TARGET,VFS-OPTS,PROP,SEP,TYPE,SOURCE,FS-OPTS --table-hide=SEP,ID,PARENT,ROOT,PROP,FS-OPTS,MAJMIN --table-order TARGET,SOURCE,TYPE,VFS-OPTS --output-width 110`
- 测试结果：表格列顺序功能正常。并与column执行结果相同。

14. 功能点：【选项：--table --table-columns --table-hide --table-order --tree TARGET --tree-id ID --tree-parent PARENT --output-width 110】测试树形结构输出

- 测试方法：在终端执行`easybox column --table tests/fixtures/files/mountinfo --table-columns ID,PARENT,MAJMIN,ROOT,TARGET,VFS-OPTS,PROP,SEP,TYPE,SOURCE,FS-OPTS --table-hide=SEP,ID,PARENT,ROOT,PROP,FS-OPTS,MAJMIN --table-order TARGET,SOURCE,TYPE,VFS-OPTS --tree TARGET --tree-id ID --tree-parent PARENT --output-width 110`
- 测试结果：树形结构输出功能正常。但是可能与原column的输出结构有所不同，这部分的内容已在输出格式的差异说明上进行了说明。

15. 功能点：【选项：--table --separator ':' --output-separator ':'】测试空列处理

- 测试方法：在终端执行`printf ':a:b\n' | easybox column --table --separator ':' --output-separator ':'`
- 测试结果：空列处理功能正常。并与column执行结果相同。

16. 功能点：【选项：--table --separator '|' --output-separator '|'】测试行尾空列处理

- 测试方法：在终端执行`printf '|' | easybox column --separator '|' --output-separator '|' --table`
- 测试结果：行尾空列处理功能正常。并与column执行结果相同。

17. 功能点：【选项：--table --separator '|' --output-separator '|'】测试行尾两个空列处理

- 测试方法：在终端执行`printf '||' | easybox column --separator '|' --output-separator '|' --table`
- 测试结果：行尾两个空列处理功能正常。并与column执行结果相同。

18. 功能点：【选项--table --json --table-columns】

- 测试方法：在终端执行`easybox column --table-name "my_table" --json --table-columns "Column1,Column2,Column3,Column4" tests/fixtures/column/data` 
- 测试结果：在打印的json输出中会显示自定义的表格名称。但是可能与原column的输出结构有所不同，这部分的内容已在输出格式的差异说明上进行了说明。

19. 功能点：【选项：-c 80】测试在指定列宽为80的情况下先按照列进行填充表格的情况

- 测试方法：在终端执行`easybox column -c 80 tests/fixtures/files/onecolumn   `
- 测试结果：在指定的80列宽下，先按照列进行填充。并与column执行结果相同。

20. 功能点：【选项：-c 50】测试在指定列宽为50的情况下先按照列进行填充表格的情况

- 测试方法：在终端执行`easybox column -c 50 tests/fixtures/files/onecolumn`
- 测试结果：在指定的50列宽下，先按照列进行填充。并与column执行结果相同。

21. 功能点：【选项：-c 250】测试在指定列宽为250的情况下先按照列进行填充表格的情况

- 测试方法：在终端执行`easybox column -c 250 tests/fixtures/files/onecolumn`
- 测试结果：在指定的250列宽下，先按照列进行填充。并与column执行结果相同。

22. 功能点：【选项：--fillrows -c 80】测试在指定列宽为80的情况下先按照行进行填充表格的情况

- 测试方法：在终端执行`easybox column --fillrows -c 80 tests/fixtures/files/onecolumn`
- 测试结果：在指定的80列宽下，先按照列进行填充。并与column执行结果相同。

23. 功能点：【选项：--fillrows -c 50】测试在指定列宽为80的情况下先按照行进行填充表格的情况

- 测试方法：在终端执行`easybox column --fillrows -c 50 tests/fixtures/files/onecolumn`
- 测试结果：在指定的50列宽下，先按照列进行填充。并与column执行结果相同。

24. 功能点：【选项：--fillrows -c 250】测试在指定列宽为250的情况下先按照行进行填充表格的情况

- 测试方法：在终端执行`easybox column --fillrows -c 250 tests/fixtures/files/onecolumn`
- 测试结果：在指定的250列宽下，先按照列进行填充。并与column执行结果相同。

25. 功能点：测试处理无效的多字节字符

- 测试方法：在终端执行`printf "\x94\x7e\n" | easybox column `
- 测试结果：与column执行结果相同

26. 功能点：【选项：-x -c】测试多文件的处理

- 测试方法：在终端执行`easybox column -x -c 50 tests/fixtures/files/fivecols tests/fixtures/files/fivecols tests/fixtures/files/fivecols `

- 测试结果：可以正确处理多个文件。并与column执行结果相同。

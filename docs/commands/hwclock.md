### 软件支持的功能清单

对于x86，x86_64，ARM，ARM64以及Risc-V架构的处理器（DEC Alpha的机器rust不支持，因此DEC Alpha的选项 `--get-epoch` ，`--set-epoch`，`--epoch`并未列出）：

| 软件功能                                                     | 原有软件 | Rust重写后的软件 |
| ------------------------------------------------------------ | -------- | ---------------- |
| hwclock [功能] [选项...]                                     | 支持     | 支持             |
| -r, --show 查看原硬件时间                                    | 支持     | 支持             |
| --get 获取校准偏移后的硬件时间                               | 支持     | 支持             |
| --set 设置硬件时间为--date所指示的时间                       | 支持     | 支持             |
| -w, --systohc 将硬件时钟设置为系统时间                       | 支持     | 支持             |
| -s, --hctosys 将系统时间设置为硬件时间                       | 支持     | 支持             |
| --systz 设置Linux内核的时区                                  | 支持     | 支持             |
| --adjust 校准硬件时间                                        | 支持     | 支持             |
| --param-get <参数名或者ID> 读取RTC参数                       | 支持     | 支持             |
| --param-set <参数名或者ID> <参数值> 设置RTC参数              | 支持     | 支持             |
| --vl-read 读取低电压记录                                     | 支持     | 支持             |
| --vl-clear 清除低电压记录                                    | 支持     | 支持             |
| --predict 预测当系统时间为--date所指示时的硬件时间           | 支持     | 支持             |
| --update-drift 计算硬件时钟偏移量，并在调整配置文件（通常为`/etc/adjtime`）中更新偏移系数 | 支持     | 支持             |
| -f, --rtc <RTC设备路径>                                      | 支持     | 支持             |
| -u, --utc 忽略调整配置文件所指示的时区，RTC时区为UTC         | 支持     | 支持             |
| -l, --localtime 忽略调整配置文件所指示的时区，RTC时区为当地时间 | 支持     | 支持             |
| --directisa 使用处理器指令而不是设备文件访问RTC              | 支持     | 支持             |
| --date <时间> 日期时间                                       | 支持     | 支持             |
| --delay <秒数> 使用该秒数作为RTC延迟                         | 支持     | 支持             |
| --noadjfile 不使用调整配置文件                               | 支持     | 支持             |
| --adjfile <文件路径> 使用该文件路径作为调整配置文件          | 支持     | 支持             |
| --test 测试模式，不作修改                                    | 支持     | 支持             |
| -v, --verbose 打印调试信息                                   | 支持     | 支持             |
| -D, --debug 打印调试信息，在util-linux中已废弃，应该使用--verbose | 支持     | 支持             |
| -h, --help 输出帮助信息                                      | 支持     | 支持             |
| -V, --version 输出版本信息                                   | 支持     | 支持             |

### 软件自带用例对比验证

以下为部分软件自带测试用例，对于方便直接判断正确性的功能，测试方法即：

#### 测试--show功能

在测试环境中打印出硬件时间，将其转换为微秒时间戳后与从Linux内核`/sys/class/rtc/rtc0`读取到的RTC时间进行对比，如果误差在+-1秒内，即说明测试成功

> 由于RTC硬件上报的时间间隔是以秒为单位，所以有可能比如`/sys/class/rtc/rtc0`为10:20:30时，再过几毫秒RTC就变为10:20:31，随后hwclock再进行测试时会出现二者秒数不同的情况，故设置一秒的容忍度
> 此外，由于在部分系统上，例如QEMU虚拟机中，从`/sys/class/rtc/rtc0`读取的时间甚至有晚于后来hwclock测试时得到的时间的情况（原因未知，可能是QEMU虚拟RTC硬件的问题），所以在参考时间还需要向下设置大约100毫秒的容忍度

#### 测试--get功能

在测试环境中获取到硬件时间，通过已设置偏移系数的配置文件`tests/fixtures/hwclock/adjust_drift`计算偏移量，打印纠正后的硬件时间，将其转换为微秒时间戳，与按照公式计算得到的纠正后的硬件时间对比，误差允许在+-1秒范围内，即说明测试成功

#### 测试--set功能

传入`--date`为`2001-11-20 10:20:30`，在测试环境中设置硬件时间为该时间，随后再从`/sys/class/rtc/rtc0`中获取修改后的硬件时间，将其与`2001-11-20 10:20:30`进行对比，在+-1秒误差允许的范围内，即说明测试成功

> 不同的RTC硬件，RTC设计的延迟不同，例如在x86_64机器上常见的MC146818A，其延迟为0.5秒，也就是对RTC进行ioctl设置时间时，RTC实际上将其设置为后0.5秒，这样hwclock程序就需要在设置之前延迟0.5秒

> 如果忽略这个延迟时间直接测量，很可能会测试失败，即使这不是程序的问题，因此在测试中还需要对程序执行时间进行计算，然后让目标时间加上这个时间并减去设计的延迟时间，才是需要的时间结果

#### 测试--systohc功能

记录最初的硬件时间，传入`--date`为`2001-11-20 10:20:30`，在测试环境中设置硬件时间为该时间，此时硬件时钟已经被设置为`2001-11-20 10:20:30`，再次运行并传入`--systohc`，随后从Linux内核获取时间，与最初记录下来的硬件时间对比，在+-1秒误差允许的范围内，即说明测试成功

#### 测试--hctosys功能

传入`--date`为`2001-11-20 10:20:30`，在测试环境中设置硬件时间为该时间，此时硬件时钟已经被设置为`2001-11-20 10:20:30`，再次运行并传入`--hctosys`，随后获取系统时间，与`2001-11-20 10:20:30`对比，在+-1秒误差允许的范围内，即说明测试成功

#### 测试--localtime功能

传入`--localtime`参数，在测试环境中打印出将RTC时区视为当地时区的硬件时间，并将其与`/sys/class/rtc/rtc0/date`和`/sys/class/rtc/rtc0/time`中Linux内核获取到的时间计算后得到的带有当地时区的时间进行对比，在+-1秒误差允许的范围内，即说明测试成功

#### 测试--adjfile功能

配置文件`adjtime_local`设置时区为当地时区，在测试环境中打印出将RTC时区视为当地时区的硬件时间，并将其与`/sys/class/rtc/rtc0/date`和`/sys/class/rtc/rtc0/time`中Linux内核获取到的时间计算后进行对比，在+-1秒误差允许的范围内，即说明测试成功

在测试环境中获取到硬件时间，通过已设置偏移系数的配置文件`adjtime_drift_factor`计算偏移量，打印纠正后的硬件时间，与按照公式计算得到的纠正后的硬件时间对比，在+-1秒误差允许的范围内，即说明测试成功

#### 测试--noadjfile功能

```
--noadjfile`选项指明时，需要指示`--utc`或者`--localtime
```

在测试环境中传入`--noadjfile --utc`获取到硬件时间，将硬件时钟视为UTC，并将从`/sys/class/rtc/rtc0`中获得的时间也视为UTC，进行时区变换后，两者一致

在测试环境中传入`--noadjfile --localtime`获取到硬件时间，将硬件时钟视为LOCALTIME，并将从`/sys/class/rtc/rtc0`中获得的时间也视为LOCALTIME，进行时区变换后，两者一致

#### 测试--delay功能

传入`--systohc --delay 3`参数，随后获取当前系统时间和硬件时间，如果硬件时间加上程序执行的时间再减去3秒延迟时间与系统时间误差在+-1秒内，并且测得程序延迟时间大于三秒，即说明测试成功

#### 测试--rtc功能

传入参数`--show --rtc /dev/rtc0`，与测试`--show`的方法一样，在误差+-1秒内，即说明测试成功

#### 测试--predict功能

传入参数 `--predict --date "2001-10-10 10:20:30" --adjfile adjtime_drift`，由于--predict功能只和adjfile以及--date有关，则输出为`2001-10-10 14:11:24.522118 +08:00\n`，即证明测试成功

传入参数 `--predict --date "2018-10-10 10:20:30" --adjfile adjtime_drift`，由于--predict功能只和adjfile以及--date有关，则输出为`2018-10-09 16:58:33.822119 +08:00\n`，即证明测试成功

> `2001-10-10 14:11:24.522118 +08:00`和`2018-10-09 16:58:33.822119 +08:00` 这个时间是通过util-linux的hwclock以及按照定义计算得到的

#### 测试--directisa功能

DirectISA即直接通过汇编指令（通常是in/out）访问RTC，目前只有x86系列的处理器能使用这个功能

测试`--show`，`--set`两个参数，不同的是，对于每次测试都额外传入`--directisa`参数，对比验证

#### 测试--update-drift功能

> drift_factor为RTC每天偏差的秒数，如果RTC比标准时间慢，则这个偏移量就为正数

--update-drift为将计算得到的RTC偏移量写入adjfile中

在测试环境中传入`--set --date "2020-11-23 10:20:30" --update-drift --adjfile adjtime_to_update`等参数，由于`2020-11-23 10:20:30`这个时间比当前时间早，因此计算得到的偏移系数绝对为负数

但当这个时间与当前差别非常远时，hwclock会认为RTC报告的时间有错误，将其忽略，此时偏移系数为0

由于`adjtime_to_update`测试文件中`drift_factor`为正数，所以写入新的偏移系数后，如果读取到偏移系数为非正数，则测试成功

#### 测试--adjust参数

在测试环境中传入`--adjust --adjfile adjtime_to_update`等参数，hwclock读取当前硬件时间，并读取adjfile，进行校正后写入校准后的硬件时间，并写入校准时间到adjfile。测试中检测到测试文件被写入了当前时间的时间戳，且RTC被设置为校准后的时间，则测试成功

> **各校准RTC功能的区别**
>
> --set相信偏移系数和用户指定的--date就是正确时间，--systohc相信系统时间就是正确时间
>
> --update-drift不相信adjfile中的偏移系数，也不相信硬件时间，需要从用户或者系统指定的正确时间重新计算偏移系数，因此--update-drift需要--set以及--systohc
>
> --adjust相信adjfile中的偏移系数，但不相信硬件时间，adjust功能会读取偏移系数，读取RTC并设置RTC为校准后的时间
>
> --hctosys相信硬件时间正确

#### 测试--test功能

对RTC进行设置、更新drift_factor和adjust操作，分别检测操作后是否变化，均未变化，则测试正确

#### 测试--date多种时间格式的支持度

传入不同格式的--date，检查命令执行情况是否报告invalid argument for --date

测试结果

![test](https://foruda.gitee.com/images/1701255938333276040/fd35a69d_13268368.png)

### 功能对比验证

1. 功能点1：--show

- 测试方法：在终端中执行`cat /sys/class/rtc/rtc0/date; cat /sys/class/rtc/rtc0/time; sudo easybox hwclock --show`
- 测试结果：显示 `2023-11-29\n10:25:13\n2023-11-29 18:25:13.180718 +08:00\n`。并与hwclock执行结果符合

1. 功能点2：--get

- 测试方法：在终端中执行`sudo easybox hwclock --get --adjfile tests/fixtures/adjtime_drift`
- 测试结果：显示`2023-11-30 18:10:05.007882 +08:00`，与计算得到的相同。并与hwclock执行结果符合

1. 功能点3：--set

- 测试方法：在终端中执行`sudo easybox hwclock --set --date "2020-10-20 10:20:30"; cat /sys/class/rtc/rtc0/date; cat /sys/class/rtc/rtc0/time`
- 测试结果：显示`2020-10-20\n02:20:30`。并与hwclock执行结果符合

1. 测试点4：--systohc

- 测试方法：在终端执行`sudo easybox hwclock --systohc; cat /sys/class/rtc/rtc0/date; cat /sys/class/rtc/rtc0/time`
- 测试结果：显示`2023-11-29\n10:33:13`。并与hwclock执行结果符合

1. 测试点5：--hctosys

- 测试方法：设置RTC为非当前时间，在终端执行`sudo easybox hwclock --hctosys"; date`
- 测试结果：显示`Wed Oct 12 06:34:53 PM CST 2001`。并与hwclock执行结果符合

1. 测试点6：--systz

- 测试方法：在C程序中调用gettimeofday(NULL, &tz)，tz->minutewest为-80（东一区），在终端执行`sudo easybox hwclock --systz`
- 测试结果：再次在C程序中调用gettimeofday(NULL, &tz)，tz->minutewest为-480（东八区），测试成功。并与hwclock执行结果符合

1. 测试点7：--directisa

- 测试方法：在终端执行`sudo easybox hwclock --directisa --verbose`
- 测试结果：输出的调试信息含有`Using direct ISA access to the clock`。并与hwclock执行结果符合

1. 测试点8：--adjust

- 测试方法：在终端执行`sudo easybox hwclock --adjust --adjfile tests/fixtures/hwclock/adjtime_drift --verbose`
- 测试结果：输出调试信息中含有读取的RTC时间、设置的校准时间以及新的adjfile值。并与hwclock执行结果符合

1. 测试点9：--update-drift

- 测试方法：在终端执行`sudo easybox hwclock --update-drift --adjfile tests/fixtures/hwclock/adjtime_to_update`
- 测试结果：在tests/fixtures/hwclock/adjtime_to_update文件中更新了新的偏移系数，并与hwclock执行结果符合

1. 测试点10：--test

- 测试方法：在终端中执行`sudo easybox hwclock --set --date "2020-10-20 10:20:30" --test`
- 测试结果：RTC时间并未改变。并与hwclock执行结果符合

1. 测试点11：--verbose

- 测试方法：在终端中执行`sudo easybox hwclock --verbose`
- 测试结果：输出调试信息

1. 测试点12：--debug

- 测试方法：在终端中执行`sudo easybox hwclock --debug`
- 测试结果：打印出`hwclock: use --verbose, --debug has been deprecated. \n2023-11-29 18:46:42.954205 +08:00`

1. 测试点13：--help

- 测试方法：在终端中执行`sudo easybox hwclock --help`
- 测试结果：打印出帮助信息

1. 测试点14：--version

- 测试方法：在终端中执行`sudo easybox hwclock --version`
- 测试结果：打印出版本信息

1. 测试点15：--param-get

- 测试方法：在终端中执行`sudo easybox hwclock --param-get features`
- 测试结果：输出`The RTC parameter 0x0 is set to 0x11`，与hwclock执行结果符合

1. 测试点17：--param-set

- 测试方法：在终端中执行`sudo easybox hwclock --param-set features 0x09`
- 测试结果：执行`sudo easybox hwclock --param-get features`后，输出`The RTC parameter 0x0 is set to 0x09`，与hwclock执行结果符合

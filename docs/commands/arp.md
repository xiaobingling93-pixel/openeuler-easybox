# arp 命令

## 概述

arp 命令用于操作或显示系统的 ARP（Address Resolution Protocol）缓存。它可以添加、删除或显示 ARP 表中的条目。ARP 协议用于将 IPv4 地址映射到网络邻居的 MAC 地址。

## 支持的参数

| 参数 | 简写 | 说明 | 状态 |
|------|------|------|------|
| --all | -a | 以 BSD 风格显示所有 ARP 条目 | ✓ |
| --linux-style | -e | 以 Linux 风格（默认）显示所有 ARP 条目 | ✓ |
| --set | -s | 添加新的 ARP 条目 | ✓ |
| --delete | -d | 删除指定的 ARP 条目 | ✓ |
| --verbose | -v | 输出详细信息 | ✓ |
| --numeric | -n | 不解析主机名，显示数字地址 | ✓ |
| --device | -i | 指定网络接口（如 eth0） | ✓ |
| --use-device | -D | 从给定设备读取硬件地址 | ✓ |
| --protocol | -p, -A | 指定协议族（默认 inet） | ✓ |
| --file | -f | 从文件读取 ARP 条目（默认 /etc/ethers） | ✓ |
| --hw-type | -H, -t | 指定硬件地址类型（默认 ether） | ✓ |
| --help | -h | 显示帮助信息 | ✓ |
| --version | -V | 显示版本信息 | ✓ |

### 协议族支持

- ✓ inet (DARPA Internet) - 完全支持

### 硬件地址类型支持

- ✓ ether (Ethernet) - 完全支持并测试
- ✓ ash (Ash) - 支持
- ✓ fddi (Fiber Distributed Data Interface) - 支持（未测试）
- ✓ hippi (HIPPI) - 支持（未测试）
- ✓ ax25 (AMPR AX.25) - 支持
- ✓ rose (AMPR ROSE) - 支持（未测试）
- ✓ netrom (AMPR NET/ROM) - 支持
- ✓ x25 (generic X.25) - 支持
- ✓ arcnet (ARCnet) - 支持（未测试）
- ✓ dlci (Frame Relay DLCI) - 支持（未测试）
- ✓ irda (IrLAP) - 支持（未测试）
- ✓ infiniband (InfiniBand) - 支持（未测试）
- ✓ eui64 (Generic EUI-64) - 支持（未测试）

## 使用示例

### 基本用法

显示当前 ARP 缓存：

```bash
easybox arp
```

### BSD 风格显示

以 BSD 风格显示所有 ARP 条目：

```bash
easybox arp -a
```

### Linux 风格显示

以 Linux 风格（默认）显示 ARP 条目：

```bash
easybox arp -e
```

### 显示数字地址

不解析主机名，直接显示 IP 地址：

```bash
easybox arp -n
```

### 详细输出

显示详细的 ARP 信息：

```bash
easybox arp -v
```

### 添加 ARP 条目

添加一个静态 ARP 条目：

```bash
easybox arp -i eth0 -s 192.168.1.100 00:11:22:33:44:55
```

### 从设备读取硬件地址

使用指定接口的 MAC 地址添加 ARP 条目：

```bash
easybox arp -i eth0 -Ds 192.168.1.100 eth1
```

### 删除 ARP 条目

删除指定的 ARP 条目：

```bash
easybox arp -d 192.168.1.100
```

### 从文件添加条目

从文件读取并添加 ARP 条目：

```bash
easybox arp -f /etc/ethers
```

文件格式示例：
```
192.168.1.100 00:11:22:33:44:55
192.168.1.101 00:11:22:33:44:56 pub
```

### 指定硬件类型

指定硬件地址类型：

```bash
easybox arp -H ether
```

## 测试用例情况

| 测试类型 | 数量 | 通过 | 跳过 | 说明 |
|----------|------|------|------|------|
| 基本显示功能 | 3 | 3 | 0 | 测试默认、BSD、Linux 风格显示 |
| 参数组合测试 | 9 | 9 | 0 | 测试 -a, -e, -v, -n 参数组合 |
| 添加/删除条目 | 3 | 3 | 0 | 需要 root 权限和网络能力 |
| 文件读取功能 | 1 | 1 | 0 | 测试从文件添加 ARP 条目 |
| 设备地址读取 | 1 | 1 | 0 | 测试 -D 参数 |

### 测试环境说明

测试用例考虑了不同执行环境的差异：

1. **物理服务器**：所有测试均可执行
2. **虚拟机**：所有测试均可执行
3. **容器内**：需要 CAP_NET_ADMIN 权限，否则跳过部分测试

测试代码会自动检测环境能力：
- 检查是否有 root 权限
- 检查是否有 CAP_NET_ADMIN 能力
- 在容器内自动跳过受限测试

## 已知限制

1. **权限要求**：
   - 添加（-s）和删除（-d）ARP 条目需要 root 权限或 CAP_NET_ADMIN 能力
   - 在容器内执行可能受限，需要适当的权限配置

2. **协议支持**：
   - 内核仅支持 'inet' 协议族
   - 其他协议族（如 inet6）不被内核支持

3. **硬件类型**：
   - 部分硬件类型（如 fddi, hippi, arcnet 等）已实现但未经测试
   - 这些类型在实际环境中较少使用

4. **-N 参数**：
   - --symbolic (-N) 参数尚未实现
   - 使用时会提示 "arp: -N not yet supported."

## 与原命令差异

### 完全兼容的功能

所有主要功能与 openEuler 上的原 arp 命令完全兼容：

1. **参数名称**：所有参数名称与原命令完全一致
2. **使用方式**：命令使用方式与原命令保持一致
3. **输出格式**：输出格式与原命令一致
4. **错误处理**：错误信息和退出码与原命令一致

### 实现说明

1. **Unsafe 代码**：
   - 使用 unsafe 代码进行系统调用（ioctl）和内存操作
   - 所有 unsafe 代码块都有详细的 SAFETY 注释
   - 说明安全性保证和使用条件

2. **内存安全**：
   - 优先使用 Safe Rust 实现功能
   - 仅在与内核交互时使用 unsafe 代码
   - 所有 unsafe 操作都经过仔细审查

3. **错误处理**：
   - 使用 UResult 和 USimpleError 进行错误处理
   - 错误信息与原命令保持一致
   - 正确设置退出码

## 代码质量

### Unsafe 代码审查

所有 unsafe 代码块都包含以下内容：

1. **函数级文档**：说明函数用途和安全性保证
2. **SAFETY 注释**：详细说明为什么 unsafe 代码是安全的
3. **前置条件**：说明调用者需要满足的条件
4. **安全性保证**：列出所有安全性保证措施

### 测试覆盖

测试用例覆盖所有主要功能：

1. **功能测试**：测试每个参数和参数组合
2. **对比测试**：与原命令输出进行对比
3. **边界测试**：测试错误处理和边界条件
4. **环境测试**：考虑不同执行环境的差异

## 参考资源

- ARP 协议：RFC 826
- Linux arp 命令手册：man arp
- Linux 内核 ARP 实现：/proc/net/arp

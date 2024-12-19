---
title: 2024秋冬季开源操作系统训练营项目1方向2总结-shj
date: 2024-12-20 20:20:20
categories:
    - 2024秋冬季开源操作系统训练营项目1方向2总结
tags:
    - author: Foreverhighness
    - repo: https://github.com/Foreverhighness/igb-driver
---

## 碎碎念

因为之前在忙别的事情，所以晚了一周进组，写代码更是只剩两天时间，所以对代码基本是能跑就行的态度。

不过也有好处，因为晚开始所以资料会比一开始弄多点，开始写的时候就基本能 link up 了。

## 找参考资料

最重要的参考资料自然是 Intel 82576 手册。不过如果有代码参考肯定是更好的。

ArceOS 自己项目里面就有一个 ixgbe 的驱动，虽然是不同型号的网卡，但是部分逻辑可以参考，而且是 Rust 编写，很好理解。

其次就是 igb 官方驱动，在 github 上可以找到，参考资料里也给了链接。

我简单瞟了两眼，里面感觉到这估计在 linux kernel 源码里也有一份，一搜果然有。

正好我机器上有一份之前用来学习的 linux 源码，配置一下正好可以看看。

## Linux src

把 CONFIG_IGB 开起来，编译一下再 gen_compile_commands.py 生成下 compile_commands.json 就可以愉快的跳转了。

驱动初始化代码在 `__igb_open` 里，感觉把这玩意实现了应该就可以了。

为了方便实现，我直接跳过了 Flow Control 的部分，感觉应该不会有太大问题。


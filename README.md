# 江科大 B 站 STM32 入门课程代码的 Rust 实现

这个是江科大 B 站 STM32 入门课程代码的 Rust 实现，课程链接：[STM32 入门教程](https://www.bilibili.com/video/BV1th411z7sn/)

## Tools

```bash
mise install rust@stable
# A decoding library for the ARM Cortex-M ITM/DWT packet protocol
cargo install itm
brew install arm-none-eabi-gdb
brew install minicom openocd
cargo install cargo-binutils
rustup component add llvm-tools
```

Install tools from [Rust Code Template Recommendation](https://github.com/tyr-rust-bootcamp/template)

## 流程

### 连接 ST-Link 和开发板



### 编译并加载到开发板

```bash
openocd -f interface/stlink.cfg -f target/stm32f1x.cfg
openocd -f interface/stlink-v3.cfg -f target/stm32f1x.cfg
```

另外增加 openocd 的配置文件 `openocd.gdb`, 在 `.cargo/config.toml` 中指定 runner 为 `arm-none-eabi-gdb -q -x ./openocd.gdb`

## 疑难杂症

### openocd 报 target stm32f1x.cpu examination failed

在 openocd 建立连接的时候一直按着 reset 按钮可解决此问题

## 参考

* [Rust code template](https://github.com/tyr-rust-bootcamp/template)
* [Rust Embedded Discovery](https://github.com/rust-embedded/discovery)
* [STM32F1xx HAL](https://github.com/stm32-rs/stm32f1xx-hal)

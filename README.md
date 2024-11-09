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
rustup target install thumbv7m-none-eabi
cargo binstall probe-rs-tools
```

Install tools from [Rust Code Template Recommendation](https://github.com/tyr-rust-bootcamp/template)

## 流程

### 连接 ST-Link 和开发板

### 编译并加载到开发板

```bash
cargo run --example 3-1-led
```

## 参考

* [Rust code template](https://github.com/tyr-rust-bootcamp/template)
* [Rust Embedded Discovery](https://github.com/rust-embedded/discovery)
* [STM32F1xx HAL](https://github.com/stm32-rs/stm32f1xx-hal)

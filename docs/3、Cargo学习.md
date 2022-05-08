
- ## Cargo教程
	
	- [官方文档地址](https://doc.rust-lang.org/stable/cargo/index.html)：https://doc.rust-lang.org/stable/cargo/index.html
	- [官网中文文档](http://llever.com/cargo-book-zh/index.zh.html )：http://llever.com/cargo-book-zh/index.zh.html 
- ## Cargo的作用

  Cargo是Rust项目的构建工具以及包管理器，可以构建代码、下载依赖的库、构建库等等。


- ## Cargo命令

1. **cargo --version**
2. **cargo new project-name**：使用cargo创建项目
3. **cargo check**：编译检查项目是否有错误，确保代码能通过编译，并不会产生任何可执行程序；check比build快得多，可以提高效率
4. **cargo run**：构建并运行项目
5. **cargo build & cargo build -release**：打包构建项目，添加-release是为发布而构建，编译时会进行优化，代码会运行的更快，但是编译时间更长
6. **cargo clean**：清除已编译文件
7. **cargo install**：从crates.io安装二进制的crate，安装的二进制包放在根目录下的bin文件夹（默认放在$HOME/.cargo/bin）
8. **cargo fmt**：格式化代码
9. **cargo test**：测试代码
10. **cargo publish** ：发布一个library到crates.io上（需要先注册crates账号）

- ## 两个重要的网站

1. https://crates.io/：包托管平台，如果你想分享你的crate给别人使用，可以发布到crates.io

2. https://docs.rs/：Rust官方提供的crates的开源文档网站，发布到[crates.io](https://crates.io/)的所有库 都已记录在案，Docs.rs 的源代码可在 [GitHub](https://github.com/rust-lang/docs.rs)上找到




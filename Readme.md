# Village内核特点:
- ### 1.使用rust编写，内存安全，运行稳定。
- ### 2.支持运行elf，hex，bin格式的PIE程序。

---
# 工程目录：
- ### village_boot:     启动引导
- ### village_kernel:   系统内核
- ### village_osbend:   系统骨架
- ### village_resource: 资源文件

---
# 搭建开发i686平台环境：

- ## 1.系统要求
		mac os / linux / windows（使用wsl子系统）

- ## 2.搭建开发环境, 以mac os为例
	## 安装vscode, git
		安装简单，跳过。安装完成之后打开vscode，安装rust-analyzer，CodeLLDB拓展插件，调试代码需要。

    ### 安装rust工具
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        rustup toolchain install nightly
        rustup default nightly
        cargo install cargo-make

	### 安装homebrew
		/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

	### 安装交叉编译工具
		brew install make

	### 安装qemu模拟器
		使用brew正常安装：
		brew install qemu

		如果出现too many open file错误时输入：
		ulimit -n 4096

- ## 3.克隆village-kernel项目
	- ### ssh方式：
			git clone git@github.com:village-kernel/village-rust.git
  
  	- ### https方式：
			git clone https://github.com/village-kernel/village-rust.git

- ## 4.使用vscode打开village-kernel项目
		编译项目
		make

- ## 5.创建rootfs文件系统镜像
        切换到vscode终端，拷贝文件系统镜像
        cp village-resource/rootfs.img rootfs.img

        右键选中rootfs.img，在Finder中打开，双击rootfs.img文件完成挂载

        拷贝相关文件到文件系统
        make rootfs

- ## 6.运行与调试代码
		切换到vscode debug界面
		选择QEMU Debug x86_64 kernel

# 许可证
- 本项目基于 GNU 通用公共许可证 v3.0 授权 —— 详情请见 [LICENSE](LICENSE) 文件。

# Village内核特点:
- ### 1.上层功能代码与底层驱动代码分离，可移植。
- ### 2.支持模块化，可裁剪，代码模块可分离
- ### 3.支持多线程，多任务
- ### 4.可动态加载模块，类似linux的insmod，rmmod
- ### 5.可运行应用程序，命令行run appname.exec
- ### 6.运行app时会根据编译时链接的动态库，进行加载dyliib文件

# Village内核目标：
- ### 1.可在低端的嵌入式设备运行，也可以在高端的PC运行
- ### 2.让嵌入式开发者花费更少精力在底层，有更多精力搞好应用
- ### 3.适配更多通用设备，让开发者更快实现业务
- ### 4.不为项目更换MCU，需要重新适配底层而烦恼

---
# 目录：
- ## village_docs:    相关文档
- ## village_boot:    启动引导
- ## village_kernel:  系统内核
- ## village_osbend:  系统骨架
- ## village_scripts: 工具脚本

---
# 搭建开发i686平台环境：

- ## 1.系统要求
		mac os / linux / windows（使用wsl子系统）

- ## 2.搭建开发环境, 以mac os为例 (Linux一样可以ubuntu22.04测试过)
	## 安装vscode, git
		安装简单，跳过。安装完成之后打开vscode，安装C/C++拓展插件，调试代码需要。

	### 安装homebrew
		/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

    ### 安装rust
        brew install rustup-init
        rustup-init

	### 安装交叉编译工具
		brew install make gcc i686-elf-binutils i686-elf-gcc i386-elf-gdb

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
		把项目目录village-kernel拉到vscode界面
		
		接着打开vscode终端，拷贝配置文件
		cp village-scripts/configs/i686.config .config
		
		修改配置
		make menuconfig
		进入Compiler选项
		
		配置宿主机编译器：
		() host compile prefix
		(-13) host compile suffix
		
		配置交叉编译器：
		(i686-elf-) cross compile prefix
		() cross compile suffix

		编译项目
		make

- ## 5.创建rootfs文件系统镜像
	- ### Mac OS
			切换到vscode终端，拷贝文件系统镜像
			cp village-scripts/rootfs.img rootfs.img

			右键选中rootfs.img，在Finder中打开，双击rootfs.img文件完成挂载

			修改rootfs文件系统挂载路径
			make menuconfig

			进入Compiler选项
			(/Volumes/VILLAGE OS) rootfs path

			拷贝相关文件到文件系统
			make rootfs

	- ### Linux
			切换到vscode终端，拷贝文件系统镜像
			cp village-scripts/rootfs.img rootfs.img

			终端挂载rootfs.img
			sudo mount -o offset=512 rootfs.img /mnt

			修改rootfs文件系统挂载路径
			make menuconfig

			进入Compiler选项
			(/mnt) rootfs path

			拷贝相关文件到文件系统
			sudo make rootfs


- ## 6.运行与调试代码
		切换到vscode debug界面
		选择QEMU Debug x86_64 kernel

# 搭建开发cortex-m平台环境：

- ## 1.系统要求
		mac os / linux / windows（使用wsl子系统）

- ## 2.搭建开发环境, 以mac os为例 (Linux一样可以ubuntu22.04测试过)
	## 安装vscode, git
		安装简单，跳过。安装完成之后打开vscode，安装C/C++, Cortex-Debug拓展插件，调试代码需要。

	### 安装homebrew
		/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

    ### 安装rust
        brew install rustup-init
        rustup-init

	### 安装交叉编译工具

    #### ~~交叉的编译工具不带newlib库~~
        brew install make openocd minicom gcc arm-none-eabi-binutils arm-none-eabi-gcc arm-none-eabi-gdb
    #### 交叉的编译工具带newlib库
        brew install make openocd minicom gcc gcc-arm-embedded

- ## 3.克隆village-kernel项目
	- ### ssh方式：
			git clone git@github.com:village-kernel/village-rust.git
  
  	- ### https方式：
			git clone https://github.com/village-kernel/village-rust.git


- ## 4.使用vscode打开village-kernel项目
		把项目目录village-kernel拉到vscode界面
		
		接着打开vscode终端，拷贝配置文件
		cp village-scripts/configs/stm32f407.config .config
		
		修改配置
		make menuconfig
		进入Compiler选项
		
		配置宿主机编译器：
		() host compile prefix
		(-13) host compile suffix
		
		配置交叉编译器：
		(arm-none-eabi-) cross compile prefix
		() cross compile suffix

		编译项目
		make

- ## 5.创建rootfs文件系统镜像
		格式化内存卡为fat分区格式
		
		设置分区名称为VILLAGE OS

		修改rootfs文件系统挂载路径
		make menuconfig

		进入Compiler选项
		(/Volumes/VILLAGE OS) rootfs path

		拷贝相关文件到文件系统
		make rootfs

		从PC拔出卡，把卡插入到开发板上面


- ## 6.运行与调试代码
		切换到vscode debug界面
		选择Cortex Debug-stlink STM32F407
        使用minicom串口终端进行交互，默认波特率115200

# 调试动态加载的程序：

- 正常启动debug内核
- 在与内核交互的串口终端使用run命令加载程序
- 运行之后会出现debug信息，其中0xxxxxxxx就是程序加载的地址
- 例如运行调试demo.exec应用程序

		root@village / # run /programs/demo.exec
		[Debug] load at 0x0016ec20, /programs/demo.exec load done

- 加载符号到gdb，暂停debug内核，切换到 调试控制台，执行以下命令:
	
		-exec add-symbol-file build/output/programs/demo.elf -o 0x0016ec20

- 继续debug内核，这样就可以调试demo应用程序了，如果demo太快运行结束那也没法调试。


# 说明
- 目前处于重写初级阶段，很多模块还没重写，暂时不可用。

//###########################################################################
// main.rs
// The specific implementation of functions related to main
//
// $Copyright: Copyright (C) village
//###########################################################################
#![no_std]
#![no_main]

// import alloc
extern crate alloc;

// import village
pub mod village;
pub use village::traits as traits;
pub use village::misc as misc;

use crate::traits::vk_driver::{Command, FBCommand};
use crate::village::misc::fopts::vk_dev_fopt::DevFopt;

// Main
#[unsafe(no_mangle)]
pub fn main(_argv: &[&str]) {
    // 0. 获取display驱动并打开
    println!("\n=== 0. 打开驱动 ===");
    let mut fbdev = DevFopt::new();
    if !fbdev.open("display0") {
        println!("打开display驱动失败");
        return;
    }

    // 1. 获取宽度
    println!("\n=== 1. 获取宽度 ===");
    let mut width_cmd = Command::FB(FBCommand::Width { width: 0 });
    fbdev.ioctrl(&mut width_cmd);
    if let Command::FB(FBCommand::Width { width }) = width_cmd {
        println!("宽度: {}", width);
    }
    
    // 2. 获取高度
    println!("\n=== 2. 获取高度 ===");
    let mut height_cmd = Command::FB(FBCommand::Height { height: 0 });
    fbdev.ioctrl(&mut height_cmd);
    if let Command::FB(FBCommand::Height { height }) = height_cmd {
        println!("高度: {}", height);
    }
    
    // 3. 清除屏幕
    println!("\n=== 3. 清除屏幕 ===");
    let mut clear_cmd = Command::FB(FBCommand::Clear { });
    fbdev.ioctrl(&mut clear_cmd);
    println!("屏幕已清除");

    // 4. 绘制点
    println!("\n=== 4. 绘制点 ===");
    let mut draw_cmd = Command::FB(FBCommand::DrawPoint { x: 100, y: 100, color: 0x1234 });
    fbdev.ioctrl(&mut draw_cmd);
    if let Command::FB(FBCommand::DrawPoint { x, y, color }) = draw_cmd {
        println!("在坐标({}, {})绘制了颜色为0x{:X}的点", x, y, color);
    }

    // 5. 读取点
    println!("\n=== 5. 读取点 ===");
    let mut read_cmd = Command::FB(FBCommand::ReadPoint { x: 100, y: 100, color: 0 });
    fbdev.ioctrl(&mut read_cmd);
    if let Command::FB(FBCommand::ReadPoint { x, y, color }) = read_cmd {
        println!("坐标({}, {})的颜色是0x{:X}", x, y, color);
    }
  
    // 6. 填充颜色
    println!("\n=== 6. 填充颜色 ===");
    let mut fill_color_cmd = Command::FB(FBCommand::FillColor { sx: 50, sy: 50, ex: 150, ey: 150, color: 0x00FF00 });
    fbdev.ioctrl(&mut fill_color_cmd);
    if let Command::FB(FBCommand::FillColor { sx, sy, ex, ey, color }) = fill_color_cmd {
        println!("在矩形区域[({}, {}), ({}, {})]填充了颜色0x{:X}", sx, sy, ex, ey, color);
    }
    
    // 7. 填充像素
    println!("\n=== 7. 填充像素 ===");
    let mut fill_pixel_cmd = Command::FB(FBCommand::FillPixel { sx: 200, sy: 200, ex: 210, ey: 210, pixel: alloc::vec![0x1234; 100] });
    fbdev.ioctrl(&mut fill_pixel_cmd);
    if let Command::FB(FBCommand::FillPixel { sx, sy, ex, ey, pixel }) = fill_pixel_cmd {
        println!("在矩形区域[({}, {}), ({}, {})]填充了{}个像素数据", 
            sx, sy,
            ex, ey,
            pixel.len());
    }
}

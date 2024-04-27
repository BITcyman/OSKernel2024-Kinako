# 开发日志

#### 4.27
完成批处理系统 rCore-ch2，中途遇到一个 bug，在 main.rs 中 如果 clear_bss() 两次，会出现后续程序输出乱码

``` rust
pub fn rust_main() -> ! {
    clear_bss();
    println!("Here is Kinako!");
    //// if this run clear_bss() again
    // clear_bss();
    logging::init();
    print_kernel_info();
    trap::init();
    batch::init();    
    batch::run_next_app();
    // loop{}
}

```


#### 4.22
加入 user

#### 4.21
完成 rCore-ch1，搭建了一个基本内核

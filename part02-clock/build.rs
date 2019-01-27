use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=memory.x");

//  used when invoking C code to configure system clock
//  cc::Build::new()
//      .define("USE_HAL_DRIVER", None)
//      .define("STM32F100xB", None)
//      .include("c/Inc")
//      .include("c/Drivers/CMSIS/Include")
//      .include("c/Drivers/CMSIS/Device/ST/STM32F1xx/Include")
//      .include("c/Drivers/STM32F1xx_HAL_Driver/Inc")
//      .file("c/Src/main.c")
//      .file("c/Src/system_stm32f1xx.c")
//      .file("c/Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal.c")
//      .file("c/Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_cortex.c")
//      .file("c/Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_gpio.c")
//      .file("c/Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_rcc.c")
//      .flag("-fno-pic")
//      .archiver("arm-none-eabi-ar")
//      .compile("libdevice.a");
//
//  println!("cargo:rerun-if-changed=c/Src/main.c");
}

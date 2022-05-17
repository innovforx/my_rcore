use crate::sbi::console_putchar;
use core::fmt::{self,Write};

// const PRINT_LEVEL:usize = 1;




pub struct Stdout;

impl Write for Stdout{
    fn write_str(&mut self,s: &str)->fmt::Result{
        for c in s.chars(){
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(arg:fmt::Arguments){
    Stdout.write_fmt(arg).unwrap();
}

#[macro_export]
macro_rules! print{
    ($fmt:literal $(,$($arg:tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println{
    ($fmt:literal $(,$($arg:tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! Errorln{
    ($fmt:literal $(,$($arg:tt)+)?) => {
        if cfg!(feature = "error") || cfg!(feature = "all"){
            $crate::console::print(format_args!(concat!("\x1b[1;31m",$fmt, "\x1b[0m\n") $(, $($arg)+)?));
        }
    }
}


#[macro_export]
macro_rules! Warnln{
    ($fmt:literal $(,$($arg:tt)+)?) => {
        if cfg!(feature = "warn") || cfg!(feature = "all"){
            $crate::console::print(format_args!(concat!("\x1b[93m",$fmt, "\x1b[0m\n") $(, $($arg)+)?));
        }
    }
}


#[macro_export]
macro_rules! Infoln{
    ($fmt:literal $(,$($arg:tt)+)?) => {
        if cfg!(feature = "info") || cfg!(feature ="all"){
            $crate::console::print(format_args!(concat!("\x1b[34;47m",$fmt, "\x1b[0m\n") $(, $($arg)+)?));
        }
    }
}

#[macro_export]
macro_rules! Debugln{
    ($fmt:literal $(,$($arg:tt)+)?) => {
        if cfg!(feature = "debug") || cfg!(feature = "all"){
            $crate::console::print(format_args!(concat!("\x1b[4;32m",$fmt, "\x1b[0m\n") $(, $($arg)+)?));
        }        
    }
}



#[macro_export]
macro_rules! Traceln{
    ($fmt:literal $(,$($arg:tt)+)?) => {
        if cfg!(feature = "tarce") || cfg!(feature = "all"){
            $crate::console::print(format_args!(concat!("\x1b[4;90m",$fmt, "\x1b[0m\n") $(, $($arg)+)?));
        }
    }
}






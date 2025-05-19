target extended-remote :12345
set print asm-demangle on
set backtrace limit 32

load

break _start
break multiboot_entry

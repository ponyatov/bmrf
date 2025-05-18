MEMORY
{
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM  (rwx) : ORIGIN = 0x20000000, LENGTH =  128K
}

/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */
/* _stext = ORIGIN(FLASH) + 0x400; /* no bootloader, vector table only */

MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  /* Adjust these memory regions to match your device memory layout */
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K
  RAM : ORIGIN = 0x20000000, LENGTH = 32K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);
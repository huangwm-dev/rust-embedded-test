/* Memory layout of the LM3S6965 microcontroller */
/* 1K = 1 KiBi = 1024 bytes */
MEMORY
{
  FLASH (rx)    : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM   (xrw)   : ORIGIN = 0x20000000, LENGTH = 96K
}

/* The entry point is the reset handler */
ENTRY(reset_handler);

EXTERN(__RESET_VECTOR);

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    /* First entry: initial Stack Pointer value */
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    /* Second entry: reset vector */
    KEEP(*(.vector_table.reset_vector));
  } > FLASH

  .text : ALIGN(4)
  {
    *(.text) 
    *(.text.*)
  } > FLASH

  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH

  .bss :
  {
    _sbss = .;
    *(.bss .bss.*);
    _ebss = .;
  } > RAM

  .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
  {
    _sdata = .;
    *(.data .data.*);
    _edata = .;
  } > RAM

  _sidata = LOADADDR(.data);

  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}

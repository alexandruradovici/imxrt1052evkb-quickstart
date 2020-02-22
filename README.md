# `cortex-m-quickstart` for imxrt1052evkb

Here are the steps:


1. Compiled a binary in MCUxpress, got the afx file (elf) and extracted the .boot_hdr section into a binary file 
  ``objcopy -O binary --only-section=.boot_hdr evkbimxrt1050_iled_blinky.elf hdr``
2. Added the hdr.s file to add the .boot_hdr section
3. Added in memory.x the BOOT_HDR section
4. Rename arm-m7 file to arm-m7.elf
5. Loaded the file with MCUxpress debug


It prints hello world in the console, GPIO is not working yet.
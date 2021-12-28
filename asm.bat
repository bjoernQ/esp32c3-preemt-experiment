REM riscv32-esp-elf-gcc trap.S


riscv32-esp-elf-gcc -ggdb3  -c -mabi=ilp32 -march=rv32i trap.S -o bin/esp32c3trap.o
riscv32-esp-elf-ar crs bin/trap_riscv32i-unknown-none-elf.a bin/esp32c3trap.o

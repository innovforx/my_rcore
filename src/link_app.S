
    .align 3
    .section .data
    .global _num_app
    .global _app_info_start
    .global _app_info_end
_num_app:
    .quad 4
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_3_start
    .quad app_3_end
    .quad app_0_info_start
    .quad app_1_info_start
    .quad app_2_info_start
    .quad app_3_info_start
    .quad app_3_info_end

    .section .data
    .global app_0_start
    .global app_0_end
    .global app_0_info_start
    .global app_0_info_end
app_0_info_start:
    .byte 7
    .string "power_3"
app_0_info_end:
app_0_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/power_3.bin"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
    .global app_1_info_start
    .global app_1_info_end
app_1_info_start:
    .byte 7
    .string "power_5"
app_1_info_end:
app_1_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/power_5.bin"
app_1_end:

    .section .data
    .global app_2_start
    .global app_2_end
    .global app_2_info_start
    .global app_2_info_end
app_2_info_start:
    .byte 7
    .string "power_7"
app_2_info_end:
app_2_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/power_7.bin"
app_2_end:

    .section .data
    .global app_3_start
    .global app_3_end
    .global app_3_info_start
    .global app_3_info_end
app_3_info_start:
    .byte 5
    .string "sleep"
app_3_info_end:
app_3_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/sleep.bin"
app_3_end:

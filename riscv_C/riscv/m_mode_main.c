// m_mode_main.c
volatile int *UART0 = (int*)0x10000000; // address for UART0 on QEMU virt machine
volatile int *UART0_SEND = (int*)0x12345678; //write address to send uart
void my_putchar(char c) {
    *UART0 = c;
}

void write_uart(){
    *UART0_SEND ^=1;
}

void my_puts(const char *s) {
    while (*s) my_putchar(*s++);
    write_uart();
}

void _start() {
    my_puts("Hello from M-mode!\n");
    while (1);
}

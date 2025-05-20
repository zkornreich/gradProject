Inspiration:

Rust is an up-and-coming memory safe language. The language offers the efficiency of C, with the addition of memory checking and safety features. As a modern language, it is highly compatible with the integration of functions in foreign languages. This provides a new attack surface, however, when integrating with memory unsafe languages like C. Allowing a C function to run on a Rust platform could lead to the tampering of memory from the C function that goes unchecked by Rust, leading to unwanted program manipulation. This inspired research into foreign function interfacing (FFI) that would mitigate this issue. A new FFI method involves a process called encapsulation, which trampolines between memory protection modes when running untrusted functions. 
RISC-V brings about a physical memory protection (PMP) unit that can be configured in M-Mode. Leveraging this PMP, we can secure memory access when executing untrusted functions. This involves the configuration of the PMP to restrict memory access for user mode functions, and trampolining between U-Mode and M-Mode.

The Initial Goal:

Proof of concept leveraging the Qemu Virt. RISC-V PMP to create an FFI that secures memory when executing untrusted functions.

The Shifted Focus:

- Trusted Execution Environment (TEE)
- - A Trusted Execution Environment (TEE) is a secure, isolated area of memory that is inaccessible by the device's operating system and other applications. 
- - TEEs are often used for secure payments, biometric authentication, and other security-sensitive tasks.
- This protection is typically achieved through hardware-level security features, making it more robust than software-based security measures.  
- TEEs ensure that data remains confidential and tamper-resistant, even if the device kernal or other applications are compromised. Particulalry, TEEs are focused around protection against operating systems with vulnaribilities. Typically, if you gain kernal control of a linux system, you then have access to every part of the system. However, TEEs protect memory from the vulnerable kernal, retaining program security despite the platform/device being compromised.
- The RISC-V PMP configuration includes an optional "Lock Bit". When this bit is set, the machine mode program must adhere to the specified pmp configuration for that memory segment. Thus, RISC-V is great for configuring Trusted Execution Environments. 
- - seL4 is an open-source OS microkernel that has been verified on RISC-V. This microkernal has leveraged the RISC-V PMP to configure TEEs.

The Shifted Goal:

Proof of concept leveraging the Qemu Virt. RISC-V PMP to create a TEE that secures memory from machine mode to prevent a compromised kernal from tampering with protected data.

Work Done & File Structure
1. Rust Research | rustBook
- using The Rust Book to learn the Rust programming language.
- Took notes within the rust-book directory on rust as I grew familiar with the language.

2. RISC-V Proof of concept - Rust | riscv_rust
- Inlined assembly with Rust was causing difficult errors regardless of versions or imported git repos
- which lead to a change in approach. 

3. RISC-V Proof of concept - C | riscv_C/riscv-helloworld-c:
- Found basic baremetal C implementation with UART to print to terminal from Qemu virt on risc-V
- Created PMP configuration code to protect a region of memory
- Created Trap and Trap Handler to catch and print memory violations
- Created user-mode transition and code to execute in user mode
- - Discovered issues with user mode transition, where any transitition to user mode will cause a trap without regard for what is contained in the user mode code.
- - If Trap is removed, user code executes without flaw.
- Created program to enable the lock bit, thus restricting M-Mode from accessing protected memory regions
- - Successful test implies proper PMPConfig abilities, so issues must be within user mode transition
- - Hypothesis, user mode needs access to it's program. Currently, accessing program data in user mode triggers traps.
- - Notes - pmp csr with lock bit enabled must come before pmp csrs without locking. Failure will result in ineffective lock

Future Work Ideas
- Debug why user mode has no permissions
- - already tried expicity granting RWX permissions to U-Mode with pmp.
- Translate code onto a pre-made OpenSBI program
- - Missing setup steps that would be provided by open-sbi
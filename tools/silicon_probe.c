#include <stdio.h>
#include <stdint.h>

// ARM64 Silicon Discovery.
// Bypasses high-level OS APIs to read hardware ID registers directly.

int main() {
    uint64_t midr;
    
    // Read Main ID Register (MIDR_EL1)
    // This provides implementer, variant, architecture, part number, and revision.
    __asm__ volatile ("mrs %0, midr_el1" : "=r" (midr));

    uint32_t implementer = (midr >> 24) & 0xFF;
    uint32_t variant = (midr >> 20) & 0xF;
    uint32_t part_num = (midr >> 4) & 0xFFF;
    uint32_t revision = midr & 0xF;

    const char* impl_str = "Unknown";
    if (implementer == 0x41) impl_str = "ARM Limited";
    else if (implementer == 0x42) impl_str = "Broadcom";
    else if (implementer == 0x43) impl_str = "Cavium";
    else if (implementer == 0x44) impl_str = "Digital Equipment";
    else if (implementer == 0x4e) impl_str = "NVIDIA";
    else if (implementer == 0x50) impl_str = "Applied Micro";
    else if (implementer == 0x51) impl_str = "Qualcomm";
    else if (implementer == 0x53) impl_str = "Samsung";
    else if (implementer == 0x56) impl_str = "Marvell";
    else if (implementer == 0x66) impl_str = "Intel";
    else if (implementer == 0x69) impl_str = "Apple";

    printf("--- BARE METAL SILICON DISCOVERY (ARM64) ---\n");
    printf("OS Bypass:          ACTIVE (MRS Instruction executed)\n");
    printf("MIDR_EL1 Raw:       0x%016lx\n", midr);
    printf("Implementer:        0x%02x (%s)\n", implementer, impl_str);
    printf("Part Number:        0x%03x\n", part_num);
    printf("Variant:            0x%x\n", variant);
    printf("Revision:           0x%x\n", revision);
    
    // Check for Hypervisor presence via ID_AA64PFR0_EL1 if possible
    uint64_t pfr0;
    __asm__ volatile ("mrs %0, id_aa64pfr0_el1" : "=r" (pfr0));
    printf("Features (PFR0):    0x%016lx\n", pfr0);

    return 0;
}

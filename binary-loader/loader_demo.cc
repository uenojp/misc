#include <stdint.h>
#include <stdio.h>

#include <string>

#include "loader.h"

int main(int argc, char** argv) {
    if (argc < 2) {
        fprintf(stderr, "Usage %s <binary>\n", argv[0]);
        return 1;
    }

    std::string filename = std::string(argv[1]);
    Binary binary;
    if (load_binary(filename, &binary, Binary::BIN_TYPE_AUTO) < 0) {
        fprintf(stderr, "Failed to load binary");
        return 1;
    }

    printf("Loaded binary '%s' %s/%s (%u bits) entrypoint@0x%016jx\n", binary.filename.c_str(),
           binary.type_str.c_str(), binary.arch_str.c_str(), binary.bits, binary.entrypoint);

    puts("\nSections:");
    printf("    %-16s   %-8s %-20s %s\n", "vma", "size", "name", "type");
    for (auto& section : binary.sections) {
        printf("    0x%016jx %-8ju %-20s %s\n", section.virtual_memory_addrres, section.size,
               section.name.c_str(), (section.type == Section::SEC_TYPE_CODE) ? "CODE" : "DATA");
    }

    puts("\nSymbols:");
    printf("    %-40s %-16s   %s\n", "name", "addr", "type");
    for (auto& symbol : binary.symbols) {
        printf("    %-40s 0x%016jx %s\n", symbol.name.c_str(), symbol.addr,
               (symbol.type & Symbol::SYM_TYPE_FUNC) ? "FUNC" : "");
    }

    unload_binary(&binary);

    return 0;
}

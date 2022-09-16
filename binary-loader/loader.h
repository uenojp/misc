#pragma once

#include <string>
#include <vector>

class Binary;
class Section;
class Symbol;

class Binary {
   public:
    enum BinaryType {
        BIN_TYPE_AUTO,
        BIN_TYPE_ELF,
        BIN_TYPE_PE,
    };

    enum BinaryArch {
        ARCH_NONE,
        ARCH_X86,
    };

    Binary() : filename(), type(BIN_TYPE_AUTO), arch(ARCH_NONE), bits(0), entrypoint(0) {}

    Section *get_text_section() {
        for (auto &section : sections)
            if (section.name == ".text")
                return &section;
        return NULL;
    }

    std::string filename;
    BinaryType type;
    std::string type_str;
    BinaryArch arch;
    std::string arch_str;
    unsigned int bits;
    uint64_t entrypoint;
    std::vector<Section> sections;
    std::vector<Symbol> symbols;
};

class Section {
   public:
    enum SectionType {
        SEC_TYPE_NONE,
        SEC_TYPE_CODE,
        SEC_TYPE_DATA,
    };

    Section()
        : name(),
          type(SEC_TYPE_NONE),
          binary(nullptr),
          virtual_memory_addrres(0),
          size(0),
          bytes(nullptr) {}

    bool contains(uint64_t addr) {
        return (virtual_memory_addrres <= addr) && (addr - virtual_memory_addrres < size);
    }

    std::string name;
    SectionType type;
    Binary *binary;
    uint64_t virtual_memory_addrres;
    uint64_t size;
    uint8_t *bytes;
};

class Symbol {
   public:
    enum SymbolType {
        SYM_TYPE_UNKOWN,
        SYM_TYPE_FUNC,
    };

    Symbol() : name(), type(SYM_TYPE_UNKOWN), addr(0) {}

    std::string name;
    SymbolType type;
    uint64_t addr;
};

int load_binary(std::string &filename, Binary *binary, Binary::BinaryType type);
void unload_binary(Binary *binary);

#pragma once

#include <string>
#include <vector>

class Binary;
class Section;
class Symbol;

class Symbol {
   public:
    enum SymbolType {
        SYM_TYPE_UNKOWN,
        SYM_TYPE_FUNC,
    };

    Symbol() : name(), type(SYM_TYPE_UNKOWN), addr(0) {}
    Symbol(std::string name, SymbolType type, uint64_t addr) : name(name), type(type), addr(addr) {}

    std::string name;
    SymbolType type;
    uint64_t addr;
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
          binary(NULL),
          virtual_memory_addrres(0),
          size(0),
          bytes(NULL) {}
    Section(std::string name, SectionType type, Binary *binary, uint64_t vma, uint64_t size,
            uint8_t *bytes)
        : name(name),
          type(type),
          binary(binary),
          virtual_memory_addrres(vma),
          size(size),
          bytes(bytes) {}

    bool contains(uint64_t addr) {
        return (virtual_memory_addrres <= addr) && (addr - virtual_memory_addrres < size);
    }

    std::string name;
    SectionType type;
    Binary *binary;
    uint64_t virtual_memory_addrres;
    uint64_t size;
    uint8_t *bytes; // Section content
};

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

int load_binary(std::string &filename, Binary *binary, Binary::BinaryType type);
void unload_binary(Binary *binary);

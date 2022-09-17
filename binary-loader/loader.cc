#include "loader.h"

#include <bfd.h>

static int load_binary_bfd(std::string &filename, Binary *binary, Binary::BinaryType type);
static bfd *open_bfd(std::string &filename);
static int load_symbols_bfd(bfd *bfd_h, Binary *binary);
static int load_dynsym_bfd(bfd *bfd_h, Binary *binary);
static int load_sections_bfd(bfd *bfd_h, Binary *binary);

int load_binary(std::string &filename, Binary *binary, Binary::BinaryType type) {
    return load_binary_bfd(filename, binary, type);
}

void unload_binary(Binary *binary) {
    for (auto &section : binary->sections) {
        if (section.bytes) {
            free(section.bytes);
        }
    }
}

static int load_binary_bfd(std::string &filename, Binary *binary, Binary::BinaryType type) {
    int ret = 0;

    bfd *bfd_h = open_bfd(filename);
    if (!bfd_h) {
        goto fail;
    }

    binary->filename = std::string(filename);
    binary->entrypoint = bfd_get_start_address(bfd_h);

    binary->type_str = std::string(bfd_h->xvec->name);
    switch (bfd_h->xvec->flavour) {
        case bfd_target_elf_flavour:
            binary->type = Binary::BIN_TYPE_ELF;
            break;
        case bfd_target_coff_flavour:
            binary->type = Binary::BIN_TYPE_PE;
        default:
            fprintf(stderr, "Unsuported binary type (%s)\n", bfd_h->xvec->name);
            goto fail;
    }

    const bfd_arch_info_type *arch_info;
    arch_info = bfd_get_arch_info(bfd_h);
    binary->arch_str = std::string(arch_info->printable_name);
    switch (arch_info->mach) {
        case bfd_mach_i386_i386:
            binary->arch = Binary::ARCH_X86;
            binary->bits = 32;
            break;
        case bfd_mach_x86_64:
            binary->arch = Binary::ARCH_X86;
            binary->bits = 64;
            break;
        default:
            fprintf(stderr, "Unsuported architecture (%s)\n", arch_info->printable_name);
            goto fail;
    }

    load_symbols_bfd(bfd_h, binary);
    load_dynsym_bfd(bfd_h, binary);
    if (load_sections_bfd(bfd_h, binary) < 0)
        goto fail;

    ret = 0;
    goto cleanup;

fail:
    ret = -1;

cleanup:
    if (bfd_h)
        bfd_cleanup(bfd_h);

    return ret;
}

static bfd *open_bfd(std::string &filename) {
    static int bfd_inited = 0;

    if (!bfd_inited) {
        bfd_init();
        bfd_inited = 1;
    }

    bfd *bfd_h = bfd_openr(filename.c_str(), NULL);
    if (!bfd_h) {
        fprintf(stderr, "Failed to open binary '%s' (%s)\n", filename.c_str(),
                bfd_errmsg(bfd_get_error()));
        return NULL;
    }

    // NOTE:
    // bfd_object,		/* Linker/assembler/compiler output.  */
    // ref. bfd.h
    if (!bfd_check_format(bfd_h, bfd_object)) {
        fprintf(stderr, "File '%s' does not look like an executable (%s)\n", filename.c_str(),
                bfd_errmsg(bfd_get_error()));
        return NULL;
    }
    // bfd_check_formatが正しいフォーマットでもbfd_error_wrong_formatを設定して返すことがある
    // (どのバージョンでこのバグ入っているか調べても分からなかった)
    // ref. p.69
    bfd_set_error(bfd_error_no_error);

    if (bfd_get_flavour(bfd_h) == bfd_target_unknown_flavour) {
        fprintf(stderr, "Unrecognized format for binary '%s' (%s)\n", filename.c_str(),
                bfd_errmsg(bfd_get_error()));
        return NULL;
    }

    return bfd_h;
}

static int load_symbols_bfd(bfd *bfd_h, Binary *binary) {
    int ret = 0;
    // bfd_get_symtab_upper_boundは.symtabのエントリ数x8が返ってくる
    // 例えば、
    // readelf --symbols ./bin | grep symtab
    // Symbol table '.symtab' contains 96 entries:
    // この96エントリなので、96 * sizeof(*asynbol) = 96 * 8 = 762
    const long nbytes = bfd_get_symtab_upper_bound(bfd_h);
    asymbol **symtab;
    if (nbytes < 0) {
        fprintf(stderr, "Failed to read symtab (%s)\n", bfd_errmsg(bfd_get_error()));
        goto fail;
    } else if (nbytes) {
        symtab = (asymbol **)malloc(nbytes);
        if (!symtab) {
            fprintf(stderr, "Out of memroy\n");
            goto fail;
        }

        // symtabにシンボルテーブルを読む
        const long nsym = bfd_canonicalize_symtab(bfd_h, symtab);
        if (nsym < 0) {
            fprintf(stderr, "Failed to read symtab (%s)\n", bfd_errmsg(bfd_get_error()));
            goto fail;
        }

        for (long i = 0; i < nsym; i++) {
            if (symtab[i]->flags && BSF_FUNCTION) {
                binary->symbols.push_back(Symbol(std::string(symtab[i]->name),
                                                 Symbol::SYM_TYPE_FUNC,
                                                 bfd_asymbol_value(symtab[i])));
            }
        }
    }

    ret = 0;
    goto cleanup;

fail:
    ret = -1;

cleanup:
    free(symtab);

    return ret;
}

static int load_dynsym_bfd(bfd *bfd_h, Binary *binary) {
    int ret = 0;
    // bfd_get_symtab_upper_boundは.symtabのエントリ数x8が返ってくる
    // 例えば、
    // readelf --symbols ./bin | grep symtab
    // Symbol table '.symtab' contains 96 entries:
    // この96エントリなので、96 * sizeof(*asynbol) = 96 * 8 = 762
    const long nbytes = bfd_get_dynamic_symtab_upper_bound(bfd_h);
    asymbol **dynsym;
    if (nbytes < 0) {
        fprintf(stderr, "Failed to read symtab (%s)\n", bfd_errmsg(bfd_get_error()));
        goto fail;
    } else if (nbytes) {
        dynsym = (asymbol **)malloc(nbytes);
        if (!dynsym) {
            fprintf(stderr, "Out of memroy\n");
            goto fail;
        }

        // symtabにシンボルテーブルを読む
        const long nsym = bfd_canonicalize_dynamic_symtab(bfd_h, dynsym);
        if (nsym < 0) {
            fprintf(stderr, "Failed to read symtab (%s)\n", bfd_errmsg(bfd_get_error()));
            goto fail;
        }

        for (long i = 0; i < nsym; i++) {
            if (dynsym[i]->flags && BSF_FUNCTION) {
                binary->symbols.push_back(Symbol(std::string(dynsym[i]->name),
                                                 Symbol::SYM_TYPE_FUNC,
                                                 bfd_asymbol_value(dynsym[i])));
            }
        }
    }

    ret = 0;
    goto cleanup;

fail:
    ret = -1;

cleanup:
    free(dynsym);

    return ret;
}

static int load_sections_bfd(bfd *bfd_h, Binary *binary) {
    for (asection *bfd_sec = bfd_h->sections; bfd_sec != NULL; bfd_sec = bfd_sec->next) {
        const int bfd_flags = bfd_section_flags(bfd_sec);

        Section::SectionType type = Section::SEC_TYPE_NONE;
        if (bfd_flags & SEC_CODE) {
            type = Section::SEC_TYPE_CODE;
        } else if (bfd_flags & SEC_DATA) {
            type = Section::SEC_TYPE_DATA;
        } else {
            continue;
        }

        const char *name = bfd_section_name(bfd_sec);
        const uint64_t vma = bfd_section_vma(bfd_sec);
        const uint64_t size = bfd_section_size(bfd_sec);
        uint8_t *bytes = (uint8_t *)malloc(size);
        if (!bytes) {
            fprintf(stderr, "Out of memory\n");
            return -1;
        }
        if (!bfd_get_section_contents(bfd_h, bfd_sec, bytes, 0, size)) {
            fprintf(stderr, "Failed to read section '%s' (%s)\n", name,
                    bfd_errmsg(bfd_get_error()));
            return -1;
        }

        binary->sections.push_back(Section(std::string(name), type, binary, vma, size, bytes));
    }

    return 0;
}

#include <bfd.h>

#include <iostream>
#include <string>

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

int run(int argc, char **argv) {
    std::string filename = "./xxx";
    bfd *abfd = open_bfd(filename);
    std::cout << bfd_get_start_address(abfd) << std::endl;
    std::cout << abfd->xvec->name << std::endl;
    std::cout << abfd->xvec->flavour << std::endl;

    const bfd_arch_info *arch_info = bfd_get_arch_info(abfd);
    std::cout << arch_info->printable_name << std::endl;
    std::cout << arch_info->arch_name << std::endl;

    std::cout << bfd_get_symtab_upper_bound(abfd) << std::endl;

    return 0;
}

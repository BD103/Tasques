{ pkgs }: {
    deps = [
        pkgs.rustc
        pkgs.cargo
        pkgs.rustfmt
        pkgs.clippy

        pkgs.vim
        pkgs.python39Full
    ];
}
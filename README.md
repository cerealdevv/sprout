# SPROUT

## what is sprout?
sprout/System PRobing OUTput is a diagnostic tool written in rust
it is similar to tools like fastfetch/neofetch, but is intended for diagnostic purposes rather
than cosmetic purposes

## why does sprout exist?
sprout is developed to debug bugs, compilation quirks, etc
PRs/Issues should include sprout to help diagnose if the issue is a one-off or if the issue
is an underlying issue with architecture/libc/os

## Support
```
╭─────────────────┬────────────┬─────────┬────────────┬──────────┬─────────╮
│OS               │Architecture│SDK/libc │Supported   │Tested    │Last Test│
├─────────────────┼────────────┼─────────┼────────────┼──────────┼─────────┤
│Arch Linux       │x86_64      │glibc    │yes         │officially│rolling  │
├─────────────────┼────────────┼─────────┼────────────┼──────────┼─────────┤
│Android 17       │aarch64     │37/Bionic│yes         │officially│0.3-wip  │
├─────────────────┼────────────┼─────────┼────────────┼──────────┼─────────┤
│Android 15       │aarch64     │35/Bionic│yes         │officially│0.1      │
├─────────────────┼────────────┼─────────┼────────────┼──────────┼─────────┤
│Android 15 TV    │armv7l      │35/Bionic│yes         │rarely    │n/a      │
├─────────────────┼────────────┼─────────┼────────────┼──────────┼─────────┤
│Android 10       │armv7l      │27/Bionic│unofficially│officially│0.1      │
├─────────────────┼────────────┼─────────┼────────────┼──────────┼─────────┤
│Windows 11       │x86_64      │UCRT     │yes         │rarely    │0.1      │
├─────────────────┼────────────┼─────────┼────────────┼──────────┼─────────┤
│LineageOS 22(A15)│aarch64     │35/Bionic│yes         │community │0.1-wip  │
╰─────────────────┴────────────┴─────────┴────────────┴──────────┴─────────╯
```

## i tested sprout on a different OS, how can i contribute to the list?
to contribute to the list, please open an issue with the relevant information.

## sprout versioning
sprout operates on a slightly different versioning than most other programs, therefore
i would like to clear up some confusion.

### sprouts versioning
sprout has recently changes its versioning behind the scenes, the public facing version is still v0.3,
however, to make nix packaging easier, we have dropped the -wip prefix officially, now, in development
versions change the PATCH field of the cargo version, it changes after each commit and resets to zero
after a MINOR or MAJOR field change.

## Packaging
```
╭───────┬────────────┬──────────────┬─────────────────────┬────────────┬───────╮
│Format │Repository  │Type          │Libc/Architecture    │Maintenance │Suffix │
├───────┼────────────┼──────────────┼─────────────────────┼────────────┼───────┤
│Cargo  │crates.io   │release       │any supported        │yes         │-cargo │
├───────┼────────────┼──────────────┼─────────────────────┼────────────┼───────┤
│AUR    │local       │release & git │x86_64/aarch64 glibc │yes         │-AUR   │
├───────┼────────────┼──────────────┼─────────────────────┼────────────┼───────┤
│make   │local       │git           │any supported        │yes         │-make  │
├───────┼────────────┼──────────────┼─────────────────────┼────────────┼───────┤
│Gentoo │local       │release & git │any supported        │yes         │-gentoo│
├───────┼────────────┼──────────────┼─────────────────────┼────────────┼───────┤
│Nix    │local       │release & git │any supported        │yes         │-nix   │
├───────┼────────────┼──────────────┼─────────────────────┼────────────┼───────┤
│APK    │local       │release       │any supported; musl  │yes         │-apk   │
├───────┼────────────┼──────────────┼─────────────────────┼────────────┼───────┤
│XBPS   │local       │release       │any supported        │yes         │-void  │
╰───────┴────────────┴──────────────┴─────────────────────┴────────────┴───────╯
```

```
╭───────┬────────────────┬──────────────────────────────┬───────────────────────╮
│Format │Repository host │Link                          │Note                   │
├───────┼────────────────┼──────────────────────────────┼───────────────────────┤
│Cargo  │crates.io       │crates.io/crates/sprout-prober│none                   │
├───────┼────────────────┼──────────────────────────────┼───────────────────────┤
│AUR    │codeberg.org    │codeberg.org/cerealbox/arch   │soon aur.archlinux.org │
├───────┼────────────────┼──────────────────────────────┼───────────────────────┤
│make   │codeberg.org    │codeberg.org/cerealdevv/sprout│not "packaging"        │
├───────┼────────────────┼──────────────────────────────┼───────────────────────┤
│Gentoo │codeberg.org    │codeberg.org/cerealbox/gentoo │maybe soon GURU?       │
├───────┼────────────────┼──────────────────────────────┼───────────────────────┤
│Nix    │codeberg.org    │codeberg.org/cerealdevv/sprout│soon in nixpkgs        │
├───────┼────────────────┼──────────────────────────────┼───────────────────────┤
│APK    │codeberg.org    │codeberg.org/cerealbox/alpine │none                   │
├───────┼────────────────┼──────────────────────────────┼───────────────────────┤
│XBPS   │codeberg.org    │codeberg.org/cerealbox/void   │none                   │
╰───────┴────────────────┴──────────────────────────────┴───────────────────────╯
```

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
│Android 17       │aarch64     │37/Bionic│yes         │officially│0.1      │
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

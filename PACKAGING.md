# SPROUT Packaging standards

SPROUT has some packaging standards, although they're not too strict.

## Changes
changes to sprout in packaging should be discussed with maintainers and the project owner.
of course, i can't stop you from packaging a slightly modified version, however you should cleanly
label your package as modified from upstream. (see Platforms)

## Platforms
as of now, there is no packaging or adoption of sprout outside of a crates.io package.
sprout has the following suffixes in mind for packaging, but they are purely suggestions, the suffix is at the maintainers discretion.
 - -deb (debian, ubuntu)
 - -rpm (fedora, osuse)
 - -arch (arch based)
 - -void (void linux)
 - -apk (alpine, chimera)
 - -fbsd (FreeBSD)
 - -obsd (OpenBSD)
 - -nbsd (NetBSD)
 - -dfbsd (DragonflyBSD)
 - -AUR (AUR)
 - -COPR (COPR)
 - -cargo (cargo)
if you make changes that are specific to your distribution, please deviate from the general packaging format suffix, and instead
tag your build something distinct, cleanly, for example, by using the first few letters of your distro name, or the last few
for ubuntu as an example, a clean build tag would be -ubu.

to set a suffix, you need to set the environment variable SPROUT_SOURCE to your desired suffix at compile time.
the suffixes -AUR and -cargo are reserved for upstream, and should not be used by packagers.

## Issues & Bugs
please only point users to upstream bug reporting if the issue actually comes from upstream.
if you are shipping outdated versions of sprout (for example on debian), you should backport bug fixes
to your packaged version if it makes sense and is possible.
as a packager for sprout, you are responsible for your package, if it does not backport bug fixes or
stay up to date with versioning, thats on you.

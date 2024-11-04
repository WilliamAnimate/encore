# The echotune music player

A TUI music player that does one thing and one thing only, but does it well.

It _only_ plays music, and it does it so well it has lower overhead than other media players.

> [!IMPORTANT]
> echotune is not yet a feature-complete music player! However, feel free to play around with it if you figure out the ropes.

## I want numbers. Sell me that its lightweight

The bulk (as of right now, all of) echotune's code is written on [this outdated laptop](https://www.ordinateursarabais.com/produit/acer-es1-521-40hc-hdmi-6-go-ram-1-tb/)[^1]. it takes about 200µs to draw to the tty and i've never seen it use >8% CPU usage. In fact, [it uses less memory than the systemd process when playing a 23 mb .flac file](./docs/img/echotune-less-bloated-than-systemd.png).

## Safe with Rust

Rust (alongside Zig) are the future of programming languages whether you like it or not. No longer will you have to choose between performance (C) or safe code (every other high level language that exists).

Because echotune is written in Rust, you need not worry about getting a remote code execution from a specifically crafted .flac file.

## Vi-inspired

Because echotune runs in the terminal, echotune comes with vi-like keybindings. That means if you are the based ones using a modal editor based on vi or vim then you will find echotune an easy adaptation.

## Multiplatform

echotune natively and always supports these platforms:

- Linux;
- ChromeOS

echotune has support and tries to maintain support for these platforms:

- *BSD[^2];
- MacOS;

Support is planned for the following platforms:

- Windows

[^1]: IOW, your modern Intel core 15 gen CPU @ 42 GHz with DDR7 RAM with a 32 TB NVMe SSD and liquid-cooled machine running the most bloated (GNU/)Linux distro (or Windows...) well surpasses the system requirements for running echotune, and that you will notice no difference in performance, if it can run pretty well on this AMD Quad-Core A4 processor @ 1.8 GHz
[^2]: *BSDs are in tier-3 support. I don't think rodio supports *BSD. echotune _may_ build but it may not function. The only "BSD" that is actually supported is Apple's Darwin operating system (where the XNU kernel uses some of FreeBSD's code)


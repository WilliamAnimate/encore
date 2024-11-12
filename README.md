<p align="center">
    <img src="docs/encore.svg" height="23">
</p>

# Encore

A TUI music player that does one thing and one thing only, but does it well.

It _only_ plays music, and it does it so well it has lower overhead than other media players.

> [!IMPORTANT]
> Encore is not yet a feature-complete music player! However, feel free to play around with it if you figure out the ropes.

## I want numbers. Sell me that its lightweight

The bulk (as of right now, all of) Encore's code is written on [this outdated laptop](https://www.ordinateursarabais.com/produit/acer-es1-521-40hc-hdmi-6-go-ram-1-tb/)[^1]. it takes about 200µs to draw to the tty and i've never seen it use >8% CPU usage. In fact, [it uses less memory than the systemd process when playing a 23 mb .flac file](./docs/img/encore-less-bloated-than-systemd.png).

## Safe with Rust

Rust (alongside Zig) are the future of programming languages whether you like it or not. No longer will you have to choose between performance (C) or safe code (every other high level language that exists).

Because Encore is written in Rust, you need not worry about getting a remote code execution from a specifically crafted .flac file.

## Vi-inspired

Because Encore runs in the terminal, Encore comes with vi-like keybindings. That means if you are the based ones using a modal editor based on vi or vim then you will find Encore an easy adaptation.

## Multiplatform

Encore natively and will always support these platforms:

- Linux;
- ChromeOS;
- Windows 10/11[^2]

Encore has support and tries to maintain support for these platforms:

- MacOS

Support is planned for the following platforms:

- Android (via termux, unrooted?)

[^1]: IOW, your modern Intel core 15 gen CPU @ 42 GHz with DDR7 RAM with a 32 TB NVMe SSD and liquid-cooled machine running the most bloated (GNU/)Linux distro (or Windows...) well surpasses the system requirements for running Encore, and that you will notice no difference in performance, if it can run pretty well on this AMD Quad-Core A4 processor @ 1.8 GHz
[^2]: Do not tell me that Encore is slow on Windows. [that's a fault of microsoft.](https://github.com/cmuratori/refterm/blob/main/faq.md).


# Tanglit - Installation Guide

Tanglit only supports \*nix systems so far, and was tested with Ubuntu 24.04

## from source

### prerequisites

You'll need to install some dependencies into your system

First, make sure the package repository is updated with

~~~
$ apt update
~~~

We will need to install the following dependencies:

```
libwebkit2gtk-4.1-dev build-essential clang lld llvm libayatana-appindicator3-dev librsvg2-dev libssl-dev libxdo-dev xdg-util
```
which we can do in a shell with:
~~~
$ sudo apt install build-essential \
    clang \
    lld \
    llvm \
    nodejs \
    npm \
    libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libssl-dev \
    libxdo-dev \
    xdg-utils
~~~

**Note:**
Sometimes we've seen builds fail due to other missing dependencies on slightly different systems. We've found that installing these packages solved them in some cases:
~~~
libgio-2.0-dev libdbus-1-dev
~~~


You will also need to have [Rust](https://rust-lang.org/) installed in your system.  
If you plan on using Rust apart from simply compiling Tanglit, you might want to manage rust by installing [rustup](https://rustup.rs/) and use that tool to install the rust compiler and related tooling. 

If you are following this guide in Ubuntu 24.04, then rustup can be installed via `apt`:  
~~~
$ sudo apt install rustup
~~~

Rustup can also be installed with this shell command if your package manager doesn't have rustup in its repository (requires curl)  
`$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh `

But if you just need rust to compile Tanglit, then Rust can be installed directly. Check the [archives](https://forge.rust-lang.org/infra/archive-stable-version-installers.html) page for your apropiate compiler. Keep in mind that `rustup` is the recommended install method.

#### pdf rendering
We currently use the **chrome** browser as the rendering engine to turn html into pdf documents. This requires you to have chrome installed in your system if you want to export slides and documents in pdf format.

Please follow the [official instructions to install chrome](https://www.google.com/chrome/) on your system.

### compiling

Clone this project's repository via git somewhere in your system and navigate to the `frontend` directory:
~~~
$ git clone https://github.com/TP-FIUBA-133/tanglit.git
$ cd tanglit/frontend
~~~

Then run *make* to compile and run the application (debug mode)

~~~
$ make run-dev
~~~

which will install all the required npm and rust libraries, compile them, build the application and run it.

## from prebuilt binaries

This project's github repository provides prebuilt binaries targeting Debian on x86_64 (and a few other targets). Head to the [releases](https://github.com/TP-FIUBA-133/tanglit/releases) page.  
The easiest to get started with is the *AppImage* release (targets amd64/x86_64). Download it, make it executable if necessary and execute it.
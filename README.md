###### Jimmy's Markdown Compiler for CLI Output UNFINISHED

> Markdown compiler for CLI

### Installation

> $ git clone https://github.com/LordGoatius/cli-md-compiler \
 $ cd cli-md-compiler \
 $ cargo build

You can then take the binary in target/release and add it to the path.

### Usage

The program is overall quite simple.

By running 

> $ mdcli README.md 

the program will compile the Markdown into the most human readable unicode file, and output it to stdout.

### Overview

When attempting to find a simple soltuion for editing a Markdown file that fit into my casual workflow, I struggled. I could use [Obsidian](https://obsidian.md/), but that doesn't fit in line with my CLI workflow quite as well as I would hope. I used the [grip](https://github.com/joeyespo/grip) tool, but it created a local server and used the browser, which also isn't ideal for my workflow. It also uses the GitHub syntax and API, so you know what it looks like on GitHub, but that isn't what I wanted either. I wanted something simple (like cat or batcat) that would allow me to quickly preview files within my workflow, and so I begin this project.

<div align="center">
  <h1><ruby>ラ<rp>(</rp><rt>Ra</rt><rp>)</rp>ス<rp>(</rp><rt>su</rt><rp>)</rp>ト<rp>(</rp><rt>to</rt><rp>)</rp>バ<rp>(</rp><rt>ba</rt><rp>)</rp>ス<rp>(</rp><rt>su</rt><rp>)</rp>タ<rp>(</rp><rt>ta</rt><rp>)</rp>ー</ruby><p align="center">Rustbuster</p></h1>
</div>

<a href="https://rust-lang.org">![built-with](https://github.com/0niSec/rustbuster/assets/9609041/241e8c72-0b2c-4dcc-8938-a9e7014fb5be)</a> <a href="https://discord.com/users/tyr4el">![discord](https://github.com/0niSec/rustbuster/assets/9609041/530f7982-d1d5-4bc7-9aaa-77fbd5675a04)</a>

# 🤔 What Is This?

Rustbuster is a directory enumeration tool built in the [Rust](https://rust-lang.org) programming language. It is similar to other tools like [gobuster](https://github.com/OJ/gobuster), [wfuzz](https://github.com/xmendez/wfuzz), [ffuf](https://github.com/ffuf/ffuf) and others. Inspiration very clearly came from gobuster, which is by far my most used tool in Hack the Box.

## ❓ OK, So Then Why?

I really just wanted to come up with a project that wasn't too difficult while also allowing me to develop my Rust programming skills and knowledge and to put something into my Github that was "completed". In addition to that, at the time I was working on this, I was trying to develop skills in hacking and cyber security to hopefully change careers and maybe land a job in that field.

Overall, this turned out to be a really fun project that I used to improve my knowledge of Rust, HTTP, asynchronous code, threading, and to improve my development process (please don't look at my commits 💀).

# 👓 Comparison

There really isn't a comparison between the other tools mentioned already. This tool will most likely never become what those others are. At least not by myself. I'm a full-time working parent and there is literally not enough time in the day to do my full-time job, and have enough "sit-down time" to gather up enough focus to work on any one feature. However, this tool should be considered "feature complete" and is at the very least, a minimum viable product that *does* work. It just may not have as many of the fancy bells and whistles as the other tools have. It *should* at least be good enough for CTFs.

# ✨ Features

- Multithreaded async allowing for fast scanning
- Utilizes the Rust programming language to enforce safe memory management, good coding practices and to keep it as lightweight as possible
- It has colors?

# Installation

Head over to <a href="https://github.com/0niSec/rustbuster/releases">Releases</a> and download the latest release and off you go.

> [!IMPORTANT]
> Sorry, Rustbuster does not support Windows at this time.

## Method 1
1. Download the release from Github
2. Save the binary to a directory of your choosing
3. `chmod u+x rustbuster`
4. `./rustbuster`

## Method 2
1. `cargo install rustbuster`
2. `export PATH=$PATH:~/.cargo/bin`
3. `rustbuster`

## Method 3 (Kali Linux)
1. `sudo apt install rustbuster`
2. `rustbuster`

# 💻 Usage

Rustbuster is very simple to use. The flags are the same as [gobuster](https://github.com/OJ/gobuster) simply because I did not want to reinvent the wheel, and if people were going to give this tool a try, I figured I'd use what is familiar to a lot of people and that was gobuster's syntax and flags.

```bash
./rustbuster -u <target_url> -w <wordlist> <OPTIONS>
```

# 🤝 Contributing

Like I mentioned <a href="https://github.com/0niSec/rustbuster/edit/develop/README.md#-comparison">above</a>, I don't exactly have all the time I'd like to put all my energy into this project. I can only make small improvements over a few days since I'm still learning. If you really believe in this project, find it interesting, want to add more to it, or just want a side-project of your own, then please by all means submit a Fork Request and we can work together on it. I'd welcome the assistance.

If you find a bug, please also feel free to <a href="https://github.com/0nisec/rustbuster/issues">submit an issue</a>. I cannot promise that all issues will be answered or fixed but major breaking bugs that prevent it from being used for its intended purpose (on CTF platforms), will be addressed as soon as I can. Again, if you want to contribute to fix an issue on your own, please submit a Fork Request and let's work together 🤝.

# ❗Disclaimer

This tool is meant to be used ethically in Capture the Flag programs such as [MetaCTF](https://metactf.com/), [Hack the Box](https://hackthebox.com), or [TryHackMe](https://tryhackme.com) (to name a few) or on sanctioned penetration tests that have a formal contract and drawn out engagement. Please do not use this tool on infrastructure that you do not have permission to. 

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

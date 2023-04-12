---
external: false
title: "Why I built zp?"
description: "Talks about the process behind building zp!"
date: 2023-04-12
---

For those of you who don't know, (which is going to be most of you), I recently published my very first crate; [zp](https://github.com/bahdotsh/zp). `zp` is just a simple tool to copy contents from a file. I built zp so that I can learn rust properly. And by properly, I mean learning something and understanding it in a way I can use it to build cool/useful things in the future.

The process was relatively simple than I expected and as everyone says, the toughest thing was deciding what to build. Thankfully, I didn't overthink it this time and decided to build something not complex at all. My problem statement was simple, "I need a tool to copy things from inside a file to the clipboard from my terminal". Since the problem statement was so clear, I was able to break it down into smaller parts and publish it in a day! 

I was really excited when the development reached the final stages as it was working exactly how I wanted it to work. I don't know if you know, but the following are the steps to publish your crate to [crates.io](https://crates.io/) :

  - Add a [license](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/licensing-a-repository) to your project.
  - Put up a proper `README.md` file.
  - Set your package information like version, description, license, etc. in the `Cargo.toml` file.
  - Build your project, `cargo build --release`
  - And finally, publish your project with `cargo publish` (you need to first create an account in crates.io and generate an API key to log in to your account from the terminal).

Yup, that's it! You have now published your crate. And what that means is that people will see your project and may even use it! That reality hit me a little late. I was really proud of my accomplishment and posted it everywhere I could think of. 

This is when the reality of other people using my project hit kind of hard. A couple of developers used it and as it turns out, my project doesn't work at all in Linux and Windows! I should've checked it, but I didn't. My embarrassment has never reached highs like this before and was kind of hard to deal with. 

Followed by this, an interesting phenomenon started to happen, people started giving me suggestions to improve my project. It was so refreshing and humbling to see really cool people giving me some really cool feedback! 

What that means is that I will get to learn rust a little bit more in-depth, and of course, improve my project in the process. The first obvious feedback I'm going to work on is to make my project cross-platform compatible, and will soon be dropping a blog on that later.

Also since you have read this much, do check out my [blog](https://gokuls.in/blog/)!
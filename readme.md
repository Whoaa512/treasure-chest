# treasure-chest
> Simple file caching built with Rust & AI 🦀 🦾

This project was created with the help of [ChatGPT](https://chat.openai.com/) using iterative prompts back & forth to both ask about code samples and to understand Rust.

## Running locally

```sh
cargo run
```

Then you can make HTTP PUT/GET requests to upload/download files. The files will be saved to disk in the `./uploads` folder in the project root.

```sh
curl -XPUT localhost:3030/path/bar/qux/foo.txt -H 'Content-Type: text/plain' -d "hello foo"
curl -XGET localhost:3030/path/bar/qux/foo.txt
```

## Thoughts on ChatGPT

This was a fun experiment in playing with this new AI tool and I was able to create this working server in less than 8 hours with very little prior knowledge of Rust. My day job is a general purpose infra engineer, and my favorite language is TypeScript/JS; also have familiarity with Go/Python/Haskell.

I was blown away by how well this chat bot was able to assist me to go from simple idea to working code.

Some of the very early commits are wholesale code samples from the AI but project quickly grew beyond it's capabilities to provide whole examples. Instead I changed up my prompts to ask more targeted Rust questions, think what you would normally search for as an inexperienced Rust dev but more natural language.

Overall I'm happy with the results since I'm very new to Rust and this is my first real project with it.

My hope is to iterate on creating a benchmarking suite for this little server to test against pure nginx and maybe other file server implementations.


Currently licensed under [MPL 2.0](./LICENSE) but this may change in the future depending on how much time I decide to dump into this.


I encourage you to try the same with any code project you've been putting off. It's remarkable, and I can't wait for this model to make it's way into the Codex mainline.

## Initial prompts used
These were some of the initial prompts that got me started.

- What's an example of a simple rust web server that saves files to disk?
- Could you show me an example of using the `warp` crate in rust to create a server that accepts file uploads on a POST to the `/put/:filepath` endpoint?
- How would you add an endpoint to get the `:filepath` on the `/get/:filepath` endpoint?

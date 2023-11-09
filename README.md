**P**rogressive **REST**ful framework written in [Rust](https://www.rust-lang.org/) to simplify cross-platform full-stack development. All it's tutorials(wip) are available in the [blog](https://prest.blog/) which is also [made with prest](https://prest.blog/about).

### Why & How
Initial inspiration came from [this proof-of-concept](https://github.com/richardanaya/wasm-service) - combination of a rust-based [Service Worker](https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API) compiled into [WebAssembly](https://webassembly.org/) with [HTMX](https://htmx.org/) library. This will likely sound pretty wild if you haven't worked with these technologies, but the underlying idea is simple - extend the regular [REST architecture](https://htmx.org/essays/rest-explained/) with a client-side worker that can respond to some of the requests. Thanks to the rich wasm support in rust you can easily cross-compile some of your server code into this worker. Thanks to HTMX you can easily build dynamic UIs without writing a single line of javascript. And thanks to [progressive web capabilities](https://web.dev/what-are-pwas/) this combo easily becomes a native-like installable application.

**Prest allows building full-stack web + cross-platform apps for the development cost of an HTTP/HTML server**. In it's simplicity it surpasses well established frameworks like [React Native](https://reactnative.dev/), [Flutter](https://flutter.dev/) and even more recent rust alternatives like [Tauri](https://tauri.app/) and [Dioxus](https://dioxuslabs.com/). While prest isn't nearly as developed, it's based on mature web standards and [arguably the most reliable practical language](https://edezhic.medium.com/reliable-software-engineering-with-rust-5bb4553b5d54).

On the other hand:

* Prest is still in active development and quite unstable
* Sometimes rust requires understanding of low-level details which are hidden in languages like Python or Javascript. 
* [Hypermedia might not be the right tool for your needs](https://htmx.org/essays/when-to-use-hypermedia/). 
* Web apis aren't as all-powerful as native ones, and if you need to use mobile client's OS bindings then project's complexity might increase dramatically.

So, for now it's only recommended for personal projects and experimentation.

### Getting started
To run locally you'll need the latest stable [rust toolchain](https://rustup.rs/). Most examples are supported on [replit](https://replit.com/) so you can [fork it there](https://replit.com/@eDezhic/prest) and run in the cloud. It includes [rust-analyzer](https://rust-analyzer.github.io/) and I recommend it for local development as well. Some examples require additional setup which is described in their tutorials. To build&start them use `cargo run -p EXAMPLE-NAME`

The simplest ones are [hello](https://prest.blog/hello) - casual restful server in just a couple of lines, and [hello PWA](https://prest.blog/hello-pwa) which also packs [everything necessary for the app to be installable](https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps/Guides/Making_PWAs_installable). Their tutorials describe the basic components of prest and how they fit together.  

There is also a whole bunch of tutorials that describe how you can use other things with prest:
- all kinds of databases - postgres through [seaorm](https://prest.blog/with-seaorm-postgres) or [diesel](https://prest.blog/with-diesel-postgres), sqlite through [sqlx](https://prest.blog/with-sqlx-sqlite) or [turbosql](https://prest.blog/with-turbosql-sqlite), [mongo](https://prest.blog/with-mongo-driver), [redis](https://prest.blog/with-redis-driver) and even a full rust combo [gluesql + sled](https://prest.blog/with-gluesql-sled)
- authentication, authorization, user and session management with [OpenID/OAuth](https://prest.blog/with-oauth-google)  
- other templating engines like [Askama](https://prest.blog/with-jinja-askama) which provides Jinja-like syntax
- compilation and bundling of [SASS/SCSS](https://prest.blog/with-grass-scss), [TypeScript](https://prest.blog/with-swc-typescript) and other sources in the build pipeline
- extensive and customizable [logging](https://prest.blog/with-tracing-subscriber), efficient concurrent [scraping](https://prest.blog/with-reqwest-scraper), built-in [HTTPS](https://prest.blog/with-rustls-https) encryption
- even [Large Language Models](https://prest.blog/with-candle-mistral) and [blockchain Smart Contracts](https://prest.blog/with-substrate-contract)!

You can also compile your client [into a native binary](https://prest.blog/into-native-wry) if you need access to system APIs, as well as compile the host [into WebAssembly with a system interface](https://prest.blog/into-wasi-host) to simplify devops. You can even combine the best of both worlds and [create portable binaries](https://github.com/dylibso/hermit). The range of possibilities is so wide that only C and C++ can exceed it, but rust provides much better development experience in most cases. To be fair rust ecosystem is relatively young, but it's growing rapidly and already has a suprisingly diverse set of stable libraries. 

### Under the hood 
Prest itself is a relatively thin wrapper around a whole bunch of rust libs, and it is intended to stay that way for the foreseeable future to enable frequent major changes in a pursuit of building a simple interface over an extendable foundation. So, its existance is only possible thanks to a number of brilliant projects. It includes a slightly modified forks of [maud](https://maud.lambda.xyz/) for convenient HTML templating inside of rust code and [rust-embed](https://github.com/pyrossh/rust-embed) to easily bundle assets with the compiled binaries.

It heavily utilizes and re-exports [axum](https://github.com/tokio-rs/axum), [http](https://docs.rs/http/latest/http/) and [tower](https://docs.rs/tower/latest/tower/) for ergonomic routing and other REST primitives on both server and client. Host is powered by [tokio](https://docs.rs/tokio/latest/tokio/) async runtime which provides exceptional performance, all kinds of primitives for concurrency and OS interactions, as well as [hyper](https://hyper.rs/) which provides extremely reliable HTTP operations. Error handling is simple, idiomatic and infinitely customizable thanks to [anyhow](https://github.com/dtolnay/anyhow). And there is a whole bunch of other specific utils which you can find among the [prest's dependencies](https://github.com/edezhic/prest/blob/main/Cargo.toml).

I also want to hightlight [hyperscript](https://hyperscript.org/) here because it can solve 99.99% of UI tasks in an easier and more maintainable way than JS, and it pairs well with htmx. They are not dependencies of prest but I highly recommend you to try them out together. Anyway, if you prefer good old React or other conventional front-end tooling you can use them with prest as well.

Also, there are plenty of [Web APIs](https://fugu-tracker.web.app/) available in rust thanks to [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) on the client side, like the ones that enable Progressive features of prest. There is also the [WASI](https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-intro.md) ecosystem to simplify devops and fascilitate serverless, [WebGPU](https://developer.chrome.com/blog/webgpu-io2023/) for cross-platform AI, complex UIs and games, and many other web-related tech being developed for all kinds of use cases.

### Publishing
Prest not published on [crates.io](https://crates.io/crates/prest) yet because public APIs are extremely unstable and it depends on the latest unreleased axum v0.7 changes. The current goal is to experiment as much as possible right now and publish the first (still somewhat unstable) version after axum. 

Blog is deployed to replit by compiling into the `musl` target and including binary into the repo due to [this issue](https://ask.replit.com/t/deployment-time-outs/73694). To deploy rebuild the binary with `cargo build -p blog --target x86_64-unknown-linux-musl --release` and move `target/release/serve` into the `_temp_deployment` folder.

# Make Web Great Again

What if the internet was blazingly fast again?

This repository compares the most popular web application platform to some less popular but much faster ones. If you want to find the fastest frameworks, visit the ranking [Web Framework Benchmarks](https://www.techempower.com/benchmarks/#section=data-r21). 

But overall I think new is coming, Rust is coming, I need to see how it works under the hood.

## Usage

### Node_Express part
Requirements:
- NodeJs
- npm

Steps to run server:
```bash
cd node_express/server
npm init
npm install express
// press enter till the end
node src/index.js
```

### Rust_Rocket part
Requirements:
- Rust

Steps to run server:
```bash
cd rust_rocket/server
cargo run --release
```

### Rust_Actix part
Requirements:
- Rust

Steps to run server:
```bash
cd rust_rocket/server
cargo run --release
```

### Conclusions
Running a server using Node requires a lot more steps than running it using Rust. *\<lol\>*

Some benchmarks result will be added here.

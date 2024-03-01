# xwasm

xwasm是一个以rust为基础，生成一个eDSL的合约语言，然后使用wasmtime运行该合约语言的项目；  
第一步是用来生成合约文件；第二步是使用wasmtime运行合约文件；  

1. 在contract目录，执行命令  
   cargo build --target wasm32-unknown-unknown --release  
   在contract/target/wasm32-unknown-unknown/release目录下生成一个contract.wasm文件  
   这里其实还可以使用wasm-strip工具，对生成wasm文件缩减体积  
   后缀wasm，就是合约文件  

2. 将合约文件拷贝到wasm\tests\wasmfile\下  
   在wasm文件下执行  
   cargo test --test twasm  
   即可执行成功  

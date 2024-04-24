# Rust 性能分析




sudo dtrace -c './rust-ev large-file' -o out.stacks -n 'profile-997 /execname == "rust-ev"/ { @[ustack(100)] = count(); }'

`rust-ev` 是指编译好的二进制文件


git clone git@github.com:brendangregg/FlameGraph.git 
把这个文件放到系统环境变量

stackcollapse.pl out.stacks | flamegraph.pl > out.svg

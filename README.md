# Rust Port Scanner
---

Port scanner inspired by nmap and written in rust.
Here are some simple examples of usage
```$ ./rust-port-scanner --target 192.168.0.1 -p "1,3,5,10-100" -c 500```
This will scan ports 1, 3, 5, 10, 11, ..., 100 on device with ip 192.168.0.1 and concurrency 500

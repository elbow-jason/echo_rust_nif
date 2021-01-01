# EchoRustNif

A Simple POC Rustler NIF that runs an async task "process" with the tokio runtime.

All together it's ~120 LOC of Rust and ~20 LOC of Elixir.

Based heavily on the [franz](https://github.com/scrogson/franz) project.


```elixir
iex(1)> {:ok, e} = EchoRustNif.start()
{:ok, #Reference<0.1187192475.3514957827.56096>}
iex(2)> flush()                         
:ok
iex(3)> EchoRustNif.echo(e, "hello?")   
:ok
iex(4)> flush()                      
{:echo, "hello?"}
:ok
iex(5)> EchoRustNif.stop(e)             
:ok
iex(6)> EchoRustNif.echo(e, "hello?!?!")
:ok
iex(7)> flush()                         
:ok
```

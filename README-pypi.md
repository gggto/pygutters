# pygutters

***Quick and dirty tools for the intrepid plumber***

`pygutters` is a python library which provides very
*very* basic functions using the [`gutters` library]
for building quick and dirty interprocess or network
protocols for passing around `f64`s. Sewer metaphors
included.

[`gutters` library]: https://github.com/gggto/gutters

## Installation

Run `pip install gutters`.

## Usage

`pygutters` provides two class:

- The `Gutter` class, which is based on TCP/IP, perfect for multi-computer
  setups.

- The `Duct` class, which is based on named pipes, perfect for best
  single-computer multi-process performances.

### Gutters

You may wait for a client gutter to connect like this:
```python
gutter = Gutter.accept("127.0.0.1:10100")
```
The client may then connect:
```python
gutter = Gutter.connect("127.0.0.1:10100")
```

For communication, you may:

- Send a number with `Gutter.throw`.
- Receive a number with `Gutter.pick_up`.
- Send an acknowledgement with `Gutter.hail`.
- Receive an acknowledgement with `Gutter.wait`.
- Send a number and receive an acknowledgement with `Gutter.throw_and_wait`.
- Receive a number and send an acknowledgement with `Gutter.pick_up_and_hail`.

They are used as such:
```python
gutter.throw(123.4)

branch = gutter.pick_up()
print(branch)

gutter.wait()

gutter.hail()

gutter.throw_and_wait(567.8)

branch = gutter.pick_up_and_hail()
print(branch)
```

### Ducts

`Duct`s work essentially the same than `Gutter`s, but performs better
and are limited to the local computer.

You may wait for a client duct to connect like this:
```python
duct = Duct.accept("duct_name")
```
The client may then connect:
```python
duct = Duct.connect("duct_name")
```

For communication, you may:

- Send a number with `Duct.throw`.
- Receive a number with `Duct.pick_up`.
- Send an acknowledgement with `Duct.hail`.
- Receive an acknowledgement with `Duct.wait`.
- Send a number and receive an acknowledgement with `Duct.throw_and_wait`.
- Receive a number and send an acknowledgement with `Duct.pick_up_and_hail`.

They are used as such:
```python
duct.throw(123.4)

branch = duct.pick_up()
print(branch)

duct.wait()

duct.hail()

duct.throw_and_wait(567.8)

branch = duct.pick_up_and_hail()
print(branch)
```

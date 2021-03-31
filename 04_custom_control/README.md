# 04 Custom control

This example builds a window with a custom control inside of it. The custom control cursor is not the traditional arrow, but a cross instead. Every time you click, a new line is drawn.

Also, a counter in the title bar is incremented at each click. The custom control stores a callback sent by the main window, and this callback is fired at each click.

![Example 04](screen.gif)

To compile and run:

```
cargo run
```

To generate the final executable:

```
cargo build --release
```

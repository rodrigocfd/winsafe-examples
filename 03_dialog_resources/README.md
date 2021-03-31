# 03 Dialog resources

In this example we create a main window and, when the button is clicked, a modal window.

Instead of creating these windows programmatically, we load dialogs from the `resources/example03.res` resource file, which was created with the resource editor of Visual Studio 2019. You can edit this file with any Win32 resource editor.

Note how the modal receives a text and returns another.

![Example 03](screen.gif)

To compile and run:

```
cargo run
```

To generate the final executable:

```
cargo build --release
```

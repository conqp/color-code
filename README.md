# ansi-color-codec
Encode arbitrary byte sequences with ANSI background colors.

[Showcase](https://user-images.githubusercontent.com/3766192/201550552-aafad8df-9935-4cc8-a1e5-05975c7b3537.webm)

## Usage
You can use the program as a library or as a command line utility.

### Library
The library provides a trait `ColorCodec<T>` that allows arbitrary byte 
iterators 
(`Iterator<Item = u8>`) to encode their bytes as ANSI background color codes.
Therefor the trait provides the functions `ansi_color_encode()` and 
`ansi_color_decode()`.

### Command line utility
You can encode bytes or decode color codes by passing them to 
`ansi-color-codec`'s STDIN:

```shell
$ echo "I use Arch btw" | ansi-color-codec
$ echo "I use Arch btw" | ansi-color-codec | ansi-color-codec -d
```

For more options, see `ansi-color-codec -h`.

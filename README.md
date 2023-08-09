# qrlink

This utility encodes a URL / link into a QR code. Pre-built binaries for a [command-line interface](https://github.com/davidk/qrlink/releases) are available.

## Usage 

`$ ./qrlink https://github.com/davidk/qrlink/releases/`

### To generate a QR code image

`$ ./qrlink https://github.com/davidk/qrlink/releases --imagefile releases.png`

![QR Code image](/img/releases.png)

### More options

```bash
USAGE:
    qrlink [ https://example.com | --ask ] [ --imagefile (output_name.png) --scale 10 |
    --svg | --svgfile (output_name.svg) ]

FLAGS:
        --svg        Emit the QR code as an SVG (to standard output)
    -a, --ask        Ask for URL/link instead of getting it through the command-line
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --scale <scale>             QR code scaling factor. Applies to imagefile 
                                    output only. [default: 10]
        --quietzone <quiet_zone>    QR code: The size of the quiet zone/border to 
                                    apply to the final QR code [default: 2]
        --imagefile <image_file>    The name of the file to save to (e.g. --imagefile qr.png). 
                                    Formats: [png, jpg, bmp]
        --svgfile <svg_file>        Save the QR code to a file (SVG formatted)

ARGS:
    <link>    URL (or text) to convert to a QR code

```

#### Building

Pre-built releases are provided on GitHub, but for development, or to build your own from source (after installing the [Rust toolchain](https://www.rust-lang.org/tools/install):

    cargo build --release

### See also

* [Format documentation, from zxing/zxing](https://github.com/zxing/zxing/wiki/Barcode-Contents)

* [qrcodegen, via project nayuki](https://docs.rs/crate/qrcodegen/1.4.0)

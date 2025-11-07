# OuDia to KDL Converter

Convert OuDia (.oud/.oud2) files to KDL.

## Web Version

[A web-based converter is available here](https://wensimehrp.github.io/oudia-to-kdl/). Simply paste your OuDia content and click "Convert" to get the KDL output.

## CLI Version

If you don't like the web version, there is also a CLI version available. You can run the converter as follows:

```bash
cargo run --release -- <input_file.oud2> <output_file.kdl>
```

By default, if there is no input file specified, it reads from `stdin`.

## Why KDL?

`.oud`/`.oud2` files' format is surprisingly simple, yet under-documented. Its structure is not like any of those popular data serialization formats such as JSON, YAML, TOML, etc., since all of them don't allow duplicate keys in the same scope. [The KDL format](https://kdl.dev/), on the other hand, allows duplicate keys, which makes it a good fit for representing OuDia files.

KDL is also human-readable and writable, making it easier to inspect and modify the converted data. For those who want to handle it via code, there are many KDL parsers available in various programming languages.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

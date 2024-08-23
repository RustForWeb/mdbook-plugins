# Trunk

Plugin which bundles packages using Trunk and includes them as iframes.

## Example

```toml,trunk
package = "book-example"
features = ["button"]
```

-   [Book source code](https://github.com/RustForWeb/mdbook-plugins/tree/main/book)
-   [Example source code](https://github.com/RustForWeb/mdbook-plugins/tree/main/book-example)

## Installation

```shell
cargo install mdbook-trunk
```

## Configuration

Add the preprocessor and renderer to `book.toml`. Note that the HTML renderer is also required.

```toml
[preprocessor.trunk]

[output.html]

[output.trunk]
```

## Usage

Define a Trunk include as follows:

````markdown
```toml,trunk
# Package to build, must be in the current workspace.
package = "book-example"

# Features to enable for the package.
features = ["button"]
```
````

The following options are available:

```toml
# Package to build, must be in the current workspace.
package = "book-example"

# Features to enable for the package.
features = ["button"]

# URL query (optional).
# The leading question mark is optional.
url_query = "?key=value"

# URL fragment (optional).
# The leading hash is optional.
url_fragment = "#header"
```

## Building

1. Build the book using `mdbook build`. This will output multiple directories in `book/build`.
2. Combine the build outputs using `mdbook-trunk combine`. This will combine the output directories in `book/dist`.
3. Serve the `book/dist` directory.

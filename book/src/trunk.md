# Trunk

Plugin which bundles packages using Trunk and includes them as iframes.

## Example

```toml,trunk
package = "book-example"
features = ["button"]
files = ["src/button.rs"]
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

Add the additional CSS and JS files to the book with the following command.

```shell
mdbook-trunk install
```

Add the additional CSS and JS files to the HTML renderer in `book.toml`.

```toml
[output.html]
additional-css = ["theme/trunk.css"]
additional-js = ["theme/trunk.js"]
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

# Sources files to render (optional).
# Relative to the package root.
files = ["src/button.rs"]

# URL query for the iframe URL (optional).
# The leading question mark is optional.
url_query = "?key=value"

# URL fragment for the iframe URL (optional).
# The leading hash is optional.
url_fragment = "#header"

# HTML attributes for the iframe (optional).
[attributes]
allow = "fullscreen"
```

## Building

1. Build the book using `mdbook build`. This will output multiple directories in `book/build`.
2. Combine the build outputs using `mdbook-trunk combine`. This will combine the output directories in `book/dist`.
3. Serve the `book/dist` directory.

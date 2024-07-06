# Mermaid Beans

A mermaid diagram generator for Spring beans.

```mermaid
graph TD;
classDef transparent fill:#0000
subgraph demo
    demoApplication["com.example.demo.DemoApplicationSpringCGLIB0"]
    firstService["com.example.demo.FirstService"]
    myController["com.example.demo.MyController"]
    myController --> firstService
    myController --> secondService
    secondService["com.example.demo.SecondService"]
end
class demo transparent
```

## Installation

```text
cargo install --git https://github.com/rudsvar/mermaid-beans
```

## Usage

```text
$ mermaid-beans --help
Usage: mermaid-beans [OPTIONS] [URI]

Arguments:
  [URI]  URL or file path. If not provided, reads from stdin

Options:
  -p, --package-filter <PACKAGE_FILTER>  Choose beans to include by package
  -h, --help                             Print help
```

# Mermaid Beans

A mermaid diagram generator for Spring beans.

```mermaid
graph TD;
classDef transparent fill:#0000
subgraph application
direction LR    demoApplication[<div title="com.example.demo.DemoApplicationSpringCGLIB0">demoApplication</div>]
    itemController[<div title="com.example.demo.feature.item.ItemController">itemController</div>]
    itemController --> itemService
    itemRepository[<div title="com.example.demo.feature.item.ItemRepository">itemRepository</div>]
    itemRepository --> jpa.namedqueries#0
    itemRepository --> jpa.ItemRepository.fragments#0
    itemRepository --> jpaSharedEM_entityManagerFactory
    itemRepository --> jpaMappingContext
    itemService[<div title="com.example.demo.feature.item.ItemService">itemService</div>]
    itemService --> itemRepository
    securityConfig[<div title="com.example.demo.infra.SecurityConfigSpringCGLIB0">securityConfig</div>]
end
class application transparent
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

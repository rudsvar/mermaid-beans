# Mermaid Beans

A mermaid diagram generator for Spring beans.

```mermaid
graph TD;
classDef transparent fill:#0000
subgraph application
direction LR    demoApplication(<b>demoApplication</b><div style="color: gray">com.example.demo.DemoApplicationSpringCGLIB0</div>)
    itemController(<b>itemController</b><div style="color: gray">com.example.demo.feature.item.ItemController</div>)
    itemController --> itemService
    itemRepository(<b>itemRepository</b><div style="color: gray">com.example.demo.feature.item.ItemRepository</div>)
    itemRepository --> jpa.namedqueries#0
    itemRepository --> jpa.ItemRepository.fragments#0
    itemRepository --> jpaSharedEM_entityManagerFactory
    itemRepository --> jpaMappingContext
    itemService(<b>itemService</b><div style="color: gray">com.example.demo.feature.item.ItemService</div>)
    itemService --> itemRepository
    securityConfig(<b>securityConfig</b><div style="color: gray">com.example.demo.infra.SecurityConfigSpringCGLIB0</div>)
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

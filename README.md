# Mermaid Beans

A mermaid diagram generator for Spring beans.

```mermaid
graph TD;
classDef transparent fill:#0000
subgraph application
direction LR    demoApplication[demoApplication]
    click demoApplication callback "com.example.demo.DemoApplicationSpringCGLIB0"
    itemController[itemController]
    click itemController callback "com.example.demo.feature.item.ItemController"
    itemController --> itemService
    itemRepository[itemRepository]
    click itemRepository callback "com.example.demo.feature.item.ItemRepository"
    itemRepository --> jpa.namedqueries#0
    itemRepository --> jpa.ItemRepository.fragments#0
    itemRepository --> jpaSharedEM_entityManagerFactory
    itemRepository --> jpaMappingContext
    itemService[itemService]
    click itemService callback "com.example.demo.feature.item.ItemService"
    itemService --> itemRepository
    securityConfig[securityConfig]
    click securityConfig callback "com.example.demo.infra.SecurityConfigSpringCGLIB0"
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

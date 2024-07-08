# Mermaid Beans

A mermaid diagram generator for Spring beans.

```mermaid
graph TD;
classDef transparent fill:#0000
subgraph application
    demoApplication["demoApplication"]
    itemController["itemController"]
    itemController --> itemService
    itemRepository["itemRepository"]
    itemRepository --> jpa.named-queries#0
    itemRepository --> jpa.ItemRepository.fragments#0
    itemRepository --> jpaSharedEM_entityManagerFactory
    itemRepository --> jpaMappingContext
    itemService["itemService"]
    itemService --> itemRepository
    securityConfig["securityConfig"]
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

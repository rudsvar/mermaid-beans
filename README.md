# Mermaid Beans

A mermaid diagram generator for Spring beans.

```mermaid
graph TB;
classDef transparent fill:#0000;
subgraph "application";
    direction TB;
    demoApplication(
        <b>demoApplication</b>
        <div style="color: gray">com.example.demo.DemoApplicationSpringCGLIB0<br></div>
    )
    entityManagerFactory(
        <b>entityManagerFactory</b>
        <div style="color: gray">jdk.proxy4.Proxy137<br></div>
    )
    entityManagerFactory --> entityManagerFactoryBuilder
    entityManagerFactoryBuilder(
        <b>entityManagerFactoryBuilder</b>
        <div style="color: gray">org.springframework.boot.orm.jpa.EntityManagerFactoryBuilder<br></div>
    )
    itemController(
        <b>itemController</b>
        <div style="color: gray">com.example.demo.feature.item.ItemController<br></div>
    )
    itemController --> itemService
    itemRepository(
        <b>itemRepository</b>
        <div style="color: gray">com.example.demo.feature.item.ItemRepository<br></div>
    )
    itemRepository --> jpaSharedEM_entityManagerFactory
    itemService(
        <b>itemService</b>
        <div style="color: gray">com.example.demo.feature.item.ItemService<br></div>
    )
    itemService --> itemRepository
    jpaSharedEM_entityManagerFactory(
        <b>jpaSharedEM_entityManagerFactory</b>
        <div style="color: gray">jdk.proxy4.Proxy141<br></div>
    )
    jpaSharedEM_entityManagerFactory --> entityManagerFactory
    securityConfig(
        <b>securityConfig</b>
        <div style="color: gray">com.example.demo.infra.SecurityConfigSpringCGLIB0<br></div>
    )
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
  -t, --type <TYPE>            Include beans by type (package)
  -n, --name <NAME>            Include beans by name
  -d, --direction <DIRECTION>  Direction of the graph [default: LR]
  -h, --help                   Print help
```

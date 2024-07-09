# Mermaid Beans

A mermaid diagram generator for Spring beans.

```mermaid
graph TD;
classDef transparent fill:#0000
subgraph "application"
  direction LR
    basicErrorController(<b>basicErrorController</b><div style="color: gray">org.springframework.boot.autoconfigure.web.servlet.error.BasicErrorController</div>)
    conversionServicePostProcessor(<b>conversionServicePostProcessor</b><div style="color: gray">org.springframework.security.config.crypto.RsaKeyConversionServicePostProcessor</div>)
    entityManagerFactory(<b>entityManagerFactory</b><div style="color: gray">jdk.proxy4.Proxy137</div>)
    entityManagerFactory --> entityManagerFactoryBuilder
    entityManagerFactoryBuilder(<b>entityManagerFactoryBuilder</b><div style="color: gray">org.springframework.boot.orm.jpa.EntityManagerFactoryBuilder</div>)
    itemController(<b>itemController</b><div style="color: gray">com.example.demo.feature.item.ItemController</div>)
    itemController --> itemService
    itemRepository(<b>itemRepository</b><div style="color: gray">com.example.demo.feature.item.ItemRepository</div>)
    itemRepository --> jpaSharedEM_entityManagerFactory
    itemService(<b>itemService</b><div style="color: gray">com.example.demo.feature.item.ItemService</div>)
    itemService --> itemRepository
    jpaSharedEM_entityManagerFactory(<b>jpaSharedEM_entityManagerFactory</b><div style="color: gray">jdk.proxy4.Proxy141</div>)
    jpaSharedEM_entityManagerFactory --> entityManagerFactory
    metricsRepositoryMethodInvocationListener(<b>metricsRepositoryMethodInvocationListener</b><div style="color: gray">org.springframework.boot.actuate.metrics.data.MetricsRepositoryMethodInvocationListener</div>)
    metricsRepositoryMethodInvocationListener --> org.springframework.boot.actuate.autoconfigure.metrics.data.RepositoryMetricsAutoConfiguration
    metricsRepositoryMethodInvocationListenerBeanPostProcessor(<b>metricsRepositoryMethodInvocationListenerBeanPostProcessor</b><div style="color: gray">org.springframework.boot.actuate.autoconfigure.metrics.data.MetricsRepositoryMethodInvocationListenerBeanPostProcessor</div>)
    mvcConversionService(<b>mvcConversionService</b><div style="color: gray">org.springframework.boot.autoconfigure.web.format.WebConversionService</div>)
    org.springframework.boot.actuate.autoconfigure.metrics.data.RepositoryMetricsAutoConfiguration(<b>org.springframework.boot.actuate.autoconfigure.metrics.data.RepositoryMetricsAutoConfiguration</b><div style="color: gray">org.springframework.boot.actuate.autoconfigure.metrics.data.RepositoryMetricsAutoConfiguration</div>)
    org.springframework.boot.autoconfigure.security.servlet.UserDetailsServiceAutoConfiguration(<b>org.springframework.boot.autoconfigure.security.servlet.UserDetailsServiceAutoConfiguration</b><div style="color: gray">org.springframework.boot.autoconfigure.security.servlet.UserDetailsServiceAutoConfiguration</div>)
    preserveErrorControllerTargetClassPostProcessor(<b>preserveErrorControllerTargetClassPostProcessor</b><div style="color: gray">org.springframework.boot.autoconfigure.web.servlet.error.ErrorMvcAutoConfigurationPreserveErrorControllerTargetClassPostProcessor</div>)
    simpleControllerHandlerAdapter(<b>simpleControllerHandlerAdapter</b><div style="color: gray">org.springframework.web.servlet.mvc.SimpleControllerHandlerAdapter</div>)
    viewControllerHandlerMapping(<b>viewControllerHandlerMapping</b><div style="color: gray">org.springframework.web.servlet.HandlerMapping</div>)
    viewControllerHandlerMapping --> mvcConversionService
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

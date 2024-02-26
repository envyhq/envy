# nv compiler

## Compilation flow

> Character stream -> grapheme stream -> Token stream -> Syntax tree -> Client code

### Character stream

> var secret: string

```rs
["v", "a", "r", " ", "s", "e", "c", "r", "e", "t", ":", " ", "s", "t", "r", "i", "n", "g"]
```

### Grapheme stream

A grapheme is the smallest unit in language. A char to your average programming language is a single unicode character code point. The difference is that a grapheme is a single character as understood by humans. For example, an emoji like 💆🏽‍♀ is made of multiple unicode characters but understood as one. This is also true for `\r\n` for example that is 2 unicode characters that represent a single newline.

> var 🔑: string

```rs
["v", "a", "r", " ", "🔑", ":", " ", "s", "t", "r", "i", "n", "g"]
```

### Token stream

> var something: string
>
> pub var something: int
>
> pub var something: url {
> description = "wow so useful, thanks!"
> protocol = "https"
> }

```rs
[
Token { kind: "VarKeyword" }, Token { kind: "Identifier", value: "something" }, Token { kind: "VarAssignmentColon" }, Token { kind: "VarType", value: "string" },
Token { kind: "PubKeyword" }, Token { kind: "VarKeyword" }, Token { kind: "Identifier", value: "something" }, Token { kind: "VarAssignmentColon" }, Token { kind: "VarType", value: "int" },
Token { kind: "VarKeyword" }, Token { kind: "Identifier", value: "something" }, Token { kind: "VarAssignmentColon" }, Token { kind: "VarType", value: "url" }, Token { kind: "BlockOpenCurly" }
Token { kind: "AttributeName", value: "description" }, Token { kind: "AttributeAssignmentEquals" }, Token { kind: "StringLiteral", value: "wow so useful, thanks!" },
Token { kind: "AttributeName", value: "protocol" }, Token { kind: "AttributeAssignmentEquals" }, Token { kind: "StringLiteral", value: "https" },
Token { kind: "BlockCloseCurly" }
]
```

### Syntax tree

> var secret: string
>
> var some_url_with_api_key: url
>
> pub var some_public_value: int
>
> var some_url_with_api_key_and_description: url {
> description = "wow so useful, thanks!"
> protocol = "https"
> }

```rs
SourceFile {
  declarations: [
    Declaration {
      identifier: Identifier { name: "secret" },
      type: "string",
      attributes: None,
      modifiers: None,
    },

    Declaration {
        identifier: Identifier { name: "some_url_with_api_key" },
        type: "url",
        attributes: None,
        modifiers: None,
    },

    Declaration {
        identifier: Identifier { name: "some_public_value" },
        type: "int",
        attributes: None,
        modifiers: [PublicVar { .. }]
    },

    Declaration {
        identifier: Identifier { name: "some_url_with_api_key_and_description" },
        type: "url",
        modifiers: None,
        attributes:  [
            Attribute {
                name = "description",
                value = "wow so useful, thanks!"
            },
            Attribute {
                name = "protocol",
                value = "https"
            }
          ]
    }
    ]

}
```

### Client code

`nv` aims to compile into multiple target languages to make integration easy and to allow nv to be opinionated (and useful) throughout the development lifecycle.

Currently you don't see any references to "providers", here is your first introduction. Still WIP, providers allow developers to specify the data sources of various config values. These could be pulled from a central nv config service, or it could be a third party or external secret management system like AWS Secrets Manager or HasiCorp Vault.

By generating client code, we can also handle the fetching of these values at runtime, which is crucial in particular for private values. A private value provided at any point before the runtime is likely easily recoverable by at least a dev team, which makes it recoverable by bad actors.

We could also generate other declarative formats like JSON, YAML, dotenv, etc.

#### TypeScript

```ts
export const secret = () => "some_string_at_build_time";
export const moduleName = {
  otherSecret: () => "some_other_string_at_jit_time",
};
export const asyncModuleName = {
  otherAsyncSecret: async () =>
    await getProvider("some_provider").getValue("other_async_runtime_secret"),
};

const providers = {
  some_provider: class SomeProvider {
    async getValue(key: string) {
      return "value_from_provider";
    }
  },
};

export const getProvider = async (providerName: string) => {
  return new providers[providerName]();
};
```

#### Rust

```rs
pub fn secret() -> String {
"some_string_at_build_time".to_string()
}

pub struct ModuleName {
pub fn other_secret() -> String {
"some_other_string_at_build_time".to_string()
}

    pub async fn other_async_secret() -> Result<String> {
        get_provider("some_provider").get_value("other_async_scret").await
    }

}

struct SomeProvider {
async fn get_value(&self, key: &str) -> Result<String> {
Ok("value_from_provider".to_string())
}
}

fn get\*provider(provider_name: &str) -> Result<Box<dyn Provider>, Error> {
match provider_name {
"some_provider" => Ok(Box::new(SomeProvider)),

- => Err(Error::new(format!("Provider {} not found", provider_name)))
  }
  }
```

##### Rust blocking

For use in syncronous programs, a blocking/non-async/non-tokio version of nv rust client.

```rs
// TBC
```

#### Python

> :warning: need confirmation on this, was generated by gpt4, would also like a mojo flavour. I'll refresh my python soon.

```py
import asyncio

def secret():
return "some_string_at_build_time"

class ModuleName:
@staticmethod
def other_secret():
return "some_other_string_at_jit_time"

class SomeProvider:
async def get_value(self, key):
return "value_from_provider"

providers = {
"some_provider": SomeProvider
}

async def get_provider(provider_name):
provider_class = providers.get(provider_name)
if provider_class:
return provider_class()
else:
raise ValueError(f"Provider {provider_name} not found")

class AsyncModuleName:
@staticmethod
async def other_async_secret():
provider = await get_provider('some_provider')
return await provider.get_value('other_async_runtime_secret')
```

#### Go

```go
// TBC
```

#### C

```c
// TBC
```

#### Swift

```swift
// TBC
```

#### Java

```java
// TBC
```

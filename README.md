# rust-gql-sample  

async_graphql + sqlxでGraphQLサーバを軽くいじるための環境。  

## 用意するもの  
---
- Rust  
- Docker  
  
## ツール類  
---  
  
[cargo-watch](https://crates.io/crates/cargo-watch)  
```$ cargo install cargo-watch```  
  
[sqlx-cli](https://crates.io/crates/sqlx-cli)  
```$ cargo install sqlx-cli```  

## 起動  
--- 
まず.env.exampleを参考に.envを置いてください。  

```$ docker-compose up -d```  
```$ sqlx migrate run```  
```$ cd backend ```  
```$ cargo watch -x run```  

以上で8088にサーバが立ち上がります。  
localhost:8088/graphqlにアクセスすると、GraphQL Playgroundが立ち上がります。  

## TODO  
- [ ] Readme書く
- [ ] Dockerに入れる

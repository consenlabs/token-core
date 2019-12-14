# API
Token Core X （下称TCX) 提供了类似RPC的机制方便与 Java/Swift 等语言通讯。

## API 接口说明
TCX 提供了统一的 `Buffer call_tcx_api(Buffer buf);` C接口。参数和返回值为按照 Protobuf 序列化后的字节数组。 Buffer 为内部定义结构体，主要用来方便对字节数组的包装。
在实际使用中所有的方法都会被封装入统一的`Action API`:

```protobuf
message TcxAction {
    string method = 1;
    google.protobuf.Any param = 2;
}
```
`method`字段标明需要调用的方法。 param 为实际目标方法的请求参数，如导入助记词`method`为:`hd_store_import`，实际参数类型为`HdStoreImportParam`。`HdStoreImportParam`参数声明如下：

```protobuf
message HdStoreImportParam {
    string chainType = 1;
    string mnemonic = 2;
    string password = 3;
    string path = 4;
    string source = 5;
    string name = 6;
    string network = 7;
    string segWit = 8;
    string passwordHint = 9;
    bool overwrite = 10;
}
```
实际调用成功之后会返回 WalletResult 类型。完整的示例参见[handler.rs](../tcx/src/handler.rs)

## 开发说明
目前为了方便统一管理，所有proto文件全部放入`tcx-proto`项目内管理。目前常用的通讯参数如 api.proto, api_param.proto 已内置。    
对于链的开发者，因为每个链需要签名结构不同，需要自行编写 _chain_.proto 并且定义链相关的TransactionInput 和 TransactionOutput。    
TransactionInput 将作为SignTxParam中的input字段传入。如需要其他字段也可以放入其中。示例参见[btc-fork.proto](../tcx-proto/src/btc-fork.proto), [handler.rs#sign_tx](../tcx/src/handler.rs)。    
编写完成之后配置`tcx-proto`中`build.rs`文件，将新定义的结构编译到链所在的package中即可使用。    

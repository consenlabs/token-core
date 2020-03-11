//
//  UserData.swift
//  iOSExample
//
//  Created by xyz on 2019/10/23.
//  Copyright © 2019 consenlabs. All rights reserved.
//


import SwiftUI
import Combine
import SwiftProtobuf
import TokenCoreX

final class UserData: ObservableObject  {
  @Published var operations = [
    Operation(id: "INIT", desc: "Init TokenCoreX"),
    Operation(id: "CREATE_WALLET", desc: "Create Wallet"),
    Operation(id: "IMPORT_WALLET", desc: "Import Wallet"),
    Operation(id: "SIGN_TX", desc: "Sign Transaction"),
    Operation(id: "CLEAN_WALLETS", desc: "Clean All Test Wallets"),
  ]
  
  @Published var operationParams: String = ""
  @Published var operationResult: String = ""
  @Published var isLoading: Bool = false
  @Published var loadingLabel: String = ""
  
  private var apiRequest: String = ""
  private var apiResult: String = ""
  
  
  static var IsInitialized = false;
  
  static let shared = UserData()
  
  var walletsDirectory: URL {
    let walletsPath = "\(NSHomeDirectory())/Documents/wallets"
    var walletsDirectory = URL(fileURLWithPath: walletsPath)
    
    do {
      if !FileManager.default.fileExists(atPath: walletsPath) {
        try FileManager.default.createDirectory(atPath: walletsDirectory.path, withIntermediateDirectories: true, attributes: nil)
        var resourceValues = URLResourceValues()
        resourceValues.isExcludedFromBackup = true
        try walletsDirectory.setResourceValues(resourceValues)
      }
    } catch let err {
      debugPrint(err)
    }
    
    return walletsDirectory
  }
  
  
  func performOperation(_ op: Operation) {
    switch op.id {
    case "INIT":
      doWorkBackground("Initializing", hardWork: initTokenCoreX)
    case "CREATE_WALLET":
      doWorkBackground("Creating wallet", hardWork: createWallet)
      return
    case "IMPORT_WALLET":
      doWorkBackground("Importing wallet", hardWork: importWallet)
      return
    case "SIGN_TX":
      doWorkBackground("Signing Transaction", hardWork: signTransaction)
      return
    case "CLEAN_WALLETS":
      doWorkBackground("Cleaning Wallets", hardWork: cleanWallets)
      return
    default:
      undefinedOp()
    }
  }
  
   func wrap_action(_ data: Data, method: String) throws -> Data {
     var action = Api_TcxAction()
     action.method = method;
     var paramAny = Google_Protobuf_Any()
     paramAny.typeURL = "imtoken.hd_store_import_param";
     paramAny.value = data
     action.param = paramAny
     return try action.serializedData()
   }
  
  func initTokenCoreX() {
    if UserData.IsInitialized {
      return
    }

    var fileDir = walletsDirectory.absoluteString.dropFirst(7)
    if fileDir.last == "/" {
      fileDir = fileDir.dropLast()
    }
    
    let param = [
      "fileDir": String(fileDir),
      "xpubCommonKey128": "B888D25EC8C12BD5043777B1AC49F872",
      "xpubCommonIv": "9C0C30889CBCC5E01AB5B2BB88715799"
    ]
    self.apiRequest = prettyPrintJSON(param);
    init_token_core_x(prettyPrintJSON(param))
    
    self.apiResult = ""
  }
  
  func createWallet() {
    do {
      var param = Api_HdStoreCreateParam()
      param.name = "test_create"
      param.password = "imToken"
      param.passwordHint = "imtoken"
      
      self.apiRequest = try param.jsonString()
      var data = try wrap_action(try param.serializedData(), method: "hd_store_create")
      let retData = try callTokenCoreXApi(&data)
      let ret = try Api_WalletResult(serializedData: retData)
      self.apiResult = try ret.jsonString()
    } catch {
      print(error)
    }
  }
  
 
  
  func importWallet() {
    do {
      var param = Api_HdStoreImportParam()
          param.chainType = "BITCOINCASH"
          param.mnemonic = "inject kidney empty canal shadow pact comfort wife crush horse wife sketch"
          param.network = "MAINNET";
          param.path = "m/44'/145'/0'/0/0"
          param.segWit = "NONE"
          param.overwrite = true
          param.password = "imToken"

      var data = try wrap_action(try param.serializedData(), method: "hd_store_import")
          let result = try! callTokenCoreXApi(&data)
          let importResult = try! Api_WalletResult(serializedData: result)
          self.apiResult = try! importResult.jsonString()
    } catch {
      print(error)
    }
    
  }

  
  func signTransaction() {
    importWallet()
    
    let ret = try! JSONSerialization.jsonObject(with: self.apiResult.data(using: .utf8)!, options: .allowFragments) as! [String: Any]
    
    do {
      var param = Api_SignParam()
      param.id = ret["id"] as! String
      param.password = "imToken"
      param.chainType = "BITCOINCASH"
      param.address = "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r"
      
      self.apiRequest = try param.jsonString()
      var utxo = Transaction_Utxo()
      utxo.txHash = "09c3a49c1d01f6341c43ea43dd0de571664a45b4e7d9211945cb3046006a98e2"
      utxo.vout = 0
      utxo.amount = 100000
      utxo.address = "qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r"
      utxo.scriptPubKey = "76a91488d9931ea73d60eaf7e5671efc0552b912911f2a88ac"
      utxo.derivedPath = "0/0"
      
      var input = Transaction_BtcForkTxInput()
      input.amount = 93454
      input.to = "qq40fskqshxem2gvz0xkf34ww3h6zwv4dcr7pm0z6s"
      input.fee = 6000
      input.changeIdx = 1;
      input.network = "MAINNET"
      input.segWit = "NONE"
      
      input.unspents = [utxo]
      
      var txInAny = Google_Protobuf_Any()
      txInAny.typeURL = "imtoken.tx"
      txInAny.value = try input.serializedData()
      param.input = txInAny
      
      
      var data = try wrap_action(try param.serializedData(), method: "sign_tx")
      let retData = try callTokenCoreXApi(&data)
      let ret = try Transaction_BtcForkSignedTxOutput(serializedData: retData)
      self.apiResult = try ret.jsonString()
    } catch {
      print(error)
    }
  }
  
  func cleanWallets() {
    let fileManager = FileManager.default
    print(walletsDirectory.absoluteString)
    do {
      let filePaths = try fileManager.contentsOfDirectory(atPath: walletsDirectory.absoluteString)
        for filePath in filePaths {
          try fileManager.removeItem(atPath: walletsDirectory.absoluteString + filePath)
        }
    } catch {
        print("Could not clear temp folder: \(error)")
    }
//    self.apiResult = prettyPrintJSON(param);
    self.apiResult = "All files clean"
  }
  
  func undefinedOp() {
    operationParams = "Undefined Operation"
    operationResult = "";
  }
  
  
  private func doWorkBackground(_ workTip: String, hardWork: @escaping () -> Void) {
//    let hud = MBProgressHUD.showAdded(to: self.view, animated: true)
//    hud.label.text = workTip
    self.loadingLabel = workTip
    self.isLoading = true
    DispatchQueue.global().async {
      hardWork()
      
      DispatchQueue.main.async {
        self.isLoading = false
        self.operationParams = self.apiRequest
        self.operationResult = self.apiResult
//        MBProgressHUD.hide(for: self.view, animated: true)
//        self.presentResult(self.requestResult)
//        self.operationResult =
      }
    }
  }

  
  
  private func callTokenCoreXApi(_ data: inout Data) throws -> Data {
    var returnData = Data();
    
    clear_err()
    
    let retPointer = call_tcx_api(data.hexEncodedString())!
    let errPointer = get_last_err_message()!
    defer {
      free_const_string(retPointer)
      free_const_string(errPointer)
    }
    let err = String(cString: errPointer)
    let ret = String(cString: retPointer)
    if !err.isEmpty {
      throw err
//      reject("", err, nil)
//      return
    }
    
    return dataWithHexString(ret)
    
//    let buf = callTokenCoreXApi(data)
//    let buf = call_tcx_api(data.)
//        data.withUnsafeMutableBytes{  [ bytesLen = data.count ] (bytes: UnsafeMutablePointer<UInt8>) -> Void in
//             //Use `bytes` inside this closure
//          let buf = Buffer(data: bytes, len: UInt(bytesLen))
//          clear_err()
//          let retBuf = call_tcx_api(buf)
//          defer {
//            free_buf(retBuf)
//          }
//          let errBuf = get_last_err()
//
//          if errBuf.len > 0 {
//            returnData = Data(bytes: errBuf.data, count: Int(errBuf.len))
//            free_buf(errBuf)
//          } else {
//            returnData = Data(bytes: retBuf.data, count: Int(retBuf.len))
//        }
//        }
//    return returnData
    }
  
}

struct Operation: Identifiable {
  let id: String
  let desc: String
}


extension String: Error {}

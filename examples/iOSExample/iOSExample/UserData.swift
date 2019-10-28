//
//  UserData.swift
//  iOSExample
//
//  Created by xyz on 2019/10/23.
//  Copyright © 2019 consenlabs. All rights reserved.
//


import SwiftUI
import Combine

final class UserData: ObservableObject  {
  @Published var operations = [
    Operation(id: "INIT", desc: "Init TokenCoreX"),
    Operation(id: "CREATE_WALLET", desc: "Create Wallet"),
    Operation(id: "IMPORT_WALLET", desc: "Import Wallet"),
    Operation(id: "FIND_WALLET", desc: "Find Wallet"),
    Operation(id: "EXPORT_MNEMONIC", desc: "Export Mnemonic"),
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
    case "FIND_WALLET":
      doWorkBackground("Finding wallet", hardWork: findWallet)
      return
    case "IMPORT_WALLET":
      doWorkBackground("Importing wallet", hardWork: importWallet)
      return
    case "EXPORT_MNEMONIC":
      doWorkBackground("Exporting Mnemonic", hardWork: exportMenmonic)
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
  
  func createWallet() {
    
    let param = [
        "name": "createWalletTest",
        "password": "Insecure Password",
        "passwordHint": "Insecure Password",
        "source": "MNEMONIC"
    ]
    self.apiResult = prettyPrintJSON(param);
    self.apiResult = try! callTokenCoreXApi(param, create_wallet)
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
  
  func findWallet() {
    let param: [String: Any] = [
      "chainType":"BITCOINCASH",
      "mnemonic":"blind gravity card grunt basket expect garment tilt organ concert great critic",
      "network":"MAINNET",
      "path":"m/44'/145'/0'/0/0",
      "segWit":"NONE"
    ]
    self.apiResult = prettyPrintJSON(param);
    self.apiResult = try! callTokenCoreXApi(param, find_wallet_by_mnemonic)
  }
  
  func importWallet() {
    let param: [String: Any] = [
      "chainType":"BITCOINCASH",
      "mnemonic":"inject kidney empty canal shadow pact comfort wife crush horse wife sketch",
      "name":"BCH-Wallet-1",
      "network":"MAINNET",
      "overwrite":true,
      "password":"Insecure Password",
      "passwordHint":"",
      "path":"m/44'/145'/0'/0/0",
      "segWit":"NONE",
      "source":"MNEMONIC"
      ]
    self.apiResult = prettyPrintJSON(param);
    self.apiResult = try! callTokenCoreXApi(param, import_wallet_from_mnemonic)
  }
  
  func exportMenmonic() {
    createWallet()
    let ret = try! JSONSerialization.jsonObject(with: self.apiResult.data(using: .utf8)!, options: .allowFragments) as! [String: Any]
    
    let param: [String: Any] = [
      "id": ret["id"] as! String,
      "password": "Insecure Password"
    ]
    self.apiResult = prettyPrintJSON(param);
    self.apiResult = try! callTokenCoreXApi(param, export_mnemonic)
    
  }
  
  func signTransaction() {
    importWallet()
    let ret = try! JSONSerialization.jsonObject(with: self.apiResult.data(using: .utf8)!, options: .allowFragments) as! [String: Any]
    
    let utxo: [String: Any] = [
      "txHash": "09c3a49c1d01f6341c43ea43dd0de571664a45b4e7d9211945cb3046006a98e2",
      "vout": 0,
      "amount": "100000",
      "address": "bitcoincash:qzld7dav7d2sfjdl6x9snkvf6raj8lfxjcj5fa8y2r",
      "scriptPubKey": "76a91488d9931ea73d60eaf7e5671efc0552b912911f2a88ac",
      "derivedPath": "0/0"
    ]
    let param: [String: Any] = [
      "id":ret["id"] as! String,
      "password": "Insecure Password",
      "to": "bitcoincash:qq40fskqshxem2gvz0xkf34ww3h6zwv4dcr7pm0z6s",
      "amount": "93454",
      "fee": "6000",
      "internalUsed": 0,
      "chainType": "BITCOINCASH",
      "chainId": "145",
      "segWit":"NONE",
      "outputs": [
          utxo
      ]
    ]
    self.apiResult = prettyPrintJSON(param);
    self.apiResult = try! callTokenCoreXApi(param, sign_transaction)
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
  
  private func callTokenCoreXApi(_ param: JSONObject, _ block: (_ mapStr: UnsafePointer<Int8>?) -> UnsafePointer<Int8>?) throws -> String {
      
      let data = try! JSONSerialization.data(withJSONObject: param, options: [])
      let mapStr = String(data: data, encoding: .utf8)!
      
      clear_err()
      let cPtr = block(mapStr)
      defer {
        free_const_string(cPtr)
      }
      let cErrPtr = get_last_err_message()!
      defer {
        free_const_string(cErrPtr)
      }
      
      let errStr = String(cString: cErrPtr)
      if errStr.count > 0 {
        throw errStr
      } else if let cPtr = cPtr {
        let ret = String(cString: cPtr)
        if ret ==  "{}" {
  
          return ret
        }
        
        let retObj = try? JSONSerialization.jsonObject(with: ret.data(using: .utf8)!, options: []) as? [AnyHashable: Any]
        if let retObj = retObj {
          return prettyPrintJSON(retObj)
        } else {
          return ret
        }
        
      } else {
        return ""
      }
    }
  
}

struct Operation: Identifiable {
  let id: String
  let desc: String
}


extension String: Error {}

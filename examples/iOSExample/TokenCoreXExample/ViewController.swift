//
//  ViewController.swift
//  TokenCoreXExample
//
//  Created by xyz on 2019/5/22.
//  Copyright © 2019 consenlabs. All rights reserved.
//

import UIKit

class ViewController: UIViewController {

  let MNEMONIC = "inject kidney empty canal shadow pact comfort wife crush horse wife sketch"
  let PASSWORD = "imToken1"
  let WIF = "L1uyy5qTuGrVXrmrsvHWHgVzW9kKdrp27wBC7Vs6nZDTF2BRUVwy"
  
  @IBOutlet weak var tvResult: UITextView!
  
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
  
  override func viewDidLoad() {
    super.viewDidLoad()
    // Do any additional setup after loading the view, typically from a nib.
    scanWallets()
  }
  
  func readFileByRust(filePath path: String) -> String {
    let cPtr = (try! read_file(path))!
    defer {
      try! free_const_string(cPtr)
    }
    return String(cString: cPtr)
  }
  
  func importBchWalletFromMnemonic(_ mnemonic: String, encryptedBy password: String) -> String {
    
    let fileDir = walletsDirectory.absoluteString.substring(from: String.Index(encodedOffset: "file://".count))
    let map: [String: Any] = [
      "password": password,
      "mnemonic": mnemonic,
      "path": "m/44'/145'/0'",
      "overwrite": true,
      "name": "bch-ios",
      "passwordHint": "",
      "chainType": "BCH",
      "network": "MAINNET",
      "source": "MNEMONIC",
      "fileDir": fileDir
    ];
    let data = try! JSONSerialization.data(withJSONObject: map, options: [])
    let mapStr = String(data: data, encoding: .utf8)!
    let cPtr = (try! import_wallet_from_mnemonic(mapStr))!
    defer {
      try! free_const_string(cPtr)
    }
    return String(cString: cPtr)
  }
  
  // 7848c875-3650-47ef-8cf8-9cfb69ce280b
  func signTransaction() -> String {
    
    let fileDir = walletsDirectory.absoluteString.substring(from: String.Index(encodedOffset: "file://".count))
    let unspents:[[String: Any]] = [
      [
        "txHash": "115e8f72f39fad874cfab0deed11a80f24f967a84079fb56ddf53ea02e308986",
        "vout": 0,
        "amount": 50000,
        "address": "17XBj6iFEsf8kzDMGQk5ghZipxX49VXuaV",
        "scriptPubKey": "76a91447862fe165e6121af80d5dde1ecb478ed170565b88ac",
        "derivedPath": "0/0",
        "sequence": 0
      ]
    ];
    let map: [String: Any] = [
      "id": "c8c5fa34-eb36-469a-bbcf-cbd7f6cc63f5",
      "password": "imToken1",
      "to": "1Gokm82v6DmtwKEB8AiVhm82hyFSsEvBDK",
      "amount": "15000",
      "memo": "",
      "fee": "35000",
      "internalUsed": 0,
      "chainId": "0",
      "chainType": "BCH",
      "outputs": unspents
      
    ];
    let data = try! JSONSerialization.data(withJSONObject: map, options: [])
    let mapStr = String(data: data, encoding: .utf8)!
    
    clear_err()
    let cPtr = sign_transaction(mapStr)!
    
    defer {
      free_const_string(cPtr)
    }
    let cErrPtr = get_last_err_message()!
    defer {
      free_const_string(cErrPtr)
    }
    
    let errStr = String(cString: cErrPtr)
    if errStr.count > 0 {
      return errStr
    } else {
      return String(cString: cPtr)
    }
    
    
  }
  
  func scanWallets() {
    let fileDir = walletsDirectory.absoluteString.substring(from: String.Index(encodedOffset: "file://".count))
    let map: [String: Any] = [
      "fileDir": fileDir,
    ];
    let data = try! JSONSerialization.data(withJSONObject: map, options: [])
    let mapStr = String(data: data, encoding: .utf8)!
    try! scan_wallets(mapStr)
  }
  
//  func importBchWalletFromPrivateKey(_ privateKey: String, encryptedBy password: String) -> String {
//
//    let fileDir = walletsDirectory.absoluteString.substring(from: String.Index(encodedOffset: "file://".count))
//    let map: [String: Any] = [
//      "password": password,
//      "privateKey": privateKey,
//      "overwrite": true,
//      "name": "bch-ios",
//      "passwordHint": "",
//      "chainType": "BCH",
//      "network": "MAINNET",
//      "fileDir": fileDir
//    ];
//    let data = try! JSONSerialization.data(withJSONObject: map, options: [])
//    let mapStr = String(data: data, encoding: .utf8)!
//    let cPtr = (try! import_wallet_from_private_key(mapStr))!
//    defer {
//      try! free_const_string(cPtr)
//    }
//    return String(cString: cPtr)
//  }
  
  @IBAction func onMnemonicImportClick(_ sender: Any) {
    self.tvResult.text = importBchWalletFromMnemonic(MNEMONIC, encryptedBy: PASSWORD)
    print(self.tvResult.text)
  }
  
  @IBAction func onPrivateKeyImportClick(_ sender: Any) {
//    self.tvResult.text = importBchWalletFromPrivateKey(WIF, encryptedBy: PASSWORD)
    
    print(self.tvResult.text)
  }
  
  
  @IBAction func onSignTransactionClick(_ sender: Any) {
    self.tvResult.text = signTransaction()
    print (self.tvResult.text)
  }
  
}


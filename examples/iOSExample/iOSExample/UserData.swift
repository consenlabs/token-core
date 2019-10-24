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
  @Published var operations = [Operation(id: "CREATE_WALLET", desc: "Create Wallet"), Operation(id: "IMPORT_WALLET", desc: "Import Wallet")]
  
  @Published var operationParams: String = ""
  @Published var operationResult: String = ""
  
  static let shared = UserData()
  
  func performOperation(_ op: Operation) {
    switch op.id {
    case "CREATE_WALLET":
        createWallet()
      return
    default:
      undefinedOp()
    }
  }
  
  func createWallet() {
    
    operationParams = """
    {
        "name": "createWalletTest",
        "password": "Insecure Password",
        "passwordHint": "Insecure Password",
        "source": "MNEMONIC"
    }
    """
//    let param =
    
  }
  
  func importWallet() {
    
  }
  
  func undefinedOp() {
    operationParams = "Undefined Operation"
    operationResult = "";
  }
  
  
  func doWorkBackground(_ workTip: String, hardWork: @escaping () -> Void) {
//    let hud = MBProgressHUD.showAdded(to: self.view, animated: true)
//    hud.label.text = workTip
    
    DispatchQueue.global().async {
      hardWork()
      
      DispatchQueue.main.async {
//        MBProgressHUD.hide(for: self.view, animated: true)
//        self.presentResult(self.requestResult)
//        self.operationResult =
      }
    }
  }
  
}

struct Operation: Identifiable {
  let id: String
  let desc: String
}

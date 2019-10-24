//
//  FuncResultView.swift
//  iOSExample
//
//  Created by xyz on 2019/10/23.
//  Copyright © 2019 consenlabs. All rights reserved.
//

import SwiftUI

struct FuncResultView: View {
  @EnvironmentObject var userData: UserData
  
  let operation: Operation
    var body: some View {
      NavigationView {
        Text(userData.operationParams)
          .navigationBarTitle(Text(operation.desc))
      }
  }
}

struct FuncResultView_Previews: PreviewProvider {
    static var previews: some View {
        FuncResultView(operation: Operation(id: "CREATE_WALLET", desc: "Create Wallet"))
    }
}

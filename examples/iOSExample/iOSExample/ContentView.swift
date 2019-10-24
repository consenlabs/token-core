//
//  ContentView.swift
//  iOSExample
//
//  Created by xyz on 2019/10/23.
//  Copyright © 2019 consenlabs. All rights reserved.
//

import SwiftUI

struct ContentView: View {
  @EnvironmentObject var userData: UserData
  
    var body: some View {
      NavigationView {
        List(userData.operations) { operation in
          NavigationLink(destination: FuncResultView(operation: operation)) {
            HStack(alignment: .center) {
                Text(operation.desc)
                  .frame(minWidth: 0, maxWidth: .infinity, alignment: .center)
            }
          }
      }
      .navigationBarTitle(Text("TokenCoreX Example"))
      }
  }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView().environmentObject(UserData())
    }
}

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
      LoadingView(isShowing: $userData.isLoading, label: $userData.loadingLabel) {
          VStack {
            Text("Operation Param:")
            .bold().italic()
            .frame(minWidth: 0, maxWidth: .infinity, alignment: .topLeading)
            Text(self.userData.operationParams)
            .frame(minWidth: 0, maxWidth: .infinity, alignment: .topLeading)
            Divider()
            Text("Operation Result:")
              .bold().italic()
            .frame(minWidth: 0, maxWidth: .infinity, alignment: .topLeading)
            Text(self.userData.operationResult)
            .frame(minWidth: 0, maxWidth: .infinity, alignment: .topLeading)
              Spacer()
          }
      }.onAppear{
        self.userData.performOperation(self.operation)
      }
  }
}


struct ActivityIndicator: UIViewRepresentable {

    @Binding var isAnimating: Bool
    let style: UIActivityIndicatorView.Style

    func makeUIView(context: UIViewRepresentableContext<ActivityIndicator>) -> UIActivityIndicatorView {
        return UIActivityIndicatorView(style: style)
    }

    func updateUIView(_ uiView: UIActivityIndicatorView, context: UIViewRepresentableContext<ActivityIndicator>) {
        isAnimating ? uiView.startAnimating() : uiView.stopAnimating()
    }
}

struct LoadingView<Content>: View where Content: View {

    @Binding var isShowing: Bool
  @Binding var label: String
    var content: () -> Content

    var body: some View {
        GeometryReader { geometry in
            ZStack(alignment: .center) {

                self.content()
                    .disabled(self.isShowing)
                    .blur(radius: self.isShowing ? 3 : 0)

                VStack {
                  Text(self.label)
                    ActivityIndicator(isAnimating: .constant(true), style: .large)
                }
                .frame(width: geometry.size.width / 2,
                       height: geometry.size.height / 5)
                .background(Color.secondary.colorInvert())
                .foregroundColor(Color.primary)
                .cornerRadius(20)
                .opacity(self.isShowing ? 1 : 0)

            }
        }
    }

}


struct FuncResultView_Previews: PreviewProvider {
    static var previews: some View {
        FuncResultView(operation: Operation(id: "CREATE_WALLET", desc: "Create Wallet"))
      .environmentObject(UserData.shared)
    }
}


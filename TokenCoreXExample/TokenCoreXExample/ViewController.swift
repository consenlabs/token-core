//
//  ViewController.swift
//  TokenCoreXExample
//
//  Created by xyz on 2019/5/22.
//  Copyright © 2019 consenlabs. All rights reserved.
//

import UIKit

class ViewController: UIViewController {

  override func viewDidLoad() {
    super.viewDidLoad()
    // Do any additional setup after loading the view, typically from a nib.
    
    if let dir = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first {
      let fileURL = dir.appendingPathComponent("rust_file.txt")
      try! "This text is write by swift".write(to: fileURL, atomically: true, encoding: .utf8)
      let fullFilePath = fileURL.absoluteString.substring(from: String.Index(encodedOffset: "file://".count))
      let cPtr = (try! readFile(fullFilePath))!
      print(String(cString: cPtr))
    }
  }
  
//  func stringFromC(ptr: UnsafePointer<Int8>) -> String {
//    let data = Data(bytes: ptr, count: Int(rawDataSize))
//    let str = String(data: data, encoding: String.Encoding.utf8)
//    return str
//  }


}


//
//  Utils.swift
//  iOSExample
//
//  Created by xyz on 2019/10/24.
//  Copyright © 2019 consenlabs. All rights reserved.
//

import Foundation

typealias JSONObject = [AnyHashable: Any]

func prettyPrintJSON(_ obj: JSONObject) -> String  {
    // fail fast in demo
    let encoded = try! JSONSerialization.data(withJSONObject: obj, options: .prettyPrinted)
    return String(data: encoded, encoding: .utf8)!
}

func parseJSONObject(_ str: String) -> JSONObject {
  let data = str.data(using: .utf8)!
  try! JSONSerialization.jsonObject(with: data, options: .allowFragments);
}


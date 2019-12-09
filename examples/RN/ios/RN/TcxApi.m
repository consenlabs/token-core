//
//  WalletAPI.m
//  RN
//
//  Created by xyz on 2019/11/26.
//  Copyright © 2019 Facebook. All rights reserved.
//

#import "TcxApi.h"
#import "NSData+NSData_Conversion.h"



@implementation TcxApi

RCT_EXPORT_MODULE();

RCT_EXPORT_METHOD(callTcxApi:(NSString *)hex resolver:(RCTPromiseResolveBlock)resolve rejecter:(RCTPromiseRejectBlock)reject) {

  NSData *data = [NSData dataFromHexString:hex];
  if (!data.length) {
      NSLog(@"Got an error");
  } else {
    clear_err();
    Buffer param;
    param.data = (uint8_t * )data.bytes;
    param.len = data.length;
    Buffer result = call_tcx_api(param);
    
    Buffer error = get_last_err();
    if (error.len > 0) {
      NSData *errData = [NSData dataWithBytes:error.data length:error.len];;
      free_buf(error);
      reject(@"", errData.hexadecimalString, nil);
    } else {
      NSData *resultData = [NSData dataWithBytes:result.data length:result.len];
      resolve(resultData.hexadecimalString);
    }
  }
  
}


@end

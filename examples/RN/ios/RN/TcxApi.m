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
  clear_err();

  const char *result = call_tcx_api([hex UTF8String]);
  const char *error = get_last_err_message();
  
  NSString *errorStr = [[NSString alloc] initWithUTF8String:error];
  NSString *resultData = [[NSString alloc] initWithUTF8String:result];
  
  free_const_string(error);
  free_const_string(result);
  
  if (errorStr.length > 0) {
    reject(@"", errorStr, nil);
    return;
  }
  resolve(resultData);
  
}


@end

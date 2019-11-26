//
//  WalletAPI.m
//  RN
//
//  Created by xyz on 2019/11/26.
//  Copyright © 2019 Facebook. All rights reserved.
//

#import "WalletAPI.h"




@implementation WalletAPI

+ (NSDictionary *)dictionaryWithJsonString:(NSString *)jsonString {
    if (jsonString == nil) {
        return nil;
    }
    NSData *jsonData = [jsonString dataUsingEncoding:NSUTF8StringEncoding];
    NSError *err;
    NSDictionary *dic = [NSJSONSerialization JSONObjectWithData:jsonData
                                                        options:NSJSONReadingMutableContainers
                                                          error:&err];
    if(err) {
        NSLog(@"json解析失败：%@",err);
        return nil;
    }
    return dic;
}


+ (void)callTokenCoreXApi:(NSDictionary *)map resolver:(RCTPromiseResolveBlock)resolve rejecter:(RCTPromiseRejectBlock)reject block:(TokenCoreAPI)block  {
  NSError *error;
  NSData *jsonData = [NSJSONSerialization dataWithJSONObject:map
                                                    options:NSJSONWritingPrettyPrinted // Pass 0 if you don't care about the readability of the generated string
                                                       error:&error];

  if (!jsonData) {
      NSLog(@"Got an error: %@", error);
  } else {
      NSString *jsonString = [[NSString alloc] initWithData:jsonData encoding:NSUTF8StringEncoding];
    clear_err();
    const char *param = [jsonString cStringUsingEncoding:NSASCIIStringEncoding];
    const char *cRet = block(param);
//    const char *param = [ret cStringUsingEncoding:NSASCIIStringEncoding];
    const char *cLastError = get_last_err_message();
    
    NSString *error = [NSString stringWithUTF8String:cLastError];
    if ([error length] > 0) {
      reject(@"", error, Nil);
    } else {
      NSString *ret = [NSString stringWithUTF8String:cRet];
      if ([ret isEqualToString:@"{}"]) {
        resolve(Nil);
        return;
      }
      
      NSDictionary *retObj = [WalletAPI dictionaryWithJsonString:ret];
      resolve(retObj);
    }
  }
  
}

+ (void) importWalletFromMnemonic:(NSDictionary *)map resolver:(RCTPromiseResolveBlock)resolve rejecter:(RCTPromiseRejectBlock)reject {
  [WalletAPI callTokenCoreXApi:map resolver:resolve rejecter:reject block:^(const char *param) {
    return import_wallet_from_mnemonic(param);
  }];
}
@end

//
//  WalletAPI.h
//  RN
//
//  Created by xyz on 2019/11/26.
//  Copyright © 2019 Facebook. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <React/RCTBridgeModule.h>
#import "tcx.h"

typedef const char *(^TokenCoreAPI)(const char *);

NS_ASSUME_NONNULL_BEGIN

@interface WalletAPI : NSObject <RCTBridgeModule>

+(void)callTokenCoreXApi:(NSDictionary *)map resolver:(RCTPromiseResolveBlock)resolve rejecter:(RCTPromiseRejectBlock)reject block:(TokenCoreAPI)block;
-(void) importWalletFromMnemonic:(NSDictionary *)map resolver:(RCTPromiseResolveBlock)resolve rejecter:(RCTPromiseRejectBlock)reject;

@end

NS_ASSUME_NONNULL_END

//
//  NSData+NSData_Conversion.h
//  RN
//
//  Created by xyz on 2019/12/9.
//  Copyright © 2019 Facebook. All rights reserved.
//

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

@interface NSData (NSData_Conversion)

#pragma mark - String Conversion
- (NSString *)hexadecimalString;
+ (NSData *)dataFromHexString:(NSString *)string;

@end

NS_ASSUME_NONNULL_END

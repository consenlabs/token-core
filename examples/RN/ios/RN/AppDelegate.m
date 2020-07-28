/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#import "AppDelegate.h"

#import <React/RCTBridge.h>
#import <React/RCTBundleURLProvider.h>
#import <React/RCTRootView.h>
#import "tcx.h"
#import "TcxApi.h"
#import "Api.pbobjc.h"

@implementation AppDelegate

+ (NSString *) walletsDirectory {
  NSArray *paths = NSSearchPathForDirectoriesInDomains(NSDocumentDirectory, NSUserDomainMask, YES);

  NSString *docDir = [paths objectAtIndex:0];
  NSArray *components = [NSArray arrayWithObjects:docDir, @"wallets", nil];
  NSString *path = [NSString pathWithComponents:components];
  NSFileManager *fileManager = [NSFileManager defaultManager];
  if(![fileManager fileExistsAtPath:path]) {
    [fileManager createDirectoryAtPath:path withIntermediateDirectories:YES attributes:Nil error:Nil];
  }
  return path;
}

- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions
{
  RCTBridge *bridge = [[RCTBridge alloc] initWithDelegate:self launchOptions:launchOptions];
  RCTRootView *rootView = [[RCTRootView alloc] initWithBridge:bridge
                                                   moduleName:@"RN"
                                            initialProperties:nil];

//  var walletsDirectory: URL {
//    let walletsPath = "\(NSHomeDirectory())/Documents/wallets"
//    var walletsDirectory = URL(fileURLWithPath: walletsPath)
//
//    do {
//      if !FileManager.default.fileExists(atPath: walletsPath) {
//        try FileManager.default.createDirectory(atPath: walletsDirectory.path, withIntermediateDirectories: true, attributes: nil)
//        var resourceValues = URLResourceValues()
//        resourceValues.isExcludedFromBackup = true
//        try walletsDirectory.setResourceValues(resourceValues)
//      }
//    } catch let err {
//      debugPrint(err)
//    }
//
//    return walletsDirectory
//  }
  
  NSString *walletsDirectory = [AppDelegate walletsDirectory];
  NSDictionary *param = @{
                         @"fileDir": walletsDirectory,
                         @"xpubCommonKey128": @"B888D25EC8C12BD5043777B1AC49F872",
                         @"xpubCommonIv": @"9C0C30889CBCC5E01AB5B2BB88715799"
  };
  NSError *error;
  NSData *jsonData = [NSJSONSerialization dataWithJSONObject:param
                                                    options:NSJSONWritingPrettyPrinted // Pass 0 if you don't care about the readability of the generated string
                                                       error:&error];

  if (!jsonData) {
      NSLog(@"Got an error: %@", error);
  } else {
      NSString *jsonString = [[NSString alloc] initWithData:jsonData encoding:NSUTF8StringEncoding];
    clear_err();
    const char *param = [jsonString cStringUsingEncoding:NSASCIIStringEncoding];
    InitTokenCoreXParam  *initParam = [[InitTokenCoreXParam alloc] init];
    initParam.fileDir = walletsDirectory;
    initParam.xpubCommonKey = @"B888D25EC8C12BD5043777B1AC49F872";
    initParam.xpubCommonIv = @"9C0C30889CBCC5E01AB5B2BB88715799";
    initParam.isDebug = YES;
//    init_token_core_x(param);
  }
    
  rootView.backgroundColor = [[UIColor alloc] initWithRed:1.0f green:1.0f blue:1.0f alpha:1];

  self.window = [[UIWindow alloc] initWithFrame:[UIScreen mainScreen].bounds];
  UIViewController *rootViewController = [UIViewController new];
  rootViewController.view = rootView;
  self.window.rootViewController = rootViewController;
  [self.window makeKeyAndVisible];
  return YES;
}

- (NSURL *)sourceURLForBridge:(RCTBridge *)bridge
{
#if DEBUG
  return [[RCTBundleURLProvider sharedSettings] jsBundleURLForBundleRoot:@"index" fallbackResource:nil];
#else
  return [[NSBundle mainBundle] URLForResource:@"main" withExtension:@"jsbundle"];
#endif
}

@end

#
# Be sure to run `pod lib lint TokenCoreX.podspec' to ensure this is a
# valid spec before submitting.
#
# Any lines starting with a # are optional, but their use is encouraged
# To learn more about a Podspec see https://guides.cocoapods.org/syntax/podspec.html
#

Pod::Spec.new do |s|
  s.name             = 'TokenCoreX'
  s.version          = '2.5.1'
  s.summary          = 'A cross-platform library that implements crypto wallet functions for blockchains'

# This description is used to generate tags and improve search results.
#   * Think: What does it do? Why did you write it? What is the focus?
#   * Try to keep it short, snappy and to the point.
#   * Write the description between the DESC delimiters below.
#   * Finally, don't worry about the indent, CocoaPods strips it!

  s.description      = <<-DESC
A cross-platform library that implements crypto wallet functions for blockchains, exporting c interfaces in Protobuf protocol. This library totally wrote in Rust, and now provides friendly interfaces for the mobile platform including ReactNative, iOS, and Android.
                       DESC

  s.homepage         = 'https://github.com/consenlabs/token-core'
  # s.screenshots     = 'www.example.com/screenshots_1', 'www.example.com/screenshots_2'
  s.license          = { :type => 'Apache Licence v2.0', :file => 'LICENSE' }
  s.author           = 'Neal Xu', 'sun', 'Kai'
  # s.source           = { :git => 'https://github.com/consenlabs/token-core.git', :tag => s.version.to_s }
  s.source           = { :http => "https://github.com/consenlabs/token-core/releases/download/v#{s.version}/ios-v#{s.version}.zip" }
  s.social_media_url = 'https://twitter.com/imTokenOfficial'

  s.ios.deployment_target = '8.0'
  s.static_framework = true
  s.vendored_frameworks = ['TokenCoreX.framework']
  # s.vendored_libraries = ['libtcx.a']
  # s.source_files = ['tcx.h']
  s.ios.pod_target_xcconfig = {'ENABLE_BITCODE' => 'YES' }
  
  # s.resource_bundles = {
  #   'TokenCoreX' => ['TokenCoreX/Assets/*.png']
  # }

  # s.public_header_files = 'Pod/Classes/**/*.h'
  # s.frameworks = 'UIKit', 'MapKit'
  # s.dependency 'AFNetworking', '~> 2.3'
end

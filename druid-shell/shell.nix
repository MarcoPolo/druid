let
  pkgs = import <nixpkgs> {};
  androidComposition = pkgs.androidenv.composeAndroidPackages {
    includeNDK = true;
    ndkVersion = "21.0.6113669";
    platformVersions = [ "29" ];
    abiVersions = [ "x86" "x86_64"];
  };
in
pkgs.mkShell rec {
  ANDROID_SDK = androidComposition.androidsdk;
  buildInputs = [
    pkgs.hello
    ANDROID_SDK
  ];
  # TODO what is linux's path here?
  ANDROID_CLANG = ANDROID_SDK + "/libexec/android-sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android29-clang";
  CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER = ANDROID_CLANG;
}

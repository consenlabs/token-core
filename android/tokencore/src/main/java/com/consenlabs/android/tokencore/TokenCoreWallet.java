package com.consenlabs.android.tokencore;

public class TokenCoreWallet {

  static {
    System.loadLibrary("TrezorCrypto");
    System.loadLibrary("rust");
    initLog();
  }
  public static native void initLog();

  public native void generateMnemonic(int strength);

  public void generateMnemonicWrapper(int strength) {
    generateMnemonic(128);
  }

}

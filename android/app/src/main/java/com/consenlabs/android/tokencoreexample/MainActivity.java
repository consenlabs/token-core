package com.consenlabs.android.tokencoreexample;

import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.widget.TextView;

import com.consenlabs.android.tokencore.TokenCoreWallet;

public class MainActivity extends AppCompatActivity {


  static {
    System.loadLibrary("TrezorCrypto");
    System.loadLibrary("rust");
    initLog();
  }
  public static native void initLog();

  public static native String generateMnemonic(int strength);

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    setContentView(R.layout.activity_main);
//    TokenCoreWallet.initLog();
//    final TokenCoreWallet tokenCoreWallet =  new TokenCoreWallet();
    final TextView textView = findViewById(R.id.tv_mnemonic);

    findViewById(R.id.btn_generateMnemonic).setOnClickListener(new View.OnClickListener() {
      @Override
      public void onClick(View v) {
//        Log.d("MainActivity", tokenCoreWallet.generateMnemonicWrapper(128));
        textView.setText(generateMnemonic(128));
//        tokenCoreWallet.generateMnemonicWrapper(128);
      }
    });
  }

}

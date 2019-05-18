package com.consenlabs.android.tokencoreexample;

import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.widget.TextView;

import com.consenlabs.android.tokencore.TokenCoreWallet;
import com.sun.jna.Library;
import com.sun.jna.Native;

import java.io.BufferedInputStream;
import java.io.BufferedOutputStream;
import java.io.BufferedWriter;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.FileWriter;
import java.io.IOException;

public class MainActivity extends AppCompatActivity {


  static {
    System.loadLibrary("TrezorCrypto");
    System.loadLibrary("rust");
    initLog();
  }
  public static native void initLog();

  public static native String generateMnemonic(int strength);

  public static native String readFile(String filePath);

  @Override
  protected void onCreate(final Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    setContentView(R.layout.activity_main);
//    TokenCoreWallet.initLog();
//    final TokenCoreWallet tokenCoreWallet =  new TokenCoreWallet();
    final TextView textView = findViewById(R.id.tv_mnemonic);

    findViewById(R.id.btn_generateMnemonic).setOnClickListener(new View.OnClickListener() {
      @Override
      public void onClick(View v) {
        textView.setText(TokenCore.INSTANCE.generateMnemonic(128));
      }
    });

    findViewById(R.id.btn_readFile).setOnClickListener(new View.OnClickListener() {
      @Override
      public void onClick(View v) {
        try {
          File file = new File(MainActivity.this.getFilesDir(), "rust.txt");
          BufferedWriter writer = new BufferedWriter(new FileWriter(file));
          writer.write("This text is write by Java");
          writer.close();
          textView.setText(readFile(file.getAbsolutePath()));
        } catch (FileNotFoundException e) {
          e.printStackTrace();
        } catch (IOException e) {
          e.printStackTrace();
        }
      }
    });


  }

  interface TokenCore extends Library {
    TokenCore INSTANCE = (TokenCore) Native.loadLibrary("rust", TokenCore.class);
    public String generateMnemonic(int strength);
  }

}

package com.consenlabs.android.tokencoreexample;

import android.os.Bundle;
import android.support.v7.app.AppCompatActivity;
import android.text.TextUtils;
import android.util.Log;
import android.view.View;

import com.sun.jna.Library;
import com.sun.jna.Native;

import org.json.JSONException;
import org.json.JSONObject;

import java.io.File;

public class MainActivity extends AppCompatActivity {


  static {
//    System.loadLibrary("TrezorCrypto");
    System.loadLibrary("secp256k1");
    System.loadLibrary("tcx");
  }




  @Override
  protected void onCreate(final Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    setContentView(R.layout.activity_main);



    JSONObject jsonObject = new JSONObject();
    try {
      jsonObject.put("fileDir", getWalletDir());

      TokenCore.INSTANCE.scan_wallets(jsonObject.toString());
    } catch (JSONException e) {
      e.printStackTrace();
    }

    findViewById(R.id.btn_import_wallet).setOnClickListener(new View.OnClickListener() {
      @Override
      public void onClick(View v) {
//        let map: [String: Any] = [
//        "password": password,
//            "mnemonic": mnemonic,
//            "path": "m/44'/145'/0'",
//            "overwrite": true,
//            "name": "bch-ios",
//            "passwordHint": "",
//            "chainType": "BITCOINCASH",
//            "network": "MAINNET",
//            "source": "MNEMONIC",
//            "fileDir": fileDir
//    ];

        JSONObject param = new JSONObject();
        try {
          param.put("password", "Insecure Password");
          param.put("mnemonic", "inject kidney empty canal shadow pact comfort wife crush horse wife sketch");
          param.put("path", "m/44'/145'/0'");
          param.put("overwrite", true);
          param.put("name", "bch-ios");
          param.put("passwordHint", "");
          param.put("chainType", "BITCOINCASH");
          param.put("network", "MAINNET");
          param.put("source", "MNEMONIC");
//          String response = TokenCore.INSTANCE.hello("World");
          TokenCore.INSTANCE.clear_err();
          String response = TokenCore.INSTANCE.import_wallet_from_mnemonic(param.toString());
          String err = TokenCore.INSTANCE.get_last_err_message();
          if (!TextUtils.isEmpty(err)) {
            Log.e("ResultFromRust", err);
          }
          Log.i("ResultFromRust", response);
        } catch (JSONException e) {
          e.printStackTrace();
        }
      }
    });

  }

  String getWalletDir() {
    File file = new File(MainActivity.this.getFilesDir(), "wallets");
    if (!file.exists()) {
      file.mkdirs();
    }
    return file.getAbsolutePath();
  }

  interface TokenCore extends Library {
    TokenCore INSTANCE = Native.load("tcx", TokenCore.class);

    void scan_wallets(String jsonStr);
    String import_wallet_from_mnemonic(String jsonStr);
    void clear_err();
    String get_last_err_message();
  }

}

package com.clementnerma.hify

import android.app.Activity
import android.app.AlertDialog
import android.content.Context
import android.graphics.Color
import android.os.Bundle
import android.text.InputType
import android.webkit.JavascriptInterface
import android.widget.EditText
import java.net.URLEncoder

class MainActivity: Activity() {
    private val storageWebViewUrlField = "WebViewLoadingURL"

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        setContentView(R.layout.activity_main)

        val client = CustomWebViewClient { errorMsg ->
            // Toast.makeText(this, errorMsg, Toast.LENGTH_LONG).show()
            this.fail(errorMsg)
        }

        val webView = findViewById<CustomWebView>(R.id.customWebView)
        webView.webViewClient = client
        webView.addJavascriptInterface(object {
            @JavascriptInterface
            fun updateAppUrl() {
                this@MainActivity.promptUrlUpdate()
            }
        }, "hifyAndroidInjectedObject")
        webView.setBackgroundColor(Color.BLACK)

        val url = this.getAppUrl()

        if (url == null) {
            this.fail("Please define an application URL first.")
            return
        }

        webView.loadUrl(url)

        val timeout = 5L

        Timeout(timeout * 1000) {
            if (!client.loaded) {
                this.fail("Page didn't load after $timeout seconds!")
            }
        }.start()
    }

    override fun onBackPressed() {
        val webView = findViewById<CustomWebView>(R.id.customWebView)
        webView.simulateBackButton()
    }

    private fun reload() {
        val mIntent = intent
        this.finish()
        this.startActivity(mIntent)
    }

    private fun fail(message: String) {
        val webView = findViewById<CustomWebView>(R.id.customWebView)
        webView.loadUrl("file:///android_asset/error.html?message=" + URLEncoder.encode(message, "utf-8"))
    }

    private fun getAppUrl(): String? {
        val prefs = this.getPreferences(Context.MODE_PRIVATE)
        return prefs.getString(this.storageWebViewUrlField, null)
    }

    private fun setAppUrl(url: String) {
        val prefs = this.getPreferences(Context.MODE_PRIVATE).edit()

        prefs.putString(this.storageWebViewUrlField, url)

        if (!prefs.commit()) {
            this.fail("Failed to save the new application URL")
        }
    }

    private fun prompt(title: String, message: String, default: String?, callback: (String?) -> Unit) {
        val builder = AlertDialog.Builder(this)
        builder.setTitle(title)

        val input = EditText(this)
        input.hint = message
        input.inputType = InputType.TYPE_CLASS_TEXT
        input.setText(default)

        builder.setView(input)

        builder.setPositiveButton("OK") { _, _ ->
            callback(input.text.toString())
        }

        builder.setNegativeButton("Cancel") { dialog, _ ->
            dialog.cancel()
            callback(null)
        }

        builder.show()
    }

    private fun promptUrlUpdate() {
        val url = this.getAppUrl()

        this.prompt("Please input the application URL:", "This URL will be used to load the application in the WebView", url) { newUrl ->
            if (newUrl != null) {
                this.setAppUrl(newUrl)
                this@MainActivity.reload()
            }
        }
    }
}
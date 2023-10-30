package com.clementnerma.hify

import android.annotation.SuppressLint
import android.content.Context
import android.util.AttributeSet
import android.util.Log
import android.view.KeyEvent
import android.webkit.WebSettings
import android.webkit.WebView

class CustomWebView(context: Context, attrs: AttributeSet) : WebView(context, attrs) {
    init {
        this.initView()
    }

    @SuppressLint("SetJavaScriptEnabled")
    private fun initView() {
        this.settings.javaScriptEnabled = true
        this.settings.mixedContentMode = WebSettings.MIXED_CONTENT_ALWAYS_ALLOW
        this.settings.allowContentAccess = true
        this.settings.domStorageEnabled = true
        this.settings.mediaPlaybackRequiresUserGesture = false
    }

    fun simulateBackButton() {
        this.dispatchKeyEvent(KeyEvent(KeyEvent.ACTION_DOWN, KeyEvent.KEYCODE_F4))
        this.dispatchKeyEvent(KeyEvent(KeyEvent.ACTION_UP, KeyEvent.KEYCODE_F4))
    }
}